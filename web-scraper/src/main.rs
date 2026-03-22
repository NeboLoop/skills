use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use chrono::Utc;
use clap::Parser;
use reqwest::Client;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use similar::{ChangeTag, TextDiff};
use tracing::{info, warn};

#[derive(Parser)]
#[command(name = "web-differ", about = "Scrape web pages, cache content, detect changes")]
struct Args {
    /// URLs to scrape (one or more)
    #[arg(long, num_args = 1..)]
    urls: Vec<String>,

    /// Read URLs from a file (one per line)
    #[arg(long)]
    urls_file: Option<String>,

    /// Cache directory for storing previous scrapes
    #[arg(long, default_value = ".web-differ-cache")]
    cache_dir: String,

    /// Output JSON to file (default: stdout)
    #[arg(long)]
    output: Option<String>,

    /// Only scrape, don't diff against cache
    #[arg(long, default_value = "false")]
    no_diff: bool,

    /// Include full page text in output (not just changes)
    #[arg(long, default_value = "false")]
    include_full_text: bool,

    /// Custom user agent
    #[arg(long, default_value = "NeboBot/1.0 (+https://neboloop.com/bot)")]
    user_agent: String,
}

// ── Output types ───────────────────────────────────────────────

#[derive(Serialize)]
struct DiffOutput {
    timestamp: String,
    pages: Vec<PageResult>,
    summary: DiffSummary,
}

#[derive(Serialize)]
struct PageResult {
    url: String,
    status: u16,
    title: Option<String>,
    content_hash: String,
    scraped_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    previous_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    previous_scraped_at: Option<String>,
    changed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    diff: Option<DiffData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    full_text: Option<String>,
    metadata: PageMetadata,
}

#[derive(Serialize)]
struct DiffData {
    added_lines: usize,
    removed_lines: usize,
    changed_sections: Vec<ChangedSection>,
}

#[derive(Serialize)]
struct ChangedSection {
    #[serde(rename = "type")]
    change_type: String, // "added", "removed", "modified"
    content: String,
    context: Option<String>,
}

#[derive(Serialize)]
struct PageMetadata {
    meta_description: Option<String>,
    h1: Vec<String>,
    word_count: usize,
    links_count: usize,
    images_count: usize,
}

#[derive(Serialize)]
struct DiffSummary {
    total_pages: usize,
    pages_changed: usize,
    pages_unchanged: usize,
    pages_new: usize,
    pages_failed: usize,
    total_additions: usize,
    total_removals: usize,
}

#[derive(Serialize, Deserialize)]
struct CachedPage {
    url: String,
    content_hash: String,
    text: String,
    scraped_at: String,
}

// ── Main ───────────────────────────────────────────────────────

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "web_differ=info".into()),
        )
        .with_target(false)
        .init();

    let args = Args::parse();

    // Collect URLs
    let mut urls = args.urls.clone();
    if let Some(file) = &args.urls_file {
        let content = std::fs::read_to_string(file)
            .with_context(|| format!("Failed to read URLs file: {file}"))?;
        for line in content.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() && !trimmed.starts_with('#') {
                urls.push(trimmed.to_string());
            }
        }
    }

    if urls.is_empty() {
        anyhow::bail!("No URLs provided. Use --urls or --urls-file");
    }

    // Ensure cache dir exists
    let cache_dir = PathBuf::from(&args.cache_dir);
    std::fs::create_dir_all(&cache_dir)?;

    let client = Client::builder()
        .user_agent(&args.user_agent)
        .timeout(std::time::Duration::from_secs(15))
        .build()?;

    let mut pages: Vec<PageResult> = Vec::new();
    let mut pages_failed = 0usize;

    for url in &urls {
        info!(url = %url, "scraping");

        match scrape_and_diff(&client, url, &cache_dir, args.no_diff, args.include_full_text).await
        {
            Ok(result) => {
                if result.changed {
                    info!(url = %url, "CHANGED");
                } else if result.previous_hash.is_none() {
                    info!(url = %url, "NEW (first scrape)");
                } else {
                    info!(url = %url, "unchanged");
                }
                pages.push(result);
            }
            Err(e) => {
                warn!(url = %url, error = %e, "failed");
                pages_failed += 1;
            }
        }
    }

    // Build summary
    let pages_changed = pages.iter().filter(|p| p.changed).count();
    let pages_new = pages.iter().filter(|p| p.previous_hash.is_none()).count();
    let pages_unchanged = pages.len() - pages_changed - pages_new;
    let total_additions: usize = pages
        .iter()
        .filter_map(|p| p.diff.as_ref())
        .map(|d| d.added_lines)
        .sum();
    let total_removals: usize = pages
        .iter()
        .filter_map(|p| p.diff.as_ref())
        .map(|d| d.removed_lines)
        .sum();

    let output = DiffOutput {
        timestamp: Utc::now().to_rfc3339(),
        pages,
        summary: DiffSummary {
            total_pages: urls.len(),
            pages_changed,
            pages_unchanged,
            pages_new,
            pages_failed,
            total_additions,
            total_removals,
        },
    };

    let json = serde_json::to_string_pretty(&output)?;
    if let Some(path) = &args.output {
        std::fs::write(path, &json)?;
        info!(path = %path, "output written");
    } else {
        println!("{json}");
    }

    Ok(())
}

