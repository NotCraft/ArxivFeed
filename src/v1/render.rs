use crate::core::Arxiv;
use crate::Config;
use super::rhai_ext::{RegexPackage, RhaiMatch};
use super::utils::TEMPLATES_SRC;

use anyhow::Result;
use chrono::{DateTime, SecondsFormat, Utc};
use handlebars::Handlebars;
use handlebars::{no_escape, Context, Helper, Output, RenderContext, RenderError};
use regex::Regex;
use rhai::packages::Package;
use tracing::info;

pub fn handlebars(config: &Config) -> Result<Handlebars<'static>> {
    info!("Building Script Engine!");
    let mut engine = rhai::Engine::new();
    engine
        .register_type_with_name::<Regex>("Regex")
        .register_type_with_name::<RhaiMatch>("Match")
        .register_type_with_name::<Arxiv>("Arxiv");

    info!("Building Script Engine Done!");
    engine.register_global_module(RegexPackage::new().as_shared_module());

    info!("Building Handlebars Render Engine!");
    let mut handlebars = Handlebars::new();

    handlebars.set_engine(engine);
    handlebars.register_helper("time_format", Box::new(time_format_helper));

    handlebars.register_escape_fn(no_escape);
    handlebars.register_template_string("index", TEMPLATES_SRC)?;
    handlebars.register_templates_directory(".hbs", &config.templates_dir)?;

    for (name, script_path) in &config.scripts {
        info!("Loading Script: {} => {}", name, script_path);
        handlebars.register_script_helper_file(name, script_path)?;
    }

    info!("Building Handlebars Render Engine Done!");

    Ok(handlebars)
}

fn time_format_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    // get parameter from helper or throw an error
    let datetime: DateTime<Utc> = h
        .param(0)
        .and_then(|v| serde_json::from_value(v.value().clone()).unwrap())
        .ok_or_else(|| {
            RenderError::new("Param 0 (datetime) is required for time format helper.")
        })?;

    let fmt = h.param(1).and_then(|v| v.value().as_str());
    let rendered = match fmt {
        None => datetime.to_string(),
        Some("rfc2822") => datetime.to_rfc2822(),
        Some("rfc3339") => datetime.to_rfc3339(),
        Some("rfc3339_opts") => {
            let secform = h
                .hash_get("secform")
                .and_then(|v| v.value().as_str())
                .ok_or_else(|| {
                    RenderError::new(
                        "Param secform in [Secs,Millis,Micros,Nanos,AutoSi] required for datetime helper.",
                    )
                })?;

            let secform = match secform {
                "Secs" => SecondsFormat::Secs,
                "Millis" => SecondsFormat::Millis,
                "Micros" => SecondsFormat::Micros,
                "Nanos" => SecondsFormat::Nanos,
                "AutoSi" => SecondsFormat::AutoSi,
                _ => SecondsFormat::Secs,
            };

            let use_z = h
                .hash_get("use_z")
                .and_then(|v| v.value().as_bool())
                .ok_or_else(|| RenderError::new("Param use_z in required for datetime helper."))?;

            datetime.to_rfc3339_opts(secform, use_z)
        }
        Some(fmt) => datetime.format(fmt).to_string(),
    };
    out.write(&rendered)?;
    Ok(())
}
