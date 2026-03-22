use std::collections::HashMap;

use scraper::{Html, Selector};

use crate::{HeadingData, HreflangData, ImageData, LinkData, LinkInfo, PageData, SchemaData};

pub fn extract_page_data(
    url: &str,
    status: u16,
    load_time_ms: u64,
    document: &Html,
    include_body: bool,
) -> PageData {
    let base_url = url::Url::parse(url).ok();
    let host = base_url
        .as_ref()
        .and_then(|u| u.host_str())
        .unwrap_or_default()
        .to_string();

    PageData {
        url: url.to_string(),
        status,
        redirect_url: None,
        load_time_ms,
        title: extract_title(document),
        meta_description: extract_meta(document, "description"),
        canonical: extract_link_rel(document, "canonical"),
        robots_meta: extract_meta(document, "robots"),
        og_tags: extract_og_tags(document),
        headings: extract_headings(document),
        links: extract_links(document, &host, base_url.as_ref()),
        images: extract_images(document),
        schema: extract_schema(document),
        hreflang: extract_hreflang(document),
        word_count: extract_word_count(document),
        body_text: if include_body {
            Some(extract_body_text(document))
        } else {
            None
        },
    }
}

fn extract_title(doc: &Html) -> Option<String> {
    let sel = Selector::parse("title").ok()?;
    doc.select(&sel)
        .next()
        .map(|el| el.text().collect::<String>().trim().to_string())
        .filter(|s| !s.is_empty())
}

fn extract_meta(doc: &Html, name: &str) -> Option<String> {
    let sel = Selector::parse(&format!("meta[name=\"{name}\"]")).ok()?;
    doc.select(&sel)
        .next()
        .and_then(|el| el.value().attr("content").map(|s| s.trim().to_string()))
        .filter(|s| !s.is_empty())
}

fn extract_link_rel(doc: &Html, rel: &str) -> Option<String> {
    let sel = Selector::parse(&format!("link[rel=\"{rel}\"]")).ok()?;
    doc.select(&sel)
        .next()
        .and_then(|el| el.value().attr("href").map(|s| s.trim().to_string()))
        .filter(|s| !s.is_empty())
}

fn extract_og_tags(doc: &Html) -> HashMap<String, String> {
    let mut tags = HashMap::new();
    if let Ok(sel) = Selector::parse("meta[property^=\"og:\"]") {
        for el in doc.select(&sel) {
            if let (Some(prop), Some(content)) =
                (el.value().attr("property"), el.value().attr("content"))
            {
                tags.insert(prop.to_string(), content.to_string());
            }
        }
    }
    tags
}

fn extract_headings(doc: &Html) -> HeadingData {
    let extract = |tag: &str| -> Vec<String> {
        Selector::parse(tag)
            .ok()
            .map(|sel| {
                doc.select(&sel)
                    .map(|el| el.text().collect::<String>().trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect()
            })
            .unwrap_or_default()
    };

    HeadingData {
        h1: extract("h1"),
        h2: extract("h2"),
        h3: extract("h3"),
    }
}

fn extract_links(doc: &Html, host: &str, base: Option<&url::Url>) -> LinkData {
    let mut internal = Vec::new();
    let mut external = Vec::new();

    let sel = match Selector::parse("a[href]") {
        Ok(s) => s,
        Err(_) => return LinkData::default(),
    };

    for el in doc.select(&sel) {
        let href = match el.value().attr("href") {
            Some(h) => h.trim().to_string(),
            None => continue,
        };

        // Skip anchors, javascript, mailto
        if href.starts_with('#') || href.starts_with("javascript:") || href.starts_with("mailto:") || href.starts_with("tel:") {
            continue;
        }

        let text = el.text().collect::<String>().trim().to_string();
        let nofollow = el
            .value()
            .attr("rel")
            .map(|r| r.contains("nofollow"))
            .unwrap_or(false);

        let resolved = if let Some(base) = base {
            base.join(&href)
                .map(|u| u.to_string())
                .unwrap_or(href.clone())
        } else {
            href.clone()
        };

        let is_internal = if let Ok(parsed) = url::Url::parse(&resolved) {
            parsed.host_str().unwrap_or_default() == host
        } else {
            href.starts_with('/')
        };

        let info = LinkInfo {
            href: resolved,
            text: if text.len() > 200 {
                format!("{}...", &text[..200])
            } else {
                text
            },
            nofollow,
        };

        if is_internal {
            internal.push(info);
        } else {
            external.push(info);
        }
    }

    LinkData { internal, external }
}

fn extract_images(doc: &Html) -> Vec<ImageData> {
    let sel = match Selector::parse("img") {
        Ok(s) => s,
        Err(_) => return vec![],
    };

    doc.select(&sel)
        .map(|el| {
            let v = el.value();
            ImageData {
                src: v.attr("src").unwrap_or_default().to_string(),
                alt: v.attr("alt").map(|s| s.to_string()),
                width: v.attr("width").map(|s| s.to_string()),
                height: v.attr("height").map(|s| s.to_string()),
                loading: v.attr("loading").map(|s| s.to_string()),
            }
        })
        .collect()
}

fn extract_schema(doc: &Html) -> Vec<SchemaData> {
    let sel = match Selector::parse("script[type=\"application/ld+json\"]") {
        Ok(s) => s,
        Err(_) => return vec![],
    };

    doc.select(&sel)
        .filter_map(|el| {
            let raw = el.text().collect::<String>().trim().to_string();
            if raw.is_empty() {
                return None;
            }

            // Try to extract @type
            let schema_type = serde_json::from_str::<serde_json::Value>(&raw)
                .ok()
                .and_then(|v| {
                    v.get("@type")
                        .and_then(|t| t.as_str())
                        .map(|s| s.to_string())
                })
                .unwrap_or_else(|| "Unknown".to_string());

            Some(SchemaData { schema_type, raw })
        })
        .collect()
}

fn extract_hreflang(doc: &Html) -> Vec<HreflangData> {
    let sel = match Selector::parse("link[rel=\"alternate\"][hreflang]") {
        Ok(s) => s,
        Err(_) => return vec![],
    };

    doc.select(&sel)
        .filter_map(|el| {
            let lang = el.value().attr("hreflang")?.to_string();
            let href = el.value().attr("href")?.to_string();
            Some(HreflangData { lang, href })
        })
        .collect()
}

fn extract_word_count(doc: &Html) -> usize {
    extract_body_text(doc).split_whitespace().count()
}

fn extract_body_text(doc: &Html) -> String {
    let body_sel = match Selector::parse("body") {
        Ok(s) => s,
        Err(_) => return String::new(),
    };

    doc.select(&body_sel)
        .next()
        .map(|body| {
            let mut text = String::new();
            for node in body.text() {
                let trimmed = node.trim();
                if !trimmed.is_empty() {
                    if !text.is_empty() {
                        text.push(' ');
                    }
                    text.push_str(trimmed);
                }
            }
            text
        })
        .unwrap_or_default()
}
