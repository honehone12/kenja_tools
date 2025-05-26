use std::env;
use futures::TryStreamExt;
use tokio::fs;
use mongodb::{bson::doc, Client as MongoClient};
use reqwest::Client as HttpClient;
use clap::Parser;
use kenja_tools::{api::request_img, documents::local::Img};
use tracing::info;

#[derive(Parser)]
#[command(version)]
struct Args {
    #[arg(long, default_value = "json")]
    json_path: String,
    #[arg(long)]
    img_path: String,
    #[arg(long)]
    collection: String,
}

async fn img(
    args: Args, 
    mongo_client: MongoClient,
    http_client: HttpClient
) -> anyhow::Result<()> {
    let img_file_path = format!("{}/img.json", args.json_path);
    let exists = match fs::try_exists(&img_file_path).await {
        Ok(f) => f,
        Err(_) => false
    };
    let mut done_list = if exists {
        let s = fs::read_to_string(&img_file_path).await?;
        serde_json::from_str::<Vec<Img>>(&s)?
    } else {
        vec![]
    };

    let db = mongo_client.database(&env::var("SEARCH_DB")?);
    let colle = db.collection::<Img>(&args.collection);
    info!("obtaining {} documents...", args.collection);
    let img_list = colle.find(doc! {}).await?.try_collect::<Vec<Img>>().await?;
    info!("{} img documents", img_list.len());

    for mut img in img_list {
        if done_list.iter().find(|i| i.item_id == img.item_id).is_some() {
            continue;
        }

        let path = request_img(http_client.clone(), &img.img, &args.img_path).await?;
        img.path = Some(path);
        done_list.push(img); 
    }

    let s = serde_json::to_string_pretty(&done_list)?;
    fs::write(img_file_path, s).await?;
    
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
