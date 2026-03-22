use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;
use std::time::{Duration, Instant};

use chrono::Utc;
use clap::Parser;
use reqwest::Client;
use scraper::Html;
use serde::Serialize;
use tokio::sync::Semaphore;
use tokio::time::sleep;
use tracing::{debug, info, warn};
use url::Url;

mod extract;
mod robots;
mod sitemap;

use extract::extract_page_data;
use robots::RobotsData;
use sitemap::SitemapData;

#[derive(Parser)]
#[command(name = "seo-crawler", about = "Fast, polite website crawler for SEO analysis")]
struct Args {
    /// Starting URL to crawl
    #[arg(long)]
    url: String,

    /// Maximum pages to crawl
    #[arg(long, default_value = "50")]
    max_pages: usize,

    /// Maximum link depth from start URL
    #[arg(long, default_value = "3")]
    depth: usize,

    /// Milliseconds between requests to same host
    #[arg(long, default_value = "200")]
    delay: u64,

    /// Write JSON output to file (default: stdout)
    #[arg(long)]
    output: Option<String>,

    /// Only fetch and parse sitemaps, don't crawl
    #[arg(long, default_value = "false")]
    sitemap_only: bool,

    /// Include cleaned body text for content analysis
    #[arg(long, default_value = "false")]
    include_body: bool,

    /// Custom user agent string
    #[arg(long, default_value = "NeboBot/1.0 (+https://neboloop.com/bot)")]
    user_agent: String,
}

// ── Output types ───────────────────────────────────────────────

#[derive(Serialize)]
struct CrawlOutput {
    crawl: CrawlMeta,
    robots_txt: RobotsData,
    sitemaps: Vec<SitemapData>,
    pages: Vec<PageData>,
    summary: CrawlSummary,
}

#[derive(Serialize)]
struct CrawlMeta {
    start_url: String,
    timestamp: String,
    pages_crawled: usize,
    pages_skipped: usize,
    duration_ms: u64,
}

#[derive(Serialize, Clone)]
pub struct PageData {
    pub url: String,
    pub status: u16,
    pub redirect_url: Option<String>,
    pub load_time_ms: u64,
    pub title: Option<String>,
    pub meta_description: Option<String>,
    pub canonical: Option<String>,
    pub robots_meta: Option<String>,
    pub og_tags: HashMap<String, String>,
    pub headings: HeadingData,
    pub links: LinkData,
    pub images: Vec<ImageData>,
    pub schema: Vec<SchemaData>,
    pub hreflang: Vec<HreflangData>,
    pub word_count: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_text: Option<String>,
}

#[derive(Serialize, Clone, Default)]
pub struct HeadingData {
    pub h1: Vec<String>,
    pub h2: Vec<String>,
    pub h3: Vec<String>,
}

#[derive(Serialize, Clone, Default)]
pub struct LinkData {
    pub internal: Vec<LinkInfo>,
    pub external: Vec<LinkInfo>,
}

#[derive(Serialize, Clone)]
pub struct LinkInfo {
    pub href: String,
    pub text: String,
    pub nofollow: bool,
}

#[derive(Serialize, Clone)]
pub struct ImageData {
    pub src: String,
    pub alt: Option<String>,
    pub width: Option<String>,
    pub height: Option<String>,
    pub loading: Option<String>,
}

#[derive(Serialize, Clone)]
pub struct SchemaData {
    #[serde(rename = "type")]
    pub schema_type: String,
    pub raw: String,
}

#[derive(Serialize, Clone)]
pub struct HreflangData {
    pub lang: String,
    pub href: String,
}

#[derive(Serialize)]
struct CrawlSummary {
    total_pages: usize,
    total_images: usize,
    total_internal_links: usize,
    total_external_links: usize,
    pages_without_title: usize,
    pages_without_description: usize,
    pages_without_h1: usize,
    pages_with_multiple_h1: usize,
    pages_without_canonical: usize,
    pages_with_schema: usize,
    orphan_pages: Vec<String>,
    broken_links: Vec<BrokenLink>,
    duplicate_titles: Vec<DuplicateEntry>,
    duplicate_descriptions: Vec<DuplicateEntry>,
}

#[derive(Serialize)]
struct BrokenLink {
    from: String,
    to: String,
    status: u16,
}

#[derive(Serialize)]
struct DuplicateEntry {
    value: String,
    urls: Vec<String>,
}

