use anyhow::Result;
use clap::{Parser, Subcommand};
use commands::{build, init, theme_handler};
use utils::config_handler;

mod commands;
mod utils;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Init {
        name: String,
    },
    CreateTheme {
        name: String,
    },
    SetTheme {
        name: String,
    },
    Build,
    Test,
    Dev {
        path: String,
        #[command(subcommand)]
        command: Option<DevCommand>,
    },
}

#[derive(Debug, Subcommand)]
enum DevCommand {
    CreateTheme { name: String },
    Build,
    Test,
    SetTheme { name: String },
}

fn main() -> Result<()> {
    let args = Args::parse();

    // If it's a Dev command, change directory before processing
    if let Command::Dev { path, .. } = &args.command {
        std::env::set_current_dir(path)?;
    }

    match args.command {
        Command::Init { name } => init::invoke(&name),
        Command::CreateTheme { name } => theme_handler::create_theme(&name),
        Command::Build => build::invoke(),
        Command::SetTheme { name } => theme_handler::set_theme(&name),
        Command::Test => {
            let config = config_handler::read_config()?;
            println!("Config: {:?}", config);
            Ok(())
        }
        Command::Dev { path: _, command } => match command {
            Some(DevCommand::CreateTheme { name }) => theme_handler::create_theme(&name),
            Some(DevCommand::Build) => build::invoke(),
            Some(DevCommand::SetTheme { name }) => theme_handler::set_theme(&name),
            Some(DevCommand::Test) => {
                let config = config_handler::read_config()?;
                println!("Config: {:?}", config);
                Ok(())
            }
            None => {
                let config = config_handler::read_config()?;
                println!("Running in development mode");
                println!("Config: {:?}", config);
                Ok(())
            }
        },
    }
}
