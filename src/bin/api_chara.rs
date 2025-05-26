use std::{env, time::Duration};
use tokio::time;
use mongodb::Client as MongoClient;
use reqwest::Client as HttpClient;
use clap::Parser;
use serde_json::Value;
use tracing::info;
use kenja_tools::api::{paged_url, request};

#[derive(Parser)]
struct Args {
    #[arg(default_value_t = 1500)]
    interval_mil: u64
}

async fn req_top_chara(
    args: Args,
    mongo_client: MongoClient,
    http_client: HttpClient
) -> anyhow::Result<()> {
    
    let db = mongo_client.database(&env::var("POOL_DB")?);
    let collection = db.collection::<Value>(&env::var("CHARA_CL")?);
    
    let base_url = env::var("BASE_URL")?;

    let interval = Duration::from_millis(args.interval_mil);
    let url = format!("{base_url}/top/characters");
    let mut page = 0;

    loop {
        page += 1;

        let url = paged_url(&url, page);
        let (data, pagination) = request(http_client.clone(), &url).await?;

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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();
    dotenvy::dotenv()?;
    let args = Args::parse();

    let mongo_uri = env::var("MONGO_URI")?;
    let mongo_client = MongoClient::with_uri_str(mongo_uri).await?;

    let http_client = HttpClient::new();

    req_top_chara(args, mongo_client, http_client).await
}
