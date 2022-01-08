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
pub struct Source {
    pub(crate) limit: i32,
    pub(crate) title: String,
    pub(crate) category: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub(crate) limit_days: i64,
    pub(crate) site_title: String,
    pub(crate) target_dir: String,
    pub(crate) statics_dir: String,
    pub(crate) templates_dir: String,
    pub(crate) proxy: Option<String>,
    pub(crate) cache_url: Option<String>,
    pub(crate) target_name: Option<String>,
    pub(crate) sources: Vec<Source>,
    pub(crate) scripts: HashMap<String, String>,
}

impl Default for Config {
    fn default() -> Config {
        Config {
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
