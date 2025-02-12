use std::io::Write;
use std::{fs, path::PathBuf};

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    pub title: String,
    pub theme: String,
    pub out_dir: String,
}

pub(crate) fn create_config(app_name: &str, config: AppConfig) -> Result<()> {
    let toml_config = toml::to_string(&config)?;
    let file_path = PathBuf::from(format!("{}/{}", app_name, "config.toml"));
    let mut file = fs::File::create(file_path)?;

    file.write_all(toml_config.as_bytes())?;
    Ok(())
}
