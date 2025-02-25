use std::{env, time::Duration};
use tokio::time;
use anirs_dev::{AnimeCharacters, request};
use mongodb::{bson::{doc, Bson}, Client as MongoClient};
use serde_json::Value;
use reqwest::Client as HttpClient;
use tracing::{info, warn};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();
    dotenvy::dotenv()?;

    let mongo_uri = env::var("MONGO_URI")?;
    let base_path = env::var("BASE_PATH")?;

    let mongo_client = MongoClient::with_uri_str(mongo_uri).await?;
    let db = mongo_client.database("anime");
    let ani_colle = db.collection::<Value>("anime");
    let ani_chara_colle = db.collection::<AnimeCharacters>("anime_chara");
    
    let anime_list = ani_colle.distinct("mal_id", doc! {}).await?;
    let anime_chara_list = ani_chara_colle.distinct("mal_id", doc! {}).await?;

    let http_client = HttpClient::new();
    let interval = Duration::from_millis(1500);

    let mut lost_list = vec![];

    for (i, mal_id) in anime_list.iter().enumerate() {
        if i > 3500 {
            break;
        }
        
        if let Bson::Int64(id) = mal_id {
            if !anime_chara_list.contains(mal_id) {
                info!("{i}:{id} is lost");
                lost_list.push(*id);
            }
        } else {
            warn!("unexpected value {i}:{mal_id}");
        }
    }

    let total = lost_list.len();
    info!("{total} lost found");

    for (i, mal_id) in lost_list.iter().enumerate() {
        info!("{i}/{total}");
        let url = format!("{base_path}/anime/{mal_id}/characters");
        match request(&http_client, &url).await {
            Err(e) => warn!("request failed. {e}. skipping"),
            Ok((data, _)) => {
                if data.is_empty() {
                    info!("data is empty");
                } else {
                    let anime_chara = AnimeCharacters{
                        mal_id: *mal_id,
                        characters: data
                    };
                    _ = ani_chara_colle.insert_one(anime_chara).await?;
                    info!("inserted a item");
                }
            }
        }

        time::sleep(interval).await;
    }

    info!("done");

    Ok(())
}
