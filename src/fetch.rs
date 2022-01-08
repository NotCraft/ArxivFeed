use crate::ArxivCollection;
use anyhow::Result;
use reqwest::{Client, IntoUrl};
use serde::de::DeserializeOwned;
use tracing::{info, warn};

async fn feed_cache<T, S>(url: T, client: &Client) -> Result<S>
where
    T: IntoUrl,
    S: DeserializeOwned,
{
    Ok(client.get(url).send().await?.json().await?)
}

pub async fn from_cache(url: &Option<String>, client: &Client) -> ArxivCollection {
    if let Some(cache_url) = url {
        info!("Feeding rss cache from {}", cache_url);
        match feed_cache(cache_url, &client).await {
            Ok(rss) => {
                info!("Feed rss cache Successfully!");
                rss
            }
            Err(err) => {
                warn!("Failed: {}!", err.to_string());
                Default::default()
            }
        }
    } else {
        Default::default()
    }
}
