use std::env;
use reqwest::Client as HttpClient;
use mongodb::Client as MongoClient;
use serde_json::Value;
use tokio::fs;
use tracing::info;
use anirs_dev::{Season, request_pages};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();
    dotenvy::dotenv()?;
    
    let mongo_uri = env::var("MONGO_URI")?;
    let season_root = env::var("SEASON_ROOT")?;
    let base_path = env::var("BASE_PATH")?;

    let mongo_client = MongoClient::with_uri_str(mongo_uri).await?;
    let db = mongo_client.database("anime");
    let collection = db.collection::<Value>("anime");

    let doc = fs::read_to_string(season_root).await?;
    let seasons = serde_json::from_str::<Vec<Season>>(&doc)?;

    let http_client = HttpClient::new();
    const INTERVAL: u64 = 1500;

    for year in seasons {
        for quarter in year.seasons {
            let path = format!("{base_path}/seasons/{}/{quarter}", year.year);
            let list = request_pages(
                &http_client, 
                &path, 
                INTERVAL
            ).await?;

            if list.is_empty() {
                info!("list is empty");
                continue;
            }

            let res = collection.insert_many(list).await?;
            info!("inserted {}items", res.inserted_ids.len());
        }
    }    

    info!("done");
    Ok(())
}
