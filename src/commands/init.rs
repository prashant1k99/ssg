use std::{fs, path::PathBuf};

use anyhow::{Context, Result};

use crate::utils::{
    config_handler::{create_config, AppConfig},
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

pub(crate) fn invoke(name: &str) -> Result<()> {
    create_content(name).context("Unable to create cotnent")?;

    let theme_name = "psc";
    create_theme(name, theme_name).context("unable to create default theme")?;
    create_config(
        name,
        AppConfig {
            title: name.to_string(),
            out_dir: String::from("dist"),
            theme: theme_name.to_string(),
        },
    )
    .context("Unable to create config file")?;

    println!("Project created successfully");

    Ok(())
}
