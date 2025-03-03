mod commands;

use std::env;
use clap::{Parser, Subcommand};
use commands::{flatten::flatten_main, Rating};
use mongodb::Client as MongoClient;

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

    let mongo_uri = env::var("MONGO_URI")?;
    let mongo_client = MongoClient::with_uri_str(mongo_uri).await?;
    let db = mongo_client.database("anime");

    match cli.command {
        Command::Flatten { rating } => flatten_main(rating, db).await?
    }
    
    Ok(())
}
