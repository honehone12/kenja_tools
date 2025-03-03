use std::{env, time::Duration};
use tokio::time;
use super::{paged_url, request};
use mongodb::Client as MongoClient;
use reqwest::Client as HttpClient;
use serde_json::Value;
use tracing::info;

pub(crate) async fn top_chara_main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();
    dotenvy::dotenv()?;

    let mongo_uri = env::var("MONGO_URI")?;
    let base_path = env::var("BASE_PATH")?;

    let mongo_client = MongoClient::with_uri_str(mongo_uri).await?;
    let db = mongo_client.database("anime");
    let collection = db.collection::<Value>("chara");
    
    let http_client = HttpClient::new();
    let interval = Duration::from_millis(1500);
    let path = format!("{base_path}/top/characters");
    let mut page = 0;

    loop {
        page += 1;

        let url = paged_url(&path, page);
        let (data, pagination) = request(&http_client, &url).await?;

        if data.is_empty() {
            info!("data is empty");
        } else {
            let res = collection.insert_many(data).await?;
            info!("inserted {}items", res.inserted_ids.len());
        }

        if !matches!(pagination["has_next_page"], Value::Bool(true)) {
            break;
        }

        time::sleep(interval).await;
    }

    info!("done");
    Ok(())
}
