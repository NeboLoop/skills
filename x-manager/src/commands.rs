use anyhow::Result;
use serde_json::json;
use tracing::info;

use crate::api::XClient;
use crate::auth::Credentials;

// ── Posts ───────────────────────────────────────────────────────

pub async fn post_tweet(creds: &Credentials, text: &str, _media: Option<&str>) -> Result<serde_json::Value> {
    let client = XClient::new(creds);
    let body = json!({ "text": text });
    let result = client.post("/tweets", &body).await?;
    info!("posted tweet");
    Ok(result)
}

pub async fn post_thread(creds: &Credentials, tweets: &[&str]) -> Result<serde_json::Value> {
    let client = XClient::new(creds);
    let mut results = Vec::new();
    let mut reply_to: Option<String> = None;

    for (i, text) in tweets.iter().enumerate() {
        let body = if let Some(ref id) = reply_to {
            json!({
                "text": text.trim(),
                "reply": { "in_reply_to_tweet_id": id }
            })
        } else {
            json!({ "text": text.trim() })
        };

        let result = client.post("/tweets", &body).await?;

        if let Some(id) = result["data"]["id"].as_str() {
            reply_to = Some(id.to_string());
        }

        info!(tweet = i + 1, total = tweets.len(), "posted thread tweet");
        results.push(result);

        // Small delay between thread posts
        if i < tweets.len() - 1 {
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        }
    }

    Ok(json!({
        "success": true,
        "data": {
            "thread_length": results.len(),
            "tweets": results
        }
    }))
}

pub async fn reply_tweet(creds: &Credentials, tweet_id: &str, text: &str) -> Result<serde_json::Value> {
    let client = XClient::new(creds);
    let body = json!({
        "text": text,
        "reply": { "in_reply_to_tweet_id": tweet_id }
    });
    client.post("/tweets", &body).await
}

pub async fn delete_tweet(creds: &Credentials, tweet_id: &str) -> Result<serde_json::Value> {
    let client = XClient::new(creds);
    client.delete(&format!("/tweets/{tweet_id}")).await
}

// ── Reading ────────────────────────────────────────────────────

pub async fn get_timeline(creds: &Credentials, limit: u32) -> Result<serde_json::Value> {
    let client = XClient::new(creds);
    let me = client.get_me().await?;
    let limit_str = limit.to_string();
    client.get(
        &format!("/users/{me}/timelines/reverse_chronological"),
        &[
            ("max_results", limit_str.as_str()),
            ("tweet.fields", "created_at,public_metrics,author_id"),
        ],
    ).await
}

pub async fn get_user_posts(creds: &Credentials, username: &str, limit: u32) -> Result<serde_json::Value> {
    let client = XClient::new(creds);
    let user_id = client.get_user_id(username).await?;
    let limit_str = limit.to_string();
    client.get(
        &format!("/users/{user_id}/tweets"),
        &[
            ("max_results", limit_str.as_str()),
            ("tweet.fields", "created_at,public_metrics"),
        ],
    ).await
}

pub async fn search_tweets(creds: &Credentials, query: &str, limit: u32) -> Result<serde_json::Value> {
    let client = XClient::new(creds);
    let limit_str = limit.to_string();
    client.get(
        "/tweets/search/recent",
        &[
            ("query", query),
            ("max_results", limit_str.as_str()),
            ("tweet.fields", "created_at,public_metrics,author_id"),
        ],
    ).await
}

pub async fn get_tweet(creds: &Credentials, tweet_id: &str) -> Result<serde_json::Value> {
    let client = XClient::new(creds);
    client.get(
        &format!("/tweets/{tweet_id}"),
        &[("tweet.fields", "created_at,public_metrics,author_id,entities")],
    ).await
}

pub async fn get_mentions(creds: &Credentials, limit: u32) -> Result<serde_json::Value> {
    let client = XClient::new(creds);
    let me = client.get_me().await?;
    let limit_str = limit.to_string();
    client.get(
        &format!("/users/{me}/mentions"),
        &[
            ("max_results", limit_str.as_str()),
            ("tweet.fields", "created_at,public_metrics,author_id"),
        ],
    ).await
}

// ── Monitoring ─────────────────────────────────────────────────

pub async fn monitor_tweets(creds: &Credentials, query: &str, interval_secs: u64) -> Result<serde_json::Value> {
    let client = XClient::new(creds);
    let mut last_id: Option<String> = None;

    loop {
        let mut params = vec![
            ("query", query),
            ("max_results", "10"),
            ("tweet.fields", "created_at,public_metrics,author_id"),
        ];

        let since_id_str;
        if let Some(ref id) = last_id {
            since_id_str = id.clone();
            params.push(("since_id", &since_id_str));
        }

        match client.get("/tweets/search/recent", &params).await {
            Ok(result) => {
                if let Some(tweets) = result["data"].as_array() {
                    if !tweets.is_empty() {
                        // Update last_id to newest tweet
                        if let Some(id) = tweets[0]["id"].as_str() {
                            last_id = Some(id.to_string());
                        }

                        // Output each new tweet
                        for tweet in tweets {
                            println!("{}", serde_json::to_string(tweet).unwrap_or_default());
                        }
                        info!(count = tweets.len(), "new tweets found");
                    }
                }
            }
            Err(e) => {
                tracing::warn!(error = %e, "monitor poll failed");
            }
        }

        tokio::time::sleep(std::time::Duration::from_secs(interval_secs)).await;
    }
}

// ── Engagement ─────────────────────────────────────────────────

pub async fn like_tweet(creds: &Credentials, tweet_id: &str) -> Result<serde_json::Value> {
    let client = XClient::new(creds);
    let me = client.get_me().await?;
    let body = json!({ "tweet_id": tweet_id });
    client.post(&format!("/users/{me}/likes"), &body).await
}

pub async fn retweet(creds: &Credentials, tweet_id: &str) -> Result<serde_json::Value> {
    let client = XClient::new(creds);
    let me = client.get_me().await?;
    let body = json!({ "tweet_id": tweet_id });
    client.post(&format!("/users/{me}/retweets"), &body).await
}

pub async fn quote_tweet(creds: &Credentials, tweet_id: &str, text: &str) -> Result<serde_json::Value> {
    let client = XClient::new(creds);
    let body = json!({
        "text": text,
        "quote_tweet_id": tweet_id
    });
    client.post("/tweets", &body).await
}

pub async fn bookmark_tweet(creds: &Credentials, tweet_id: &str) -> Result<serde_json::Value> {
    let client = XClient::new(creds);
    let me = client.get_me().await?;
    let body = json!({ "tweet_id": tweet_id });
    client.post(&format!("/users/{me}/bookmarks"), &body).await
}

pub async fn get_bookmarks(creds: &Credentials, limit: u32) -> Result<serde_json::Value> {
    let client = XClient::new(creds);
    let me = client.get_me().await?;
    let limit_str = limit.to_string();
    client.get(
        &format!("/users/{me}/bookmarks"),
        &[("max_results", limit_str.as_str())],
    ).await
}

// ── Followers ──────────────────────────────────────────────────

pub async fn get_followers(creds: &Credentials, limit: u32, _new: bool) -> Result<serde_json::Value> {
    let client = XClient::new(creds);
    let me = client.get_me().await?;
    let limit_str = limit.to_string();
    client.get(
        &format!("/users/{me}/followers"),
        &[
            ("max_results", limit_str.as_str()),
            ("user.fields", "created_at,public_metrics,description"),
        ],
    ).await
}

pub async fn get_following(creds: &Credentials, limit: u32) -> Result<serde_json::Value> {
    let client = XClient::new(creds);
    let me = client.get_me().await?;
    let limit_str = limit.to_string();
    client.get(
        &format!("/users/{me}/following"),
        &[
            ("max_results", limit_str.as_str()),
            ("user.fields", "created_at,public_metrics,description"),
        ],
    ).await
}

pub async fn follow_user(creds: &Credentials, username: &str) -> Result<serde_json::Value> {
    let client = XClient::new(creds);
    let me = client.get_me().await?;
    let target = client.get_user_id(username).await?;
    let body = json!({ "target_user_id": target });
    client.post(&format!("/users/{me}/following"), &body).await
}

pub async fn unfollow_user(creds: &Credentials, username: &str) -> Result<serde_json::Value> {
    let client = XClient::new(creds);
    let me = client.get_me().await?;
    let target = client.get_user_id(username).await?;
    client.delete(&format!("/users/{me}/following/{target}")).await
}

// ── Trends ─────────────────────────────────────────────────────

pub async fn get_trends(creds: &Credentials, _location: u64) -> Result<serde_json::Value> {
    let client = XClient::new(creds);
    // X API v2 trends endpoint
    client.get("/trends/by/woeid/1", &[]).await
}

// ── Analytics ──────────────────────────────────────────────────

pub async fn get_analytics(creds: &Credentials, tweet_id: Option<&str>, _days: u32) -> Result<serde_json::Value> {
    let client = XClient::new(creds);

    if let Some(id) = tweet_id {
        // Get metrics for a specific tweet
        client.get(
            &format!("/tweets/{id}"),
            &[("tweet.fields", "public_metrics,organic_metrics,non_public_metrics,created_at")],
        ).await
    } else {
        // Get recent tweets with metrics
        let me = client.get_me().await?;
        client.get(
            &format!("/users/{me}/tweets"),
            &[
                ("max_results", "20"),
                ("tweet.fields", "public_metrics,created_at"),
            ],
        ).await
    }
}