async fn scrape_and_diff(
    client: &Client,
    url: &str,
    cache_dir: &Path,
    no_diff: bool,
    include_full_text: bool,
) -> Result<PageResult> {
    let resp = client.get(url).send().await?;
    let status = resp.status().as_u16();
    let body = resp.text().await?;
    let document = Html::parse_document(&body);

    let text = extract_text(&document);
    let title = extract_title(&document);
    let metadata = extract_metadata(&document);

    let content_hash = hash_text(&text);
    let now = Utc::now().to_rfc3339();
    let cache_key = url_to_cache_key(url);
    let cache_path = cache_dir.join(format!("{cache_key}.json"));

    // Load previous cache
    let previous = if !no_diff {
        load_cache(&cache_path)
    } else {
        None
    };

    // Diff against previous
    let (changed, diff, previous_hash, previous_scraped_at) = if let Some(ref prev) = previous {
        if prev.content_hash == content_hash {
            (false, None, Some(prev.content_hash.clone()), Some(prev.scraped_at.clone()))
        } else {
            let diff = compute_diff(&prev.text, &text);
            (true, Some(diff), Some(prev.content_hash.clone()), Some(prev.scraped_at.clone()))
        }
    } else {
        (false, None, None, None)
    };

    // Save to cache
    let cached = CachedPage {
        url: url.to_string(),
        content_hash: content_hash.clone(),
        text: text.clone(),
        scraped_at: now.clone(),
    };
    save_cache(&cache_path, &cached)?;

    Ok(PageResult {
        url: url.to_string(),
        status,
        title,
        content_hash,
        scraped_at: now,
        previous_hash,
        previous_scraped_at,
        changed,
        diff,
        full_text: if include_full_text { Some(text) } else { None },
        metadata,
    })
}

fn extract_text(doc: &Html) -> String {
    let body_sel = match Selector::parse("body") {
        Ok(s) => s,
        Err(_) => return String::new(),
    };

    doc.select(&body_sel)
        .next()
        .map(|body| {
            let mut lines = Vec::new();
            for node in body.text() {
                let trimmed = node.trim();
                if !trimmed.is_empty() {
                    lines.push(trimmed.to_string());
                }
            }
            lines.join("\n")
        })
        .unwrap_or_default()
}

fn extract_title(doc: &Html) -> Option<String> {
    let sel = Selector::parse("title").ok()?;
    doc.select(&sel)
        .next()
        .map(|el| el.text().collect::<String>().trim().to_string())
        .filter(|s| !s.is_empty())
}

fn extract_metadata(doc: &Html) -> PageMetadata {
    let meta_description = Selector::parse("meta[name=\"description\"]")
        .ok()
        .and_then(|sel| {
            doc.select(&sel)
                .next()
                .and_then(|el| el.value().attr("content").map(|s| s.to_string()))
        });

    let h1 = Selector::parse("h1")
        .ok()
        .map(|sel| {
            doc.select(&sel)
                .map(|el| el.text().collect::<String>().trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        })
        .unwrap_or_default();

    let word_count = extract_text(doc).split_whitespace().count();

    let links_count = Selector::parse("a[href]")
        .ok()
        .map(|sel| doc.select(&sel).count())
        .unwrap_or(0);

    let images_count = Selector::parse("img")
        .ok()
        .map(|sel| doc.select(&sel).count())
        .unwrap_or(0);

    PageMetadata {
        meta_description,
        h1,
        word_count,
        links_count,
        images_count,
    }
}

fn hash_text(text: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(text.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn url_to_cache_key(url: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(url.as_bytes());
    format!("{:x}", hasher.finalize())[..16].to_string()
}

fn load_cache(path: &Path) -> Option<CachedPage> {
    let data = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(&data).ok()
}

fn save_cache(path: &Path, page: &CachedPage) -> Result<()> {
    let json = serde_json::to_string_pretty(page)?;
    std::fs::write(path, json)?;
    Ok(())
}

fn compute_diff(old: &str, new: &str) -> DiffData {
    let diff = TextDiff::from_lines(old, new);
    let mut added = 0usize;
    let mut removed = 0usize;
    let mut sections: Vec<ChangedSection> = Vec::new();

    for change in diff.iter_all_changes() {
        match change.tag() {
            ChangeTag::Insert => {
                added += 1;
                let content = change.value().trim().to_string();
                if !content.is_empty() {
                    sections.push(ChangedSection {
                        change_type: "added".to_string(),
                        content,
                        context: None,
                    });
                }
            }
            ChangeTag::Delete => {
                removed += 1;
                let content = change.value().trim().to_string();
                if !content.is_empty() {
                    sections.push(ChangedSection {
                        change_type: "removed".to_string(),
                        content,
                        context: None,
                    });
                }
            }
            ChangeTag::Equal => {}
        }
    }

    // Collapse consecutive same-type changes
    let mut collapsed: Vec<ChangedSection> = Vec::new();
    for section in sections {
        if let Some(last) = collapsed.last_mut() {
            if last.change_type == section.change_type {
                last.content.push('\n');
                last.content.push_str(&section.content);
                continue;
            }
        }
        collapsed.push(section);
    }

    DiffData {
        added_lines: added,
        removed_lines: removed,
        changed_sections: collapsed,
    }
}
