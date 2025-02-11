use std::fs;

use anyhow::Result;

pub(crate) fn invoke(name: &str) -> Result<()> {
    // Create folder and files for the static site generator

    // Parent folder
    fs::create_dir(name).unwrap();
    // Create theme folder
    // Create
    println!("Initialize project wtih {name}");
    Ok(())
}
