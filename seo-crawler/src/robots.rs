use reqwest::Client;
use serde::Serialize;
use tracing::{debug, warn};
use url::Url;

#[derive(Serialize)]
pub struct RobotsData {
    pub exists: bool,
    pub raw: String,
    pub disallowed: Vec<String>,
    pub sitemaps: Vec<String>,
    pub ai_crawlers: AiCrawlerStatus,

    #[serde(skip)]
    matcher: Option<texting_robots::Robot>,
}

#[derive(Serialize, Default)]
pub struct AiCrawlerStatus {
    #[serde(rename = "GPTBot")]
    pub gptbot: String,
    #[serde(rename = "ClaudeBot")]
    pub claudebot: String,
    #[serde(rename = "PerplexityBot")]
    pub perplexitybot: String,
    #[serde(rename = "Bytespider")]
    pub bytespider: String,
    #[serde(rename = "Google-Extended")]
    pub google_extended: String,
}

impl RobotsData {
    pub fn is_allowed(&self, url: &str) -> bool {
        match &self.matcher {
            Some(robot) => robot.allowed(url),
            None => true, // No robots.txt = everything allowed
        }
    }
}

pub async fn fetch_robots(client: &Client, base_url: &Url) -> RobotsData {
    let robots_url = format!(
        "{}://{}/robots.txt",
        base_url.scheme(),
        base_url.host_str().unwrap_or_default()
    );

    let resp = match client.get(&robots_url).send().await {
        Ok(r) => r,
        Err(e) => {
            warn!(error = %e, "failed to fetch robots.txt");
            return empty_robots();
        }
    };

    if !resp.status().is_success() {
        debug!(status = %resp.status(), "robots.txt not found");
        return empty_robots();
    }

    let raw = match resp.text().await {
        Ok(t) => t,
        Err(_) => return empty_robots(),
    };

    let matcher = texting_robots::Robot::new("NeboBot", raw.as_bytes()).ok();

    // Extract sitemaps
    let sitemaps: Vec<String> = raw
        .lines()
        .filter(|line| line.to_lowercase().starts_with("sitemap:"))
        .filter_map(|line| line.split_once(':').map(|(_, v)| v.trim().to_string()))
        .filter_map(|s| {
            // Handle "sitemap: url" where url might not have the scheme prefix stripped
            if s.starts_with("http") {
                Some(s)
            } else if s.starts_with("//") {
                Some(format!("https:{}", s))
            } else {
                // Reconstruct — the split on ':' may have consumed the scheme
                let full_line = raw
                    .lines()
                    .find(|l| l.to_lowercase().starts_with("sitemap:"))
                    .unwrap_or_default();
                let url_part = full_line
                    .splitn(2, ':')
                    .nth(1)
                    .map(|s| s.trim().to_string());
                url_part
            }
        })
        .collect();

    // Extract disallowed paths for all user agents
    let disallowed: Vec<String> = raw
        .lines()
        .filter(|line| line.to_lowercase().starts_with("disallow:"))
        .filter_map(|line| {
            line.splitn(2, ':')
                .nth(1)
                .map(|v| v.trim().to_string())
                .filter(|s| !s.is_empty())
        })
        .collect();

    // Check AI crawlers
    let ai_crawlers = check_ai_crawlers(&raw);

    RobotsData {
        exists: true,
        raw,
        disallowed,
        sitemaps,
        ai_crawlers,
        matcher,
    }
}

fn check_ai_crawlers(raw: &str) -> AiCrawlerStatus {
    let check = |bot: &str| -> String {
        let lower = raw.to_lowercase();
        let bot_lower = bot.to_lowercase();

        // Look for a User-agent line matching this bot
        let mut in_section = false;
        let mut has_disallow = false;

        for line in lower.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("user-agent:") {
                let agent = trimmed.split_once(':').unwrap().1.trim();
                in_section = agent == bot_lower || agent == "*";
            } else if in_section && trimmed.starts_with("disallow:") {
                let path = trimmed.split_once(':').unwrap().1.trim();
                if path == "/" {
                    return "blocked".to_string();
                }
                has_disallow = true;
            } else if trimmed.starts_with("user-agent:") {
                in_section = false;
            }
        }

        // Check if bot is specifically mentioned at all
        if raw.to_lowercase().contains(&bot_lower) {
            if has_disallow {
                "partially_blocked".to_string()
            } else {
                "allowed".to_string()
            }
        } else {
            "not_mentioned".to_string()
        }
    };

    AiCrawlerStatus {
        gptbot: check("GPTBot"),
        claudebot: check("ClaudeBot"),
        perplexitybot: check("PerplexityBot"),
        bytespider: check("Bytespider"),
        google_extended: check("Google-Extended"),
    }
}

fn empty_robots() -> RobotsData {
    RobotsData {
        exists: false,
        raw: String::new(),
        disallowed: vec![],
        sitemaps: vec![],
        ai_crawlers: AiCrawlerStatus::default(),
        matcher: None,
    }
}
