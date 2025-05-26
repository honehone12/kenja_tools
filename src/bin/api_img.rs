use std::env;
use tokio::fs;
use mongodb::Client as MongoClient;
use reqwest::Client as HttpClient;
use clap::Parser;
use kenja_tools::documents::Rating;

#[derive(Parser)]
#[command(version)]
struct Args {
    #[arg(default_value = "json")]
    json_path: String,
    img_path: String
}

async fn img(
    args: Args, 
    mongo_client: MongoClient,
    http_client: HttpClient
) -> anyhow::Result<()> {


    Ok(())
} 

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();
    dotenvy::dotenv()?;
    let args = Args::parse();

    let mongo_uri = env::var("MONGO_URI")?;
    let mongo_client = MongoClient::with_uri_str(mongo_uri).await?;

    let http_client = HttpClient::new();

    img(args, mongo_client, http_client).await
}
