use std::{env, time::Duration};
use clap::Parser;
use mongodb::{bson::{Bson, doc}, Client as MongoClient};
use reqwest::Client as HttpClient;
use serde_json::Value;
use tokio::time;
use tracing::{info, warn};

#[derive(Parser)]
#[command(version)]
struct Args {
    #[arg(long, default_value_t = 1500)]
    interval_mil: u64,
    #[arg(long, default_value_t = 5000)]
    timeout_mil: u64
}

async fn req_staff(
    args: Args,
    mongo_client: MongoClient,
    http_client: HttpClient
) -> anyhow::Result<()> {
    let db = mongo_client.database(&env::var("POOL_DB")?);
    let src_cl = db.collection::<Value>(&env::var("ANI_CL")?);
    let staff_cl = db.collection::<Value>(&env::var("STAFF_CL")?);
    
    let base_url = env::var("BASE_URL")?;

    let interval = Duration::from_millis(args.interval_mil);
    let timeout = Duration::from_millis(args.timeout_mil);

    let list = src_cl.distinct("mal_id", doc! {}).await?;
    let total = list.len();

    for (i, bson) in list.iter().enumerate() {
        if let Bson::Int64(mal_id) = bson {
            info!("{i}/{total}");
        } else {
            warn!("skipping unexpected value {i}/{total} {bson}");
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
    let mong_client = MongoClient::with_uri_str(mongo_uri).await?;

    let http_client = HttpClient::new();


    Ok(())
}