use std::{env, time::Duration};
use futures::TryStreamExt;
use tokio::time;
use mongodb::{bson::doc, Client as MongoClient};
use reqwest::Client as HttpClient;
use clap::Parser;
use serde_json::Value;
use tracing::{info, warn};
use kenja_tools::{documents::anime::AniCharaBridge, api::request};

#[derive(Parser)]
#[command(version)]
struct Args {
    #[arg(long, default_value_t = 1500)]
    interval_mil: u64,
    #[arg(long, default_value_t = 10000)]
    timeout_mil: u64
}

async fn req_chara(
    args: Args,
    mongo_client: MongoClient,
    http_client: HttpClient
) -> anyhow::Result<()> {
    let search_db = mongo_client.database(&env::var("SEARCH_DB")?);
    let flat_cl = search_db.collection::<Value>(&env::var("FLAT_CL")?);

    let db = mongo_client.database(&env::var("SEASON_DB")?);
    let ani_chara_cl = db.collection::<AniCharaBridge>(&env::var("SEASON_ANI_CHARA_CL")?);
    let chara_cl = db.collection::<Value>(&env::var("SEASON_CHARA_CL")?);
    
    let interval = Duration::from_millis(args.interval_mil);
    let timeout = Duration::from_millis(args.timeout_mil);

    let base_url = env::var("BASE_API_URL")?;
    info!("getting flat url list");
    let flat_url_list = flat_cl.distinct("url", doc! {}).await?.iter()
        .filter_map(|bson| bson.as_str().map(|str| str.to_string()))
        .collect::<Vec<String>>();
    let mut ani_chara_stream = ani_chara_cl.find(doc! {}).await?;
    let mut batch = vec![];

    while let Some(bridge) = ani_chara_stream.try_next().await? {
        for chara_cast in bridge.characters {
            if flat_url_list.iter()
                .find(|url| **url == chara_cast.character.url).is_some() 
            {
                continue;
            }

            let url = format!("{base_url}/characters/{}", chara_cast.character.mal_id);
            match request(&http_client, timeout, &url).await {
                Err(e) => warn!("request failed. {e}. skipping"),
                Ok((mut data, _)) => {
                    batch.append(&mut data);
                }
            }

            time::sleep(interval).await;
        }

        if !batch.is_empty() {
            let res = chara_cl.insert_many(&batch).await?;
            info!("inserted {} items", res.inserted_ids.len());
            batch.clear();
        }
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
    
    req_chara(args, mongo_client, http_client).await
}
