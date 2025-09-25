use clap::Parser;
use futures::TryStreamExt;
use kenja_tools::{api::request, documents::anime_src::AniCharaBridge};
use mongodb::{bson::doc, Client as MongoClient};
use reqwest::Client as HttpClient;
use serde_json::Value;
use std::{env, time::Duration};
use tokio::time;
use tracing::{info, warn};

#[derive(Parser)]
#[command(version)]
struct Args {
    #[arg(long, default_value_t = 1500)]
    interval_mil: u64,
    #[arg(long, default_value_t = 10000)]
    timeout_mil: u64,
}

async fn req_chara(
    args: Args,
    mongo_client: MongoClient,
    http_client: HttpClient,
) -> anyhow::Result<()> {
    let src_db = mongo_client.database(&env::var("API_SRC_DB")?);
    let src_cl = src_db.collection::<AniCharaBridge>(&env::var("API_SRC_CL")?);

    let dst_db = mongo_client.database(&env::var("API_DST_DB")?);
    let dst_cl = dst_db.collection::<Value>(&env::var("API_DST_CL")?);

    let interval = Duration::from_millis(args.interval_mil);
    let timeout = Duration::from_millis(args.timeout_mil);

    let base_url = env::var("BASE_API_URL")?;

    let done = dst_cl
        .distinct("mal_id", doc! {})
        .await?
        .iter()
        .filter_map(|bson| bson.as_i64())
        .collect::<Vec<i64>>();

    let list = src_cl
        .find(doc! {})
        .await?
        .try_collect::<Vec<AniCharaBridge>>()
        .await?;
    let mut total = 0;

    // this stream should be cached
    for bridge in list {
        let mut batch = vec![];

        for chara_cast in bridge.characters {
            if done.contains(&chara_cast.character.mal_id) {
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

        total += 1;

        if !batch.is_empty() {
            let res = dst_cl.insert_many(&batch).await?;
            info!(
                "iteration {total}. inserted {} items",
                res.inserted_ids.len()
            );
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
