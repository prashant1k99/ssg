use std::{fs, path::PathBuf};

use anyhow::{Context, Result};

pub(crate) fn create_theme(app_name: &str, name: &str) -> Result<()> {
    let theme_path = PathBuf::from(format!("{}/{}/{}", app_name, "theme", name));
    fs::create_dir_all(theme_path.join("posts"))
        .context("Unable to create content directory")
        .unwrap();
    // Copy index.html
    let index_file = include_str!("../asset/theme/psc/index.html");
    let index_file_path = theme_path.join("index.html");
    fs::write(&index_file_path, index_file).context("Failed to create theme index.html")?;

    // Copy posts/index.html
    let posts_index_file = include_str!("../asset/theme/psc/posts/index.html");
    let posts_index_file_path = theme_path.join("posts/index.html");
    fs::write(&posts_index_file_path, posts_index_file)
        .context("Failed to create theme posts/index.html")?;
    // Copy posts/template.html
    let posts_index_file = include_str!("../asset/theme/psc/posts/template.html");
    let posts_index_file_path = theme_path.join("posts/template.html");
    fs::write(&posts_index_file_path, posts_index_file)
        .context("Failed to create theme posts/template.html")?;

    Ok(())
}