// ── Main ───────────────────────────────────────────────────────

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "seo_crawler=info".into()),
        )
        .with_target(false)
        .init();

    let args = Args::parse();
    let start = Instant::now();

    let base_url = Url::parse(&args.url)?;
    let host = base_url.host_str().unwrap_or_default().to_string();

    let client = Client::builder()
        .user_agent(&args.user_agent)
        .timeout(Duration::from_secs(10))
        .redirect(reqwest::redirect::Policy::none())
        .build()?;

    // 1. Fetch robots.txt
    info!("fetching robots.txt");
    let robots = robots::fetch_robots(&client, &base_url).await;

    // 2. Fetch and parse sitemaps
    info!("fetching sitemaps");
    let sitemaps = sitemap::fetch_sitemaps(&client, &base_url, &robots).await;

    if args.sitemap_only {
        let output = CrawlOutput {
            crawl: CrawlMeta {
                start_url: args.url,
                timestamp: Utc::now().to_rfc3339(),
                pages_crawled: 0,
                pages_skipped: 0,
                duration_ms: start.elapsed().as_millis() as u64,
            },
            robots_txt: robots,
            sitemaps,
            pages: vec![],
            summary: empty_summary(),
        };
        write_output(&output, &args.output)?;
        return Ok(());
    }

    // 3. Crawl pages
    info!(max_pages = args.max_pages, depth = args.depth, "starting crawl");

    let semaphore = Arc::new(Semaphore::new(4)); // 4 concurrent requests
    let mut visited: HashSet<String> = HashSet::new();
    let mut queue: VecDeque<(String, usize)> = VecDeque::new();
    let mut pages: Vec<PageData> = Vec::new();
    let mut skipped = 0usize;
    let mut broken_links: Vec<BrokenLink> = Vec::new();
    let mut link_targets: HashSet<String> = HashSet::new();

    queue.push_back((normalize_url(&base_url.to_string()), 0));

    while let Some((url, depth)) = queue.pop_front() {
        if visited.len() >= args.max_pages {
            break;
        }
        if depth > args.depth {
            continue;
        }
        if visited.contains(&url) {
            continue;
        }

        // Check robots.txt
        if !robots.is_allowed(&url) {
            debug!(url = %url, "blocked by robots.txt");
            skipped += 1;
            visited.insert(url);
            continue;
        }

        // Only crawl same host
        if let Ok(parsed) = Url::parse(&url) {
            if parsed.host_str().unwrap_or_default() != host {
                continue;
            }
        } else {
            continue;
        }

        visited.insert(url.clone());

        // Rate limit
        sleep(Duration::from_millis(args.delay)).await;

        let permit = semaphore.clone().acquire_owned().await?;
        let client = client.clone();
        let url_clone = url.clone();
        let include_body = args.include_body;

        let result = tokio::spawn(async move {
            let _permit = permit;
            fetch_and_extract(&client, &url_clone, include_body).await
        })
        .await?;

        match result {
            Ok(page) => {
                // Collect internal links for further crawling
                for link in &page.links.internal {
                    let abs = resolve_url(&url, &link.href);
                    let norm = normalize_url(&abs);
                    link_targets.insert(norm.clone());
                    if !visited.contains(&norm) && !link.nofollow {
                        queue.push_back((norm, depth + 1));
                    }
                }

                // Track broken status
                if page.status >= 400 {
                    // This page itself is broken — find who linked to it
                    broken_links.push(BrokenLink {
                        from: "external/direct".to_string(),
                        to: url.clone(),
                        status: page.status,
                    });
                }

                // Handle redirects — follow them
                if let Some(redirect) = &page.redirect_url {
                    let norm = normalize_url(redirect.as_str());
                    link_targets.insert(norm.clone());
                    if !visited.contains(&norm) {
                        queue.push_back((norm, depth));
                    }
                }

                info!(url = %url, status = page.status, "crawled");
                pages.push(page);
            }
            Err(e) => {
                warn!(url = %url, error = %e, "failed to fetch");
                skipped += 1;
            }
        }
    }

    // 4. Build summary
    let summary = build_summary(&pages, &link_targets, &visited, broken_links);

    let output = CrawlOutput {
        crawl: CrawlMeta {
            start_url: args.url,
            timestamp: Utc::now().to_rfc3339(),
            pages_crawled: pages.len(),
            pages_skipped: skipped,
            duration_ms: start.elapsed().as_millis() as u64,
        },
        robots_txt: robots,
        sitemaps,
        pages,
        summary,
    };

    write_output(&output, &args.output)?;
    info!(
        pages = output.crawl.pages_crawled,
        duration_ms = output.crawl.duration_ms,
        "crawl complete"
    );

    Ok(())
}

async fn fetch_and_extract(client: &Client, url: &str, include_body: bool) -> anyhow::Result<PageData> {
    let start = Instant::now();
    let resp = client.get(url).send().await?;
    let status = resp.status().as_u16();

    // Handle redirects
    let redirect_url = if (300..400).contains(&status) {
        resp.headers()
            .get("location")
            .and_then(|v| v.to_str().ok())
            .map(|loc| resolve_url(url, loc))
    } else {
        None
    };

    let load_time = start.elapsed().as_millis() as u64;

    // For redirects, return minimal data
    if redirect_url.is_some() {
        return Ok(PageData {
            url: url.to_string(),
            status,
            redirect_url,
            load_time_ms: load_time,
            title: None,
            meta_description: None,
            canonical: None,
            robots_meta: None,
            og_tags: HashMap::new(),
            headings: HeadingData::default(),
            links: LinkData::default(),
            images: vec![],
            schema: vec![],
            hreflang: vec![],
            word_count: 0,
            body_text: None,
        });
    }

    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();

    if !content_type.contains("text/html") {
        return Ok(PageData {
            url: url.to_string(),
            status,
            redirect_url: None,
            load_time_ms: load_time,
            title: None,
            meta_description: None,
            canonical: None,
            robots_meta: None,
            og_tags: HashMap::new(),
            headings: HeadingData::default(),
            links: LinkData::default(),
            images: vec![],
            schema: vec![],
            hreflang: vec![],
            word_count: 0,
            body_text: None,
        });
    }

    let body = resp.text().await?;
    let document = Html::parse_document(&body);

    Ok(extract_page_data(url, status, load_time, &document, include_body))
}

