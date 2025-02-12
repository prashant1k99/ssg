use std::io::{Read, Write};
use std::path::Path;
use std::{fs, path::PathBuf};

use anyhow::{bail, Result};
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

pub(crate) fn read_config() -> Result<AppConfig> {
    let config_path = Path::new("config.toml");

    if !config_path.try_exists()? {
        bail!("Please initialize SSG first")
    }
    let mut file = fs::File::open(config_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let config: AppConfig = toml::from_str(&contents)?;

    Ok(config)
}
