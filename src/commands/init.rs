use std::{collections::HashMap, fs, path::PathBuf};

use anyhow::{Context, Result};

use crate::utils::{
    config_handler::{create_config, AppConfig, Settings},
    create_theme::create_theme,
};

fn create_content(name: &str) -> Result<()> {
    static TEST_POST: &str = include_str!("../asset/content/posts/tests.md");

    let output_path = PathBuf::from(format!("{}/{}/posts", name, "content"));
    fs::create_dir_all(&output_path)
        .context("Unable to create content directory")
        .unwrap();

    let output_path = output_path.join("tests.md");

    fs::write(&output_path, TEST_POST).unwrap();

    Ok(())
}

#[allow(dead_code)]
pub(crate) fn invoke(name: &str) -> Result<()> {
    create_content(name).context("Unable to create cotnent")?;

    let theme_name = "psc";
    let theme_path = PathBuf::from(format!("{}/{}/{}", name, "theme", theme_name));

    create_theme(theme_path).context("unable to create default theme")?;

    fs::create_dir_all(format!("{}/{}", name, "asset")).context("Failed to create asset dir")?;

    create_config(
        name,
        AppConfig {
            settings: Settings {
                title: name.to_string(),
                out_dir: String::from("dist"),
                theme: theme_name.to_string(),
                asset_dir: String::from("asset"),
            },
            custom: HashMap::new(),
        },
    )
    .context("Unable to create config file")?;

    println!("Project created successfully");

    Ok(())
}
