mod config;
mod fetch;
mod render;
mod rhai_ext;
mod structs;
mod utils;

use crate::config::Config;
use crate::fetch::from_cache;
use crate::structs::{Arxiv, ArxivCollection, ArxivRender};
use crate::utils::copy_statics_to_target;
use anyhow::Result;
use arxiv::ArxivQueryBuilder;
use render::handlebars;
use std::fs::File;
use std::io::Write;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::new()?;
    let client = reqwest::Client::builder().build()?;

    let mut raw_data: ArxivCollection = from_cache(&config.cache_url, &client).await;
    for source in &config.sources {
        let query = ArxivQueryBuilder::new()
            .search_query(&format!("cat:{}", source.category))
            .start(0)
            .max_results(source.limit)
            .sort_by("lastUpdatedDate") // "lastUpdatedDate" | "submittedDate"
            .sort_order("descending")
            .build();
        let arxivs = arxiv::fetch_arxivs(query).await?;
        for arxiv in arxivs {
            let arxiv = Arxiv::new(arxiv)?;
            let entry = raw_data
                .entry(arxiv.updated.date().and_hms(0, 0, 0))
                .or_default();
            let entry = entry.entry(String::from(&source.title)).or_default();
            entry.insert(arxiv);
        }
    }

    let render_data = ArxivRender::new(config.site_title.clone(), raw_data);

    let hbs = handlebars(&config)?;
    info!("Copying static files!");
    copy_statics_to_target(&config)?;
    info!("Rendering templates!");
    let render_result = hbs.render("index", &render_data)?;
    let target_dir = std::path::Path::new(&config.target_dir);
    let default_path = &config
        .target_name
        .unwrap_or_else(|| "index.html".to_string());
    let index_path = target_dir.join(default_path);
    let mut output_file = File::create(&index_path)?;
    output_file.write_all(render_result.as_bytes())?;
    println!("{} generated", index_path.to_string_lossy());
    Ok(())
}
