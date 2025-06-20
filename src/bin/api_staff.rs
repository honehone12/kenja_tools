use std::env;
use clap::Parser;
use mongodb::Client as MongoClient;
use reqwest::Client as HttpClient;

#[derive(Parser)]
#[command(version)]
struct Args {
    #[arg(long, default_value_t = 1500)]
    interval_mil: u64,
    #[arg(long, default_value_t = 5000)]
    timeout_mil: u64
}



#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();
    dotenvy::dotenv()?;
    let args = Args::parse();

    let mongo_uri = env::var("MONGO_URI")?;
    let mong_client = MongoClient::with_uri_str(mongo_uri).await?;

    let http_client = HttpClient::new();


    Ok(())
}