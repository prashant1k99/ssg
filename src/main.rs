use anyhow::Result;

use clap::{Parser, Subcommand};
use commands::init;

mod commands;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Init { name: String },
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Init { name } => init::invoke(&name),
    }
}
