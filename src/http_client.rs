use crate::cache::sqlite_cache::SqliteCache;
use reqwest::Client;
use serde::de::DeserializeOwned;
use std::path::PathBuf;
use std::sync::Arc;

pub struct HttpClient {
    client: Client,
    cache: Arc<SqliteCache>,
    cache_ttl_secs: u64,
}

impl HttpClient {
    pub fn new(cache_ttl_hours: u64) -> anyhow::Result<Self> {
        let mut db_path = PathBuf::from(".pkgrisk_cache.db");
        if let Some(home) = dirs::home_dir() {
            let cache_dir = home.join(".cache").join("pkgrisk");
            std::fs::create_dir_all(&cache_dir)?;
            db_path = cache_dir.join("cache.db");
        }

        let cache = SqliteCache::new(db_path)?;

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::USER_AGENT,
            reqwest::header::HeaderValue::from_static("pkgrisk/1.0"),
        );
        if let Ok(token) = std::env::var("GITHUB_TOKEN") {
            if let Ok(auth_val) = reqwest::header::HeaderValue::from_str(&format!("Bearer {}", token)) {
                headers.insert(reqwest::header::AUTHORIZATION, auth_val);
            }
        }

        let client = Client::builder().default_headers(headers).build()?;

        Ok(Self {
            client,
            cache: Arc::new(cache),
            cache_ttl_secs: cache_ttl_hours * 3600,
        })
    }

    pub async fn get_json<T: DeserializeOwned>(&self, url: &str) -> anyhow::Result<T> {
        if let Ok(Some(cached_value)) = self.cache.get(url, self.cache_ttl_secs) {
            if let Ok(parsed) = serde_json::from_str::<T>(&cached_value) {
                return Ok(parsed);
            }
        }

        let resp = self.client.get(url).send().await?;
        let text = resp.text().await?;
        
        let parsed: T = serde_json::from_str(&text)?;
        
        let _ = self.cache.set(url, &text);

        Ok(parsed)
    }
}
