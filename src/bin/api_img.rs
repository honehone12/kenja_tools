use std::{env, time::Duration};
use futures::TryStreamExt;
use tokio::{fs, time};
use mongodb::{bson::doc, Client as MongoClient};
use reqwest::{Client as HttpClient, Url};
use clap::Parser;
use kenja_tools::{api::request_img, documents::local::Img};
use tracing::{info, warn};

#[derive(Parser)]
#[command(version)]
struct Args {
    #[arg(long, default_value_t = 100)]
    iteration: u32,
    #[arg(long)]
    img_path: String,
    #[arg(long)]
    collection: String,
    #[arg(long, default_value_t = 1500)]
    interval_mil: u64,
    #[arg(long, default_value_t = 5000)]
    timeout_mil: u64
}

async fn img(
    args: Args, 
    mongo_client: MongoClient,
    http_client: HttpClient
) -> anyhow::Result<()> {
    let db = mongo_client.database(&env::var("SEARCH_DB")?);
    let colle = db.collection::<Img>(&args.collection);
    info!("obtaining {} documents...", args.collection);
    let img_list = colle.find(doc! {}).await?.try_collect::<Vec<Img>>().await?;
    info!("{} img documents", img_list.len());
    
    let interval = Duration::from_millis(args.interval_mil);
    let timeout = Duration::from_millis(args.timeout_mil);

    let img_root = match args.img_path.strip_suffix('/') {
        Some(r) => r,
        None => &args.img_path 
    };    

    let mut it = 0;
    for img in img_list {
        let url = Url::parse(&img.img)?;
        let path = format!("{img_root}{}", url.path());
        if fs::try_exists(path).await? {
            continue;
        }

        if let Err(e) = request_img(
            http_client.clone(), 
            timeout,
            &img.img, 
            &args.img_path
        ).await {
            warn!("{e}");
            time::sleep(interval).await; 
            continue;
        };

        time::sleep(interval).await;
        it += 1;

        if it >= args.iteration {
            info!("quit on max iteration");
            break;
        }
        
        info!("iteration {it}"); 
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

    img(args, mongo_client, http_client).await
}
