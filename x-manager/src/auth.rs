use std::path::PathBuf;

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use anyhow::{Context, Result};
use base64::{engine::general_purpose::STANDARD as B64, Engine};
use rand::RngCore;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Credentials {
    pub api_key: String,
    pub api_secret: String,
    pub access_token: String,
    pub access_secret: String,
    #[serde(skip)]
    pub user_id: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct EncryptedStore {
    nonce: String,
    data: String,
}

fn config_dir() -> Result<PathBuf> {
    let dir = dirs::config_dir()
        .context("Cannot determine config directory")?
        .join("nebo")
        .join("x-manager");
    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}

fn creds_path(account: &str) -> Result<PathBuf> {
    Ok(config_dir()?.join(format!("{account}.enc")))
}

/// Derive a simple key from machine-specific data.
/// In production, integrate with OS keyring.
fn derive_key() -> [u8; 32] {
    use sha2::Digest;
    let hostname = hostname::get()
        .map(|h| h.to_string_lossy().to_string())
        .unwrap_or_else(|_| "nebo-default".to_string());
    let seed = format!("nebo-x-manager-{hostname}-salt-v1");
    let mut hasher = sha2::Sha256::new();
    hasher.update(seed.as_bytes());
    let result = hasher.finalize();
    let mut key = [0u8; 32];
    key.copy_from_slice(&result);
    key
}

pub fn store_credentials(
    account: &str,
    api_key: &str,
    api_secret: &str,
    access_token: &str,
    access_secret: &str,
) -> Result<()> {
    let creds = Credentials {
        api_key: api_key.to_string(),
        api_secret: api_secret.to_string(),
        access_token: access_token.to_string(),
        access_secret: access_secret.to_string(),
        user_id: None,
    };

    let json = serde_json::to_string(&creds)?;
    let key = derive_key();
    let cipher = Aes256Gcm::new_from_slice(&key).context("Failed to create cipher")?;

    let mut nonce_bytes = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let encrypted = cipher
        .encrypt(nonce, json.as_bytes())
        .map_err(|e| anyhow::anyhow!("Encryption failed: {e}"))?;

    let store = EncryptedStore {
        nonce: B64.encode(nonce_bytes),
        data: B64.encode(encrypted),
    };

    let path = creds_path(account)?;
    std::fs::write(&path, serde_json::to_string_pretty(&store)?)?;
    tracing::info!(account = %account, path = %path.display(), "credentials stored");

    Ok(())
}

pub fn load_credentials(account: &str) -> Result<Credentials> {
    let path = creds_path(account)?;
    let content = std::fs::read_to_string(&path)
        .with_context(|| format!("No credentials found for account '{account}'. Run: x-manager auth --api-key ... --api-secret ... --access-token ... --access-secret ..."))?;

    let store: EncryptedStore = serde_json::from_str(&content)?;
    let key = derive_key();
    let cipher = Aes256Gcm::new_from_slice(&key).context("Failed to create cipher")?;

    let nonce_bytes = B64.decode(&store.nonce)?;
    let nonce = Nonce::from_slice(&nonce_bytes);
    let encrypted = B64.decode(&store.data)?;

    let decrypted = cipher
        .decrypt(nonce, encrypted.as_ref())
        .map_err(|e| anyhow::anyhow!("Decryption failed: {e}"))?;

    let creds: Credentials = serde_json::from_slice(&decrypted)?;
    Ok(creds)
}
