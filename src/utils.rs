use crate::config::Config;
use anyhow::Result;
use fs_extra::dir::{copy, CopyOptions};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[macro_export]
macro_rules! crate_name {
    () => {
        env!("CARGO_PKG_NAME")
    };
}

#[macro_export]
macro_rules! crate_homepage {
    () => {
        env!("CARGO_PKG_HOMEPAGE")
    };
}

#[macro_export]
macro_rules! crate_version {
    () => {
        env!("CARGO_PKG_VERSION")
    };
}

pub(crate) const TEMPLATES_SRC: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/vendor/system-templates/index.hbs"
));

pub(crate) const STATIC_CSS_SRC: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/vendor/system-statics/index.css"
));

pub(crate) const STATIC_JS_SRC: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/vendor/system-statics/index.js"
));

pub(crate) const STATIC_ICO_SRC: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/vendor/system-statics/favicon.ico"
));

pub fn copy_statics_to_target(config: &Config) -> Result<()> {
    fs::create_dir_all(&config.target_dir)?;
    if Path::new(&config.statics_dir).exists() {
        let mut options = CopyOptions::new();
        options.overwrite = true;
        copy(&config.statics_dir, &config.target_dir, &options)?;
    }
    let css_path = Path::new(&config.target_dir).join("index.css");
    if !css_path.exists() {
        let mut output_file = File::create(&css_path)?;
        output_file.write_all(STATIC_CSS_SRC.as_bytes())?;
    }
    let js_path = Path::new(&config.target_dir).join("index.js");
    if !js_path.exists() {
        let mut output_file = File::create(&js_path)?;
        output_file.write_all(STATIC_JS_SRC.as_bytes())?;
    }
    let ico_path = Path::new(&config.target_dir).join("favicon.ico");
    if !ico_path.exists() {
        let mut output_file = File::create(&ico_path)?;
        output_file.write_all(STATIC_ICO_SRC)?;
    }

    Ok(())
}
