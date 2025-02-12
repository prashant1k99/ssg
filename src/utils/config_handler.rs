use std::io::{Read, Write};
use std::path::Path;
use std::{fs, path::PathBuf};

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    pub title: String,
    pub theme: String,
    #[serde(default = "default_out_dir")] // Optional with default
    pub out_dir: String,
}

fn default_out_dir() -> String {
    "dist".to_string()
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            title: String::new(),
            theme: String::new(),
            out_dir: default_out_dir(),
        }
    }
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

    toml::from_str(&contents).map_err(|e| {
        if e.to_string().contains("missing field") {
            anyhow::anyhow!("Configuration error = {}", e.message())
        } else {
            anyhow::anyhow!("Invalid configuration format: {}", e)
        }
    })
}
