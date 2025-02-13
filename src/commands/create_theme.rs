use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};

use crate::utils::create_theme;

#[allow(dead_code)]
pub(crate) fn invoke(name: &str) -> Result<()> {
    // Check if the config.toml exist in the current dir
    let file_eixsts = Path::new("config.toml");
    let theme_dir_exists = Path::new("theme");

    if !file_eixsts.try_exists()? || !theme_dir_exists.try_exists()? {
        bail!("SSG not initialized");
    }

    let theme_path = PathBuf::from(format!("{}/{}", "theme", name));
    create_theme::create_theme(theme_path).context("Failed to create new theme")?;
    Ok(())
}