fn normalize_url(url: &str) -> String {
    if let Ok(mut parsed) = Url::parse(url) {
        parsed.set_fragment(None);
        let mut s = parsed.to_string();
        // Remove trailing slash except for root
        if s.ends_with('/') && parsed.path() != "/" {
            s.pop();
        }
        s
    } else {
        url.to_string()
    }
}

fn resolve_url(base: &str, href: &str) -> String {
    if let Ok(base_url) = Url::parse(base) {
        if let Ok(resolved) = base_url.join(href) {
            return resolved.to_string();
        }
    }
    href.to_string()
}

fn write_output(output: &CrawlOutput, path: &Option<String>) -> anyhow::Result<()> {
    let json = serde_json::to_string_pretty(output)?;
    if let Some(path) = path {
        std::fs::write(path, &json)?;
        info!(path = %path, "output written");
    } else {
        println!("{json}");
    }
    Ok(())
}

fn empty_summary() -> CrawlSummary {
    CrawlSummary {
        total_pages: 0,
        total_images: 0,
        total_internal_links: 0,
        total_external_links: 0,
        pages_without_title: 0,
        pages_without_description: 0,
        pages_without_h1: 0,
        pages_with_multiple_h1: 0,
        pages_without_canonical: 0,
        pages_with_schema: 0,
        orphan_pages: vec![],
        broken_links: vec![],
        duplicate_titles: vec![],
        duplicate_descriptions: vec![],
    }
}

fn build_summary(
    pages: &[PageData],
    link_targets: &HashSet<String>,
    visited: &HashSet<String>,
    broken_links: Vec<BrokenLink>,
) -> CrawlSummary {
    let mut title_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut desc_map: HashMap<String, Vec<String>> = HashMap::new();

    let mut total_images = 0;
    let mut total_internal = 0;
    let mut total_external = 0;
    let mut no_title = 0;
    let mut no_desc = 0;
    let mut no_h1 = 0;
    let mut multi_h1 = 0;
    let mut no_canonical = 0;
    let mut has_schema = 0;

    for page in pages {
        total_images += page.images.len();
        total_internal += page.links.internal.len();
        total_external += page.links.external.len();

        if page.title.is_none() || page.title.as_deref() == Some("") {
            no_title += 1;
        } else if let Some(ref t) = page.title {
            title_map.entry(t.clone()).or_default().push(page.url.clone());
        }

        if page.meta_description.is_none() || page.meta_description.as_deref() == Some("") {
            no_desc += 1;
        } else if let Some(ref d) = page.meta_description {
            desc_map.entry(d.clone()).or_default().push(page.url.clone());
        }

        if page.headings.h1.is_empty() {
            no_h1 += 1;
        } else if page.headings.h1.len() > 1 {
            multi_h1 += 1;
        }

        if page.canonical.is_none() {
            no_canonical += 1;
        }

        if !page.schema.is_empty() {
            has_schema += 1;
        }
    }

    // Orphan pages = crawled pages that no other page links to (except the start URL)
    let orphan_pages: Vec<String> = visited
        .iter()
        .filter(|url| !link_targets.contains(*url))
        .skip(1) // skip start URL
        .cloned()
        .collect();

    let duplicate_titles: Vec<DuplicateEntry> = title_map
        .into_iter()
        .filter(|(_, urls)| urls.len() > 1)
        .map(|(value, urls)| DuplicateEntry { value, urls })
        .collect();

    let duplicate_descriptions: Vec<DuplicateEntry> = desc_map
        .into_iter()
        .filter(|(_, urls)| urls.len() > 1)
        .map(|(value, urls)| DuplicateEntry { value, urls })
        .collect();

    CrawlSummary {
        total_pages: pages.len(),
        total_images,
        total_internal_links: total_internal,
        total_external_links: total_external,
        pages_without_title: no_title,
        pages_without_description: no_desc,
        pages_without_h1: no_h1,
        pages_with_multiple_h1: multi_h1,
        pages_without_canonical: no_canonical,
        pages_with_schema: has_schema,
        orphan_pages,
        broken_links,
        duplicate_titles,
        duplicate_descriptions,
    }
}
