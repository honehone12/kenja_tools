use std::{env, time::Duration};
use reqwest::Client as HttpClient;
use mongodb::Client as MongoClient;
use clap::Parser;
use serde_json::Value;
use tracing::info;
use kenja_tools::{api::request_pages, documents::anime_raw::Season};

#[derive(Parser)]
#[command(version)]
struct Args {
    #[arg(long)]
    year: String,
    #[arg(long)]
    season: Season,
    #[arg(long, default_value_t = 1500)]
    interval_mil: u64,
    #[arg(long, default_value_t = 10000)]
    timeout_mil: u64
}

async fn req_quarter_list(
    args: Args,
    mongo_client: MongoClient,
    http_client: HttpClient
) -> anyhow::Result<()> {
    let db = mongo_client.database(&env::var("POOL_DB")?);
    let collection = db.collection::<Value>(&env::var("ANI_CL")?);

    let base_url = env::var("BASE_URL")?;

    let interval = Duration::from_millis(args.interval_mil);
    let timeout = Duration::from_millis(args.timeout_mil);

    let url = format!("{base_url}/seasons/{}/{}", args.year, args.season);
    let list = request_pages(&http_client, interval, timeout, &url).await?;

    if list.is_empty() {
        info!("list is empty");
        return Ok(());
    }

    let res = collection.insert_many(list).await?;
    info!("inserted {}items", res.inserted_ids.len());

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

    req_quarter_list(args, mongo_client, http_client).await
}
