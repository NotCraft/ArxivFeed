use crate::crate_name;
use figment::{
    error::Result,
    providers::Toml,
    providers::{Format, Serialized},
    Figment,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;


#[derive(Debug, Deserialize, Serialize)]
pub enum Version {
    V1,
    V2,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Source {
    pub limit: i32,
    pub title: String,
    pub category: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub version: Version,
    pub limit_days: i64,
    pub site_title: String,
    pub target_dir: String,
    pub statics_dir: String,
    pub templates_dir: String,
    pub proxy: Option<String>,
    pub cache_url: Option<String>,
    pub target_name: Option<String>,
    pub sources: Vec<Source>,
    pub scripts: HashMap<String, String>,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            version: Version::V1,
            limit_days: 1,
            site_title: crate_name!().to_string(),
            target_dir: "target".to_string(),
            statics_dir: "statics".to_string(),
            templates_dir: "includes".to_string(),
            proxy: None,
            cache_url: None,
            target_name: None,
            sources: Default::default(),
            scripts: Default::default(),
        }
    }
}

impl Config {
    pub fn new() -> Result<Config> {
        info!("Loading config!");
        Figment::from(Serialized::defaults(Config::default()))
            .merge(Toml::file("config.toml"))
            .extract()
    }
}
