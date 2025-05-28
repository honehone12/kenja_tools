use std::env;
use tokio::fs;
use clap::Parser;
use mongodb::Client as MongoClient;
use anyhow::bail;
use kenja_tools::documents::id::Ids;

#[derive(Parser)]
#[command(version)]
struct Args {
    #[arg(long)]
    collection: String,
    #[arg(long)]
    pattern: String
}

async fn clean_img(args: Args, mongo_client: MongoClient) -> anyhow::Result<()> {    
    let db = mongo_client.database(&env::var("SEARCH_DB")?);
    let colle = db.collection::<Ids>(&args.collection);
    
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();
    dotenvy::dotenv()?;
    let args = Args::parse();

    let mongo_uri = env::var("MONGO_URI")?;
    let mongo_client = MongoClient::with_uri_str(mongo_uri).await?;

    

    clean_img(args, mongo_client).await
}
