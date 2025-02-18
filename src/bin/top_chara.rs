use std::env;
use mongodb::Client as MongoClient;
use reqwest::Client as HttpClient;
use serde_json::Value;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();
    dotenvy::dotenv()?;

    let mongo_uri = env::var("MONGO_URI")?;
    let base_path = env::var("BASE_PATH")?;

    let mongo_client = MongoClient::with_uri_str(mongo_uri).await?;
    let db = mongo_client.database("anime");
    let collection = db.collection::<Value>("character");
    
    let http_client = HttpClient::new();
    const INTERVAL: u64 = 1500;

    Ok(())
}