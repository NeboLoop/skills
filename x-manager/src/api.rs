use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result};
use hmac::{Hmac, Mac};
use reqwest::{Client, Response};
use sha1::Sha1;
use base64::{engine::general_purpose::STANDARD as B64, Engine};

use crate::auth::Credentials;

const API_BASE: &str = "https://api.x.com/2";

pub struct XClient {
    client: Client,
    creds: Credentials,
}

impl XClient {
    pub fn new(creds: &Credentials) -> Self {
        Self {
            client: Client::builder()
                .user_agent("NeboBot/1.0")
                .build()
                .expect("Failed to build HTTP client"),
            creds: creds.clone(),
        }
    }

    /// Make an authenticated GET request to the X API v2.
    pub async fn get(&self, endpoint: &str, params: &[(&str, &str)]) -> Result<serde_json::Value> {
        let url = format!("{API_BASE}{endpoint}");
        let auth_header = self.oauth_header("GET", &url, params)?;

        let mut req = self.client.get(&url).header("Authorization", auth_header);
        if !params.is_empty() {
            req = req.query(params);
        }

        let resp = req.send().await.context("Request failed")?;
        handle_response(resp).await
    }

    /// Make an authenticated POST request with JSON body.
    pub async fn post(&self, endpoint: &str, body: &serde_json::Value) -> Result<serde_json::Value> {
        let url = format!("{API_BASE}{endpoint}");
        let auth_header = self.oauth_header("POST", &url, &[])?;

        let resp = self
            .client
            .post(&url)
            .header("Authorization", auth_header)
            .header("Content-Type", "application/json")
            .json(body)
            .send()
            .await
            .context("Request failed")?;

        handle_response(resp).await
    }

    /// Make an authenticated DELETE request.
    pub async fn delete(&self, endpoint: &str) -> Result<serde_json::Value> {
        let url = format!("{API_BASE}{endpoint}");
        let auth_header = self.oauth_header("DELETE", &url, &[])?;

        let resp = self
            .client
            .delete(&url)
            .header("Authorization", auth_header)
            .send()
            .await
            .context("Request failed")?;

        handle_response(resp).await
    }

    /// Get the authenticated user's ID.
    pub async fn get_me(&self) -> Result<String> {
        let result = self.get("/users/me", &[]).await?;
        result["data"]["id"]
            .as_str()
            .map(|s| s.to_string())
            .context("Could not get user ID")
    }

    /// Look up a user ID by username.
    pub async fn get_user_id(&self, username: &str) -> Result<String> {
        let result = self.get(&format!("/users/by/username/{username}"), &[]).await?;
        result["data"]["id"]
            .as_str()
            .map(|s| s.to_string())
            .context(format!("User @{username} not found"))
    }

    /// Generate OAuth 1.0a Authorization header.
    fn oauth_header(&self, method: &str, url: &str, params: &[(&str, &str)]) -> Result<String> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_secs()
            .to_string();
        let nonce = uuid::Uuid::new_v4().to_string().replace('-', "");

        let mut oauth_params: BTreeMap<String, String> = BTreeMap::new();
        oauth_params.insert("oauth_consumer_key".into(), self.creds.api_key.clone());
        oauth_params.insert("oauth_nonce".into(), nonce);
        oauth_params.insert("oauth_signature_method".into(), "HMAC-SHA1".into());
        oauth_params.insert("oauth_timestamp".into(), timestamp);
        oauth_params.insert("oauth_token".into(), self.creds.access_token.clone());
        oauth_params.insert("oauth_version".into(), "1.0".into());

        // Add query params to signature base
        for (k, v) in params {
            oauth_params.insert(k.to_string(), v.to_string());
        }

        // Build parameter string
        let param_string: String = oauth_params
            .iter()
            .map(|(k, v)| format!("{}={}", percent_encode(k), percent_encode(v)))
            .collect::<Vec<_>>()
            .join("&");

        // Build signature base string
        let base_string = format!(
            "{}&{}&{}",
            method.to_uppercase(),
            percent_encode(url),
            percent_encode(&param_string)
        );

        // Sign with HMAC-SHA1
        let signing_key = format!(
            "{}&{}",
            percent_encode(&self.creds.api_secret),
            percent_encode(&self.creds.access_secret)
        );

        let mut mac = Hmac::<Sha1>::new_from_slice(signing_key.as_bytes())
            .context("HMAC creation failed")?;
        mac.update(base_string.as_bytes());
        let signature = B64.encode(mac.finalize().into_bytes());

        // Build Authorization header
        let auth = format!(
            r#"OAuth oauth_consumer_key="{}", oauth_nonce="{}", oauth_signature="{}", oauth_signature_method="HMAC-SHA1", oauth_timestamp="{}", oauth_token="{}", oauth_version="1.0""#,
            percent_encode(&self.creds.api_key),
            percent_encode(oauth_params.get("oauth_nonce").unwrap()),
            percent_encode(&signature),
            oauth_params.get("oauth_timestamp").unwrap(),
            percent_encode(&self.creds.access_token),
        );

        Ok(auth)
    }
}

async fn handle_response(resp: Response) -> Result<serde_json::Value> {
    let status = resp.status();
    let body = resp.text().await.context("Failed to read response body")?;

    if status.is_success() {
        let json: serde_json::Value =
            serde_json::from_str(&body).unwrap_or(serde_json::json!({"raw": body}));
        Ok(serde_json::json!({
            "success": true,
            "data": json.get("data").unwrap_or(&json)
        }))
    } else if status.as_u16() == 429 {
        anyhow::bail!("Rate limited by X API. Wait and try again.")
    } else {
        let error: serde_json::Value =
            serde_json::from_str(&body).unwrap_or(serde_json::json!({"raw": body}));
        Ok(serde_json::json!({
            "success": false,
            "status": status.as_u16(),
            "error": error
        }))
    }
}

fn percent_encode(s: &str) -> String {
    urlencoding::encode(s).to_string()
}
