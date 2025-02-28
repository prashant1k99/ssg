use crate::utils::config_handler::{read_config, AppConfig};
use anyhow::{bail, Context, Ok, Result};
use gray_matter::{engine::YAML, Matter, Pod};
use pulldown_cmark::{html, Parser};
use std::{
    fs,
    path::{Path, PathBuf},
};
use tera::{to_value, Context as TeraContext, Tera};

fn copy_dir(src: &Path, dst: &Path) -> Result<()> {
    if src.is_dir() {
        fs::create_dir_all(dst).context("Failed to create destination directory")?;
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

fn create_dir_for_dist(file_path: &str, out_dir: &str) -> Result<()> {
    let file_save_dir = PathBuf::from(out_dir);
    fs::create_dir_all(file_save_dir.join(file_path))?;
    Ok(())
}

fn save_rendered_file(file_path: &str, content: String, out_dir: &str) -> Result<()> {
    let file_save_dir = PathBuf::from(out_dir);
    fs::write(file_save_dir.join(file_path), content)
        .context(format!("Failed to save rendered file to `{}`", file_path))?;
    Ok(())
}

fn render_base_html(ctx: &TeraContext, tera_template: &Tera, config: &AppConfig) -> Result<()> {
    // Find all .html file in the current level and render them with ctx
    // Save them with the respective title
    let base_path = PathBuf::from("theme").join(&config.settings.theme);
    for entry in fs::read_dir(&base_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().map_or(false, |ext| ext == "html") {
            let file_name = path.file_name().unwrap().to_str().unwrap();
            let rendered_content = tera_template.render(file_name, ctx)?;
            save_rendered_file(file_name, rendered_content, &config.settings.out_dir)
                .context("Failed to save file")?;
        }
    }
    Ok(())
}

fn render_content_type_index(
    ctx: &TeraContext,
    tera_template: &Tera,
    config: &AppConfig,
    template_path: &str,
) -> Result<()> {
    // now first render index.html
    let index_html_content = tera_template
        .render(template_path, ctx)
        .context(format!("Failed to render {}", template_path))?;

    create_dir_for_dist(
        template_path.strip_suffix("/index.html").unwrap(),
        &config.settings.out_dir,
    )
    .context(format!(
        "Failed to create dir: {}/{}",
        &config.settings.out_dir, template_path
    ))?;

    save_rendered_file(
        &format!(
            "{}/index.html",
            template_path.strip_suffix("/index.html").unwrap(),
        ),
        index_html_content,
        &config.settings.out_dir,
    )
    .context(format!("Failed to save {}/index.html", template_path))?;

    Ok(())
}

fn render_content_type_template(
    tera_template: &Tera,
    file_path: &PathBuf,
    template_path: &str,
    ctx: &TeraContext,
    config: &AppConfig,
) -> Result<()> {
    // Loop over every file and folder in the current dir
    for entry in fs::read_dir(file_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            render_content_type_template(tera_template, &path, template_path, ctx, config)?;
        } else if let "md" = path.extension().unwrap().to_str().unwrap() {
            // Read the data from md and prepare a context for the file
            let markdown = fs::read_to_string(&path).expect("Failed to read the Markdown file");
            println!("Path to the file: {:?}", path);

            let matter: Matter<YAML> = Matter::new();
            let result = matter.parse(&markdown);

            let parser = Parser::new(&result.content);
            let mut html_output = String::new();
            html::push_html(&mut html_output, parser);

            let mut file_context = TeraContext::new();

            let file_title = path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .strip_suffix(".md")
                .unwrap();

            if let Some(data) = &result.data {
                for (key, value) in data.as_hashmap().unwrap() {
                    println!("Key: {}", &key);
                    match value {
                        Pod::Array(arr) => {
                            let string_array: Vec<String> =
                                arr.iter().map(|v| v.as_string().unwrap()).collect();
                            file_context.insert(key, &string_array);
                        }
                        Pod::String(val) => {
                            file_context.insert(&key, &val.as_str());
                        }
                        _ => {}
                    }
                }
            }

            file_context.insert(
                "content",
                &to_value(&html_output).unwrap().as_str().unwrap(),
            );

            file_context.extend(ctx.clone());

            let rendered_page = tera_template.render(template_path, &file_context).unwrap();

            // remove the content from the path

            let file_path_to_save = path.strip_prefix("content").unwrap().parent().unwrap();

            create_dir_for_dist(
                file_path_to_save.to_str().unwrap(),
                &config.settings.out_dir,
            )
            .context(format!(
                "Failed to create dir: {}/{}",
                &config.settings.out_dir,
                file_path_to_save.to_str().unwrap()
            ))?;

            // Save the rendered file

            save_rendered_file(
                &format!(
                    "{}/{}.html",
                    file_path_to_save.to_str().unwrap(),
                    file_title
                ),
                rendered_page,
                &config.settings.out_dir,
            )
            .context(format!("Failed to save {}/index.html", template_path))?;

            println!(
                "Render using tempalte: {}, for file: {:?}, save path: {:?}",
                template_path, file_path, file_path_to_save
            );
        }
    }

    Ok(())
}

fn render_content_type_files(
    file_path: &PathBuf,
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

    render_content_type_index(
        ctx,
        tera_template,
        config,
        index_html_path
            .strip_prefix(format!("theme/{}", config.settings.theme))
            .unwrap()
            .to_str()
            .unwrap(),
    )
    .context("Failed to render index.html")?;

    render_content_type_template(
        tera_template,
        file_path,
        template_html_path
            .strip_prefix(format!("theme/{}", config.settings.theme))
            .unwrap()
            .to_str()
            .unwrap(),
        ctx,
        config,
    )?;

    println!("Successfully processed: {:?}", file_path);
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
    let tera = Tera::new(&format!("theme/{}/**/*.html", config.settings.theme))
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
