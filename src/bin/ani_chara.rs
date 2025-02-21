use std::{env, time::Duration};
use anirs_dev::request;
use tokio::time;
use mongodb::{bson::{doc, Bson}, Client as MongoClient};
use reqwest::Client as HttpClient;
use serde_json::Value;
use tracing::{info, warn};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct AnimeCharacters {
    mal_id: i64,
    characters: Vec<Value>
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();
    dotenvy::dotenv()?;

    let mongo_uri = env::var("MONGO_URI")?;
    let base_path = env::var("BASE_PATH")?;

    let mongo_client = MongoClient::with_uri_str(mongo_uri).await?;
    let db = mongo_client.database("anime");
    let source = db.collection::<Value>("anime");
    let collection = db.collection::<AnimeCharacters>("anime_chara");

    let http_client = HttpClient::new();
    let interval = Duration::from_millis(1500);

    let list = source.distinct("mal_id", doc! {}).await?;
    let total = list.len();

    for (i, bson) in list.iter().enumerate() {
        if i <= 4603 {
            continue;
        }

        if let Bson::Int64(mal_id) = bson {
            info!("{i}/{total}");
            let url = format!("{base_path}/anime/{mal_id}/characters");
            let (data, _) = match request(&http_client, &url).await {
                Err(e) => {
                    warn!("request failed. {e}. skipping");
                    continue;
                }
                Ok(res) => res
            };

            if data.is_empty() {
                info!("data is empty");
            } else {
                let anime_chara = AnimeCharacters{
                    mal_id: *mal_id,
                    characters: data
                };
                _ = collection.insert_one(anime_chara).await?;
                info!("inserted a item");
            }
        } else {
            warn!("skipping unexpected value {i}/{total}:{bson}");
            continue;
        }

        time::sleep(interval).await;
    }

    Ok(())
}