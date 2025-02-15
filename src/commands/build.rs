use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{bail, Context, Result};
use tera::{Context as TeraContext, Tera};

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

fn save_rendered_file(file_path: &str, content: String) -> Result<()> {
    let file_save_dir = PathBuf::from("dist");

    fs::write(file_save_dir.join(file_path), content)
        .context(format!("Failed to save rendered file to `{}`", file_path))?;

    Ok(())
}

fn render_base_html(ctx: &TeraContext, tera_template: &Tera, theme_name: &str) -> Result<()> {
    // Find all .html file in the current level and render them with ctx
    // Save them with the respective title
    let base_path = format!("theme/{}", theme_name);
    for entry in fs::read_dir(base_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().map_or(false, |ext| ext == "html") {
            let file_name = path.file_name().unwrap().to_str().unwrap();
            let rendered_content = tera_template.render(file_name, ctx)?;

            save_rendered_file(file_name, rendered_content).context("Failed to save file")?;
        }
    }
    Ok(())
}

fn render_content_type_files(
    file_path: &str,
    ctx: &TeraContext,
    tera_template: &Tera,
) -> Result<()> {
    Ok(())
}

#[allow(dead_code)]
#[tokio::main]
pub(crate) async fn invoke() -> Result<()> {
    // Check if the config.toml file exists
    let config_file_exists = Path::new("config.toml");
    let theme_dir_exists = Path::new("theme");
    let content_dir_exists = Path::new("content");

    if !config_file_exists.try_exists()?
        || !theme_dir_exists.try_exists()?
        || !content_dir_exists.try_exists()?
    {
        bail!("Invalid SSG configurations")
    }

    // Read config.toml for prasing
    let config = read_config()?;

    fs::create_dir_all(&config.settings.out_dir).context(format!(
        "Unable to create {} folder",
        &config.settings.out_dir
    ))?;

    // Copy the static folder to the dist folder
    let asset_dir_exists = &Path::new(&config.settings.asset_dir);
    if asset_dir_exists.try_exists()? {
        // Create folder for asset in dist
        let dst_asset_path = &PathBuf::from(format!(
            "{}/{}",
            config.settings.out_dir, config.settings.asset_dir
        ));
        copy_dir(asset_dir_exists, dst_asset_path)
            .context("Error while copying the assets to destination")?;
    }

    // Render the index.html with the static data from config.toml
    let tera = Tera::new(&format!("theme/{}/**/*", config.settings.theme))
        .context("Something went wrong while startin gtemplate")?;
    let mut context = TeraContext::new();
    context.insert("settings", &config.settings);
    context.insert("custom", &config.custom);

    render_base_html(&context, &tera, &config.settings.theme)
        .context("Failed to render index.html file")?;

    // Render contents file based on the template from theme/{theme-name}/{cotent-type}/index.html
    // and/or template.html
    println!("Build completed successfully");
    Ok(())
}
