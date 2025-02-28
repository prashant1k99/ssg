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
    Init { name: String },
    CreateTheme { name: String },
    SetTheme { name: String },
    Build,
    Test,
}

fn main() -> Result<()> {
    let args = Args::parse();

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
    }
}
