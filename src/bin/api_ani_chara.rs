use std::{env, time::Duration};
use tokio::time;
use mongodb::{bson::{doc, Bson}, Client as MongoClient};
use reqwest::Client as HttpClient;
use clap::Parser;
use serde_json::Value;
use tracing::{info, warn};
use kenja_tools::{documents::anime_raw::AnimeCharacters, api::request};

#[derive(Parser)]
#[command(version)]
struct Args {
    #[arg(long, default_value_t = 1500)]
    interval_mil: u64,
    #[arg(long, default_value_t = 10000)]
    timeout_mil: u64
}

async fn req_ani_chara(
    args: Args,
    mongo_client: MongoClient, 
    http_client: HttpClient
) -> anyhow::Result<()> {
    let db = mongo_client.database(&env::var("POOL_DB")?);
    let source = db.collection::<Value>(&env::var("ANI_CL")?);
    let collection = db.collection::<AnimeCharacters>(&env::var("ANI_CHARA_CL")?);

    let base_url = env::var("BASE_URL")?;

    let interval = Duration::from_millis(args.interval_mil);
    let timeout = Duration::from_millis(args.timeout_mil);

    let list = source.distinct("mal_id", doc! {}).await?;
    let total = list.len();

    for (i, bson) in list.iter().enumerate() {
        if let Bson::Int64(mal_id) = bson {
            info!("{i}/{total}");
            let url = format!("{base_url}/anime/{mal_id}/characters");
            match request(&http_client, timeout, &url).await {
                Err(e) => warn!("request failed. {e}. skipping"),
                Ok((data, _)) => {
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
                }
            }
        } else {
            warn!("skipping unexpected value {i}/{total}:{bson}");
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
    
    req_ani_chara(args, mongo_client, http_client).await
}
