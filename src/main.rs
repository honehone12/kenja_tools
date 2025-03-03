mod commands;

use clap::Parser;
use clap::{Subcommand, ValueEnum};

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

#[derive(ValueEnum, Clone, Debug)]
enum Rating {
    AllAges,
    Hentai
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();
    dotenvy::dotenv()?;
    let cli = Cli::parse();
    match cli.command {
        Command::Flatten { rating } => {
            
        }
    }
    
    Ok(())
}