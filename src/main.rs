mod cache;
mod cli;
mod config;
mod ecosystems;
mod http_client;
mod output;
mod scoring;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "pkgrisk")]
#[command(about = "5 second package health check", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Check a single package's health
    Check(cli::check::CheckArgs),
    /// Scan project dependencies
    Scan(cli::scan::ScanArgs),
    /// Compare multiple packages
    Compare(cli::compare::CompareArgs),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Check(args) => cli::check::run(args).await?,
        Commands::Scan(args) => cli::scan::run(args).await?,
        Commands::Compare(args) => cli::compare::run(args).await?,
    }

    Ok(())
}
