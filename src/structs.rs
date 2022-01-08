use crate::{crate_homepage, crate_name, crate_version};
use anyhow::Result;
use arxiv::Arxiv as RawArxiv;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};

pub type ArxivCollection = HashMap<DateTime<Utc>, HashMap<String, HashSet<Arxiv>>>;

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
    pub fn new(datetime: DateTime<Utc>, raw: HashMap<String, HashSet<Arxiv>>) -> ArxivDaily {
        let mut subjects = Vec::new();
        for (subject, collection) in raw {
            subjects.push(ArxivCategory {
                subject,
                papers: collection.into_iter().collect(),
            })
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
    pub fn new(raw: RawArxiv) -> Result<Arxiv> {
        Ok(Arxiv {
            id: raw.id,
            updated: raw.updated.parse()?,
            published: raw.published.parse()?,
            title: raw.title,
            summary: raw.summary,
            authors: raw.authors,
            pdf_url: raw.pdf_url,
            comment: raw.comment,
        })
    }
}
