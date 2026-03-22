use reqwest::Client;
use serde::Serialize;
use tracing::{debug, warn};
use url::Url;

use crate::robots::RobotsData;

#[derive(Serialize)]
pub struct SitemapData {
    pub url: String,
    #[serde(rename = "type")]
    pub sitemap_type: String, // "index" or "urlset"
    pub url_count: usize,
    pub entries: Vec<SitemapEntry>,
    pub errors: Vec<String>,
}

#[derive(Serialize)]
pub struct SitemapEntry {
    pub loc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lastmod: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changefreq: Option<String>,
}

pub async fn fetch_sitemaps(
    client: &Client,
    base_url: &Url,
    robots: &RobotsData,
) -> Vec<SitemapData> {
    let mut sitemap_urls: Vec<String> = robots.sitemaps.clone();

    // If no sitemaps in robots.txt, try common locations
    if sitemap_urls.is_empty() {
        let default = format!(
            "{}://{}/sitemap.xml",
            base_url.scheme(),
            base_url.host_str().unwrap_or_default()
        );
        sitemap_urls.push(default);
    }

    let mut results = Vec::new();

    for url in sitemap_urls {
        match fetch_one_sitemap(client, &url).await {
            Ok(data) => {
                // If it's an index, fetch child sitemaps
                if data.sitemap_type == "index" {
                    let child_urls: Vec<String> =
                        data.entries.iter().map(|e| e.loc.clone()).collect();
                    results.push(data);

                    for child_url in &child_urls {
                        match fetch_one_sitemap(client, child_url).await {
                            Ok(child) => results.push(child),
                            Err(e) => {
                                warn!(url = %child_url, error = %e, "failed to fetch child sitemap");
                            }
                        }
                    }
                } else {
                    results.push(data);
                }
            }
            Err(e) => {
                debug!(url = %url, error = %e, "sitemap not found");
            }
        }
    }

    results
}

async fn fetch_one_sitemap(client: &Client, url: &str) -> anyhow::Result<SitemapData> {
    let resp = client.get(url).send().await?;

    if !resp.status().is_success() {
        anyhow::bail!("HTTP {}", resp.status());
    }

    let body = resp.text().await?;
    parse_sitemap(url, &body)
}

fn parse_sitemap(url: &str, xml: &str) -> anyhow::Result<SitemapData> {
    let mut entries = Vec::new();
    let mut errors = Vec::new();
    let mut is_index = false;

    // Determine type
    if xml.contains("<sitemapindex") {
        is_index = true;
    }

    let mut reader = quick_xml::Reader::from_str(xml);
    let mut buf = Vec::new();
    let mut current_loc = None;
    let mut current_lastmod = None;
    let mut current_priority = None;
    let mut current_changefreq = None;
    let mut in_url = false;
    let mut in_sitemap = false;
    let mut current_tag = String::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(quick_xml::events::Event::Start(e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                match name.as_str() {
                    "url" => in_url = true,
                    "sitemap" => in_sitemap = true,
                    "loc" | "lastmod" | "priority" | "changefreq" => {
                        current_tag = name;
                    }
                    _ => {}
                }
            }
            Ok(quick_xml::events::Event::Text(e)) => {
                let text = e.unescape().unwrap_or_default().trim().to_string();
                if text.is_empty() {
                    continue;
                }
                match current_tag.as_str() {
                    "loc" => current_loc = Some(text),
                    "lastmod" => current_lastmod = Some(text),
                    "priority" => current_priority = text.parse::<f64>().ok(),
                    "changefreq" => current_changefreq = Some(text),
                    _ => {}
                }
                current_tag.clear();
            }
            Ok(quick_xml::events::Event::End(e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                if (name == "url" && in_url) || (name == "sitemap" && in_sitemap) {
                    if let Some(loc) = current_loc.take() {
                        entries.push(SitemapEntry {
                            loc,
                            lastmod: current_lastmod.take(),
                            priority: current_priority.take(),
                            changefreq: current_changefreq.take(),
                        });
                    }
                    in_url = false;
                    in_sitemap = false;
                }
                current_tag.clear();
            }
            Ok(quick_xml::events::Event::Eof) => break,
            Err(e) => {
                errors.push(format!("XML parse error: {e}"));
                break;
            }
            _ => {}
        }
        buf.clear();
    }

    Ok(SitemapData {
        url: url.to_string(),
        sitemap_type: if is_index {
            "index".to_string()
        } else {
            "urlset".to_string()
        },
        url_count: entries.len(),
        entries,
        errors,
    })
}
