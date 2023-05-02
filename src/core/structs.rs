use anyhow::Result;
use chrono::{DateTime, Utc};
use indexmap::{IndexMap, IndexSet};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;

pub type ArxivCollection = IndexMap<DateTime<Utc>, IndexMap<String, IndexSet<Arxiv>>>;

#[derive(Serialize, Deserialize, Debug, Hash, Clone, Eq, PartialEq)]
pub struct ArxivDaily {
    pub datetime: DateTime<Utc>,
    pub subjects: Vec<ArxivCategory>,
}

impl ArxivDaily {
    pub fn new(datetime: DateTime<Utc>, raw: IndexMap<String, IndexSet<Arxiv>>) -> ArxivDaily {
        let mut subjects = Vec::new();
        for (subject, collection) in raw {
            let papers: Vec<Arxiv> = collection.into_iter().collect();
            subjects.push(ArxivCategory { subject, papers })
        }
        ArxivDaily { datetime, subjects }
    }

    pub fn sort(&mut self) {
        self.subjects.iter_mut().for_each(|s| s.sort());
    }
}

#[derive(Serialize, Deserialize, Debug, Hash, Clone, Eq, PartialEq)]
pub struct ArxivCategory {
    pub subject: String,
    pub papers: Vec<Arxiv>,
}

impl ArxivCategory {
    pub fn sort(&mut self) {
        self.papers.sort_by_key(|p| p.updated != p.published)
    }
}

/// A structure that stores the paper information.
#[derive(Serialize, Deserialize, Debug, Hash, Clone, Eq, PartialEq)]
pub struct Arxiv {
    pub id: String,
    pub updated: DateTime<Utc>,
    pub published: DateTime<Utc>,
    pub title: String,
    pub summary: String,
    pub authors: Vec<String>,
    pub pdf_url: String,
    pub comment: Option<String>,
}

#[allow(dead_code)]
impl Arxiv {
    pub fn new() -> Arxiv {
        Arxiv {
            updated: Utc::now(),
            published: Utc::now(),
            id: Default::default(),
            title: Default::default(),
            summary: Default::default(),
            authors: Default::default(),
            pdf_url: Default::default(),
            comment: Default::default(),
        }
    }

    /// Save the paper as a pdf from the information stored by the structure.
    pub async fn fetch_pdf(&self, out_path: &str) -> Result<()> {
        let body = reqwest::get(&self.pdf_url).await?.bytes().await?;
        let out_path = if out_path.ends_with(".pdf") {
            out_path.to_string()
        } else {
            format!("{}.pdf", out_path)
        };
        let mut file = fs::File::create(out_path)?;
        file.write_all(&body)?;
        Ok(())
    }
}

/// A structure that stores the query information.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ArxivQuery {
    pub base_url: String,
    pub search_query: String,
    pub id_list: String,
    pub start: Option<i32>,
    pub max_results: Option<i32>,
    pub sort_by: String,
    pub sort_order: String,
}

/// A builder of ArxivQuery
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct ArxivQueryBuilder {
    pub base_url: String,
    pub search_query: String,
    pub id_list: String,
    pub start: Option<i32>,
    pub max_results: Option<i32>,
    pub sort_by: String,
    pub sort_order: String,
}
