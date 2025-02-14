use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};

use crate::utils::{
    config_handler::{read_config, update_config},
    create_theme,
};

#[allow(dead_code)]
pub(crate) fn create_theme(name: &str) -> Result<()> {
    // Check if the config.toml exist in the current dir
    let file_eixsts = Path::new("config.toml");
    let theme_dir_exists = Path::new("theme");

    if !file_eixsts.try_exists()? || !theme_dir_exists.try_exists()? {
        bail!("SSG not initialized");
    }

    let theme_path = PathBuf::from(format!("{}/{}", "theme", name));
    create_theme::create_theme(theme_path).context("Failed to create new theme")?;

    println!("Theme skeleton created succesffully");
    Ok(())
}

#[allow(dead_code)]
pub(crate) fn set_theme(name: &str) -> Result<()> {
    // Check if the theme folder contains name theme
    let theme_path = PathBuf::from(format!("theme/{}", name));
    let does_theme_exists = Path::new(&theme_path);
    if !does_theme_exists.try_exists()? {
        bail!(format!("Unable to find theme `{}` in configuration.", name))
    }
    // Then update the config.toml for theme
    let mut config = read_config().context("Failed to read config.toml")?;
    config.theme = name.to_string();
    update_config(config).context("Failed to update config.toml for theme")?;

    println!("Successfully updated theme");
    Ok(())
}
