use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{bail, Context, Result};

use crate::utils::config_handler::read_config;

fn copy_dir(src: &Path, dst: &Path) -> Result<()> {
    if src.is_dir() {
        fs::create_dir_all(dst).context("Errro while creating destination folder")?;
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let path = entry.path();
            let dest_path = dst.join(path.file_name().unwrap());

            if path.is_dir() {
                copy_dir(&path, &dest_path)?;
            } else {
                fs::copy(&path, &dest_path)?;
            }
        }
    }
    Ok(())
}

#[allow(dead_code)]
pub(crate) fn invoke() -> Result<()> {
    // Check if the config.toml file exists
    let config_file_exists = Path::new("config.toml");
    let theme_dir_exists = Path::new("theme");
    let content_dir_exists = Path::new("content");
    if !config_file_exists.try_exists()?
        || !theme_dir_exists.try_exists()?
        || !content_dir_exists.try_exists()?
    {
        if config_file_exists.try_exists()?
            || theme_dir_exists.try_exists()?
            || content_dir_exists.try_exists()?
        {
            bail!("Invalid SSG configurations")
        } else {
            bail!("SSG not initialized");
        }
    }
    // Check for theme folder
    // Check for content folder
    // Read config.toml for prasing
    let config = read_config()?;
    // Prepare a dist folder

    fs::create_dir_all(&config.out_dir)
        .context(format!("Unable to create {} folder", &config.out_dir))?;
    // Copy the static folder to the dist folder
    let asset_dir_exists = &Path::new(&config.asset_dir);
    if asset_dir_exists.try_exists()? {
        // Create folder for asset in dist
        let dst_asset_path = &PathBuf::from(format!("{}/{}", config.out_dir, config.asset_dir));
        copy_dir(asset_dir_exists, dst_asset_path)
            .context("Error while copying the assets to destination")?;
    }
    // Render the index.html with the static data from config.toml
    // Render contents file based on the template from theme/{theme-name}/{cotent-type}/index.html
    // and/or template.html
    println!("Build completed successfully");
    Ok(())
}
