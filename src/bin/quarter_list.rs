use std::{env, time::Duration};
use anirs_dev::Season;
use reqwest::{Client as HttpClient, StatusCode};
use mongodb::Client as MongoClient;
use serde_json::Value;
use tokio::{fs, time};
use tracing::info;
use anyhow::bail;

async fn request_quarter_list(
    http_client: &HttpClient,
    base_path: &str,
    year: u32,
    season: &str,
    interval_mil: u64
) -> anyhow::Result<Vec<Value>> {
    let mut list = vec![];
    let mut page = 0;
    let interval = Duration::from_millis(interval_mil);

    loop {
        page += 1;
        
        let url = match page {
            0..=1 => format!("{base_path}/seasons/{year}/{season}"),
            _ => format!("{base_path}/seasons/{year}/{season}?page={page}")
        };

        info!("requesting {url} ...");
        let res = http_client.get(&url).send().await?;
        let status = res.status();
        if status != StatusCode::OK {
            bail!("{url} respnsed {status}");
        }

        let text = res.text().await?;
        let mut json = serde_json::from_str::<Value>(&text)?;

        match json["data"] {
            Value::Null => bail!("property 'data' not found"),
            ref mut v => {
                if let Some(l) = v.as_array_mut() {
                    list.append(l);
                }
            }
        };
        match json["pagination"] {
            Value::Null => bail!("property 'pagination' not found"),
            ref p => {
                time::sleep(interval).await;
                
                if !matches!(p["has_next_page"], Value::Bool(true)) {
                    break;
                }
            }
        };
    }

    Ok(list)
}

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

    let http_client = HttpClient::new();
    const INTERVAL: u64 = 1500;

    let doc = fs::read_to_string(season_root).await?;
    let seasons = serde_json::from_str::<Vec<Season>>(&doc)?;

    for year in seasons {
        if year.year == 2025 {
            continue;
        }

        for quarter in year.seasons {
            let list = request_quarter_list(
                &http_client, 
                &base_path, 
                year.year, 
                &quarter, 
                INTERVAL
            ).await?;

            let res = collection.insert_many(list).await?;
            info!("inserted {}items", res.inserted_ids.len());
        }
    }    

    info!("done");
    Ok(())
}
