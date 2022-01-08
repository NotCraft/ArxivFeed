use crate::structs::{Arxiv, ArxivQuery};
use crate::{ArxivCollection, Config};
use anyhow::Result;
use reqwest::{Client, IntoUrl};
use serde::de::DeserializeOwned;
use std::fs;
use std::fs::File;
use std::path::Path;
use tracing::{info, warn};
use xml::reader::{EventReader, XmlEvent};

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

pub fn dump_cache(cache_data: &ArxivCollection, config: &Config) -> Result<()> {
    fs::create_dir_all(&config.target_dir)?;
    let cache_path = Path::new(&config.target_dir).join("cache.json");

    info!("Dumping Cache: {}", cache_path.to_string_lossy());
    let mut f = File::create(cache_path)?;
    serde_json::to_writer(&mut f, &cache_data)?;
    Ok(())
}

/// Fetch the paper information using the arXiv API.
/// # Example
/// ```rust
/// use reqwest;
/// use arxiv::{fetch_arxivs, query};
///
/// let query = query!(search_query = "cat:cs.CL");
/// // arxivs type is Vec<Arxiv>
/// let client = reqwest::Client::new();
/// let arxivs = fetch_arxivs(query).await?;
/// ```
pub async fn fetch_arxivs(query: ArxivQuery, client: &Client) -> Result<Vec<Arxiv>> {
    let body = client.get(query.to_url()).send().await?.text().await?;
    let arxivs = parse_data(body)?;
    Ok(arxivs)
}

fn parse_data(body: String) -> Result<Vec<Arxiv>> {
    let mut parser = EventReader::from_str(&body);
    let mut arxiv = Arxiv::new();
    let mut arxivs = Vec::new();

    'outer: loop {
        match parser.next()? {
            XmlEvent::StartElement {
                name, attributes, ..
            } => match &name.local_name[..] {
                "entry" => {
                    arxiv = Arxiv::new();
                }
                "id" => {
                    if let XmlEvent::Characters(id) = parser.next()? {
                        arxiv.id = id;
                    }
                }
                "updated" => {
                    if let XmlEvent::Characters(updated) = parser.next()? {
                        arxiv.updated = updated.parse()?
                    }
                }
                "published" => {
                    if let XmlEvent::Characters(published) = parser.next()? {
                        arxiv.published = published.parse()?
                    }
                }
                "title" => {
                    if let XmlEvent::Characters(title) = parser.next()? {
                        arxiv.title = title
                    }
                }
                "summary" => {
                    if let XmlEvent::Characters(summary) = parser.next()? {
                        arxiv.summary = summary
                    }
                }
                "author" => {
                    parser.next()?;
                    parser.next()?;
                    if let XmlEvent::Characters(author) = parser.next()? {
                        arxiv.authors.push(author);
                    }
                }
                "link" => {
                    if attributes[0].value == "pdf" {
                        arxiv.pdf_url = format!(
                            "{}.pdf",
                            attributes[1].value.replacen("http", "https", 1).clone()
                        );
                    }
                }
                "comment" => {
                    if let XmlEvent::Characters(comment) = parser.next()? {
                        arxiv.comment = Some(comment);
                    }
                }
                _ => (),
            },
            XmlEvent::EndElement { name } => match &name.local_name[..] {
                "entry" => {
                    arxivs.push(arxiv.clone());
                }
                "feed" => {
                    break 'outer;
                }
                _ => (),
            },
            _ => (),
        }
    }
    Ok(arxivs)
}
