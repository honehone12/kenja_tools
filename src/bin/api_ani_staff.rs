use std::env;
use mongodb::Client as MongoClient;
use reqwest::Client as HttpClient;
use clap::Parser;
use kenja_tools::{api::request_anime_api, documents::anime_raw::StaffsRaw};

#[derive(Parser)]
#[command(version)]
struct Args {
    #[arg(long, default_value_t = 1500)]
    interval_mil: u64,
    #[arg(long, default_value_t = 10000)]
    timeout_mil: u64
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();
    dotenvy::dotenv()?;
    let args = Args::parse();

    let mongo_uri = env::var("MONGO_URI")?;
    let mongo_client = MongoClient::with_uri_str(mongo_uri).await?;

    let http_client = HttpClient::new();
    
    request_anime_api::<StaffsRaw>(
        args.interval_mil, 
        args.timeout_mil,
        "staff",
        mongo_client, 
        http_client
    ).await
}
