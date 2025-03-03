mod commands;

use clap::{Parser, Subcommand};
use commands::{flatten::flatten_main, Rating};

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Command
}

#[derive(Subcommand, Debug)]
enum Command {
    Flatten {
        #[arg(value_enum)]
        rating: Rating
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();
    dotenvy::dotenv()?;
    let cli = Cli::parse();
    
    match cli.command {
        Command::Flatten { rating } => flatten_main(rating).await?
    }
    
    Ok(())
}
