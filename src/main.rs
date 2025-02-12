use anyhow::Result;

use clap::{Parser, Subcommand};
use commands::{create_theme, init};

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
    Build,
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Init { name } => init::invoke(&name),
        Command::CreateTheme { name } => create_theme::invoke(&name),
        Command::Build => todo!(),
    }
}
