use anyhow::Result;
use clap::{Parser, Subcommand};

mod commands;
use commands::add::AddCommand;

/// Main CLI structure for command-line argument parsing.
#[derive(Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    /// The command to execute, which can be a subcommand.
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Add a new component dependency
    #[command(subcommand)]
    Add(AddCommand),
}

#[tokio::main]
async fn main() -> Result<()> {
    let app = Cli::parse();

    match app.command {
        Commands::Add(cmd) => cmd.run().await?,
    }

    Ok(())
}
