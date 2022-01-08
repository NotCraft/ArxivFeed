use crate::{crate_homepage, crate_name, crate_version};
use anyhow::Result;
use chrono::{DateTime, Utc};
use indexmap::{IndexMap, IndexSet};
use serde::{Deserialize, Serialize};
use std::cmp::Reverse;
use std::fs;
use std::io::Write;

pub type ArxivCollection = IndexMap<DateTime<Utc>, IndexMap<String, IndexSet<Arxiv>>>;

#[derive(Serialize, Deserialize, Debug, Hash, Clone, Eq, PartialEq)]
pub struct ArxivRender {
    pub site_title: String,
    pub build_time: DateTime<Utc>,
    pub project_name: &'static str,
    pub project_version: &'static str,
    pub project_homepage: &'static str,
    pub days: Vec<ArxivDaily>,
}

impl ArxivRender {
    pub fn new(title: String, raw: ArxivCollection) -> ArxivRender {
        let mut days = Vec::new();
        for (date, collection) in raw {
            days.push(ArxivDaily::new(date, collection))
        }
        days.sort_by_key(|x| Reverse(x.datetime));
        ArxivRender {
            site_title: title,
            build_time: Utc::now(),
            project_name: crate_name!(),
            project_version: crate_version!(),
            project_homepage: crate_homepage!(),
            days,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Hash, Clone, Eq, PartialEq)]
pub struct ArxivDaily {
    datetime: DateTime<Utc>,
    subjects: Vec<ArxivCategory>,
}

impl ArxivDaily {
    pub fn new(datetime: DateTime<Utc>, raw: IndexMap<String, IndexSet<Arxiv>>) -> ArxivDaily {
        let mut subjects = Vec::new();
        for (subject, collection) in raw {
            let mut papers: Vec<Arxiv> = collection.into_iter().collect();
            subjects.push(ArxivCategory { subject, papers })
        }
        ArxivDaily { datetime, subjects }
    }
}

#[derive(Serialize, Deserialize, Debug, Hash, Clone, Eq, PartialEq)]
pub struct ArxivCategory {
    subject: String,
    papers: Vec<Arxiv>,
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
