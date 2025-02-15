use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{bail, Context, Result};
use tera::{Context as TeraContext, Tera};

use crate::utils::config_handler::{read_config, AppConfig};

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

fn save_rendered_file(file_path: &str, content: String, out_dir: &str) -> Result<()> {
    let file_save_dir = PathBuf::from(out_dir);

    if file_path.contains("/") {
        println!("Need to create dir");
        let dir_path = file_path
            .rsplit("/")
            .skip(1)
            .collect::<Vec<&str>>()
            .into_iter()
            .rev()
            .collect::<Vec<&str>>()
            .join("/");
        fs::create_dir_all(file_save_dir.join(dir_path))?;
    }

    fs::write(file_save_dir.join(file_path), content)
        .context(format!("Failed to save rendered file to `{}`", file_path))?;

    Ok(())
}

fn render_base_html(ctx: &TeraContext, tera_template: &Tera, config: &AppConfig) -> Result<()> {
    // Find all .html file in the current level and render them with ctx
    // Save them with the respective title
    let base_path = format!("theme/{}", config.settings.theme);
    for entry in fs::read_dir(base_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().map_or(false, |ext| ext == "html") {
            let file_name = path.file_name().unwrap().to_str().unwrap();
            println!("file_name: {}", file_name);
            let rendered_content = tera_template.render(file_name, ctx)?;

            save_rendered_file(file_name, rendered_content, &config.settings.out_dir)
                .context("Failed to save file")?;
        }
    }
    Ok(())
}

fn render_content_type_files(
    file_path: &Path,
    ctx: &TeraContext,
    tera_template: &Tera,
    config: &AppConfig,
) -> Result<()> {
    let content_type_path = PathBuf::from(format!(
        "theme/{}/{}",
        &config.settings.theme,
        &file_path.file_name().unwrap().to_str().unwrap()
    ));
    let index_html_path = content_type_path.join("index.html");
    let template_html_path = content_type_path.join("template.html");

    if !index_html_path.try_exists()? || !template_html_path.try_exists()? {
        bail!(format!(
            "Invalid theme, does not contain tempaltes for {:?}",
            file_path.file_name().unwrap()
        ));
    }

    // now first render index.html
    let template_path = file_path.file_name().unwrap().to_str().unwrap();
    let index_html_content = tera_template
        .render(&format!("{}/index.html", template_path), ctx)
        .context(format!("Failed to render {}/index.html", template_path))?;

    save_rendered_file(
        &format!("{}/index.html", template_path),
        index_html_content,
        &config.settings.out_dir,
    )
    .context(format!("Failed to save {}/index.html", template_path))?;
    // now check if there folder with {file_path.file_name()} exists and it should contain
    // index.html and template.html
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

    render_base_html(&context, &tera, &config).context("Failed to render index.html file")?;

    for entry in fs::read_dir("content")? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            render_content_type_files(&path, &context, &tera, &config)?;
        }
    }

    // Render contents file based on the template from theme/{theme-name}/{cotent-type}/index.html
    // and/or template.html
    println!("Build completed successfully");

    Ok(())
}
