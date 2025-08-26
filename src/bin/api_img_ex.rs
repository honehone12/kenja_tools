use std::{env, time::Duration};
use futures::TryStreamExt;
use tokio::{fs, time};
use mongodb::{bson::doc, Client as MongoClient};
use reqwest::{Client as HttpClient, Url};
use clap::Parser;
use kenja_tools::{api::request_img, documents::anime_src::{ImageUrls, ImgExSrc}};
use tracing::{info, warn};

#[derive(Parser)]
#[command(version)]
struct Args {
    #[arg(long, default_value_t = 1500)]
    interval_mil: u64,
    #[arg(long, default_value_t = 10000)]
    timeout_mil: u64
}

async fn img_ex(
    args: Args, 
    mongo_client: MongoClient,
    http_client: HttpClient
) -> anyhow::Result<()> {
    let db = mongo_client.database(&env::var("API_SRC_DB")?);
    let colle = db.collection::<ImgExSrc>(&env::var("API_SRC_CL")?);

    info!("obtaining documents...");
    let img_ex_list = colle.find(doc! {}).await?.try_collect::<Vec<ImgExSrc>>().await?;
    let list_total = img_ex_list.len();
    
    let interval = Duration::from_millis(args.interval_mil);
    let timeout = Duration::from_millis(args.timeout_mil);

    let img_root = env::var("RAW_IMG_ROOT")?;

    let mut it = 0u32;
    let mut total = 0u32;
    for img_ex in img_ex_list {
        for imgs in img_ex.pictures {
            let img = match imgs.jpg {
                Some(ImageUrls{image_url: Some(s)}) => s,
                _ => continue
            };


            if img.contains("icon") {
                continue;
            }

            let url = Url::parse(&img)?;
            let file_name = format!("{img_root}{}", url.path());
            if fs::try_exists(&file_name).await? {
                total += 1;
                continue;
            }

            if let Err(e) = request_img(
                &http_client, 
                timeout,
                &img, 
                &file_name
            ).await {
                warn!("{e}");
            };

            total += 1;
            it += 1;
            info!("iteration {it} {total}/{list_total}"); 
            
            time::sleep(interval).await;
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

    img_ex(args, mongo_client, http_client).await
}
