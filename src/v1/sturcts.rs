use std::cmp::Reverse;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::core::{ArxivCollection, ArxivDaily};
use crate::{crate_homepage, crate_name, crate_version};


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

    pub fn sort(&mut self) {
        self.days.iter_mut().for_each(|s| s.sort());
    }
}