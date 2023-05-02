mod render;
mod rhai_ext;
mod utils;
mod sturcts;

use std::fs::File;
use std::io::Write;
use tracing::info;
use render::handlebars;
use sturcts::ArxivRender;
use utils::copy_statics_to_target;

use crate::config::Config;
use crate::core::ArxivCollection;

pub fn main(config: &Config, raw_data: ArxivCollection) -> anyhow::Result<()> {
    let mut render_data = ArxivRender::new(config.site_title.clone(), raw_data);
    render_data.sort();

    let hbs = handlebars(config)?;
    info!("Copying static files!");
    copy_statics_to_target(config)?;
    info!("Rendering templates!");
    let render_result = hbs.render("index", &render_data)?;
    let target_dir = std::path::Path::new(config.target_dir.as_str());
    let default_path = config
        .target_name
        .as_ref()
        .map(|s| s.as_str())
        .unwrap_or_else(|| "index.html");
    let index_path = target_dir.join(default_path);
    let mut output_file = File::create(&index_path)?;
    output_file.write_all(render_result.as_bytes())?;
    info!("{} generated", index_path.to_string_lossy());

    Ok(())
}