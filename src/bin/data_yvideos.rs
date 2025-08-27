use std::{env, time::{SystemTime, UNIX_EPOCH}, vec};
use clap::Parser;
use futures::TryStreamExt;
use mongodb::{bson::doc, Client as MongoClient};
use tokio::fs;
use tracing::info;
use kenja_tools::{data::insert_batch, documents::{
        anime_search::{
            FlatDocument, ItemType, Parent
        }, 
        anime_src::{AnimeSrc, VideoSrc, YVideo}
    }};

#[derive(Parser)]
#[command(version)]
struct Args {
    #[arg(long, default_value_t = 100)]
    batch_size: usize,
    #[arg(long)]
    list: String
}

async fn yvideos(args: Args, mongo_client: MongoClient) -> anyhow::Result<()> {
    let json = fs::read_to_string(&args.list).await?;
    let src_list = serde_json::from_str::<Vec<i64>>(&json)?;

    let src_db = mongo_client.database(&env::var("DATA_SRC_DB")?);
    let dst_db = mongo_client.database(&env::var("DATA_DST_DB")?);

    info!("obtaining data. this will take a while.");
    let anime_cl = src_db.collection::<AnimeSrc>(&env::var("DATA_SRC_ANI_CL")?);
    let anime_list = anime_cl.find(doc! {}).await?
        .try_collect::<Vec<AnimeSrc>>().await?;

    let video_cl = src_db.collection::<VideoSrc>(&env::var("DATA_SRC_VIDEOS_CL")?);
    let mut video_list = video_cl.find(doc! {}).await?
        .try_collect::<Vec<VideoSrc>>().await?;

    let dst_cl = dst_db.collection::<FlatDocument>(&env::var("DATA_DST_CL")?);

    let mut batch = vec![];
    for anime_id in src_list {
        let Some(anime) = anime_list.iter().find(|a| a.mal_id == anime_id) else {
            continue;
        };

        let Some(idx) = video_list.iter().position(|v| v.mal_id == anime_id) else {
            continue;
        };
        let video_src = video_list.remove(idx);

        let parent = Parent{
            name: anime.title.clone(),
            name_japanese: anime.title_japanese.clone()
        };

        let updated_at = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as u64;

        for videos in video_src.videos {
            for promo in videos.promo {
                let unique = match promo.trailer {
                    Some(YVideo{youtube_id: Some(u)}) => u,
                    _ => continue 
                };

                batch.push(FlatDocument::new_yvideo(
                    updated_at, 
                    ItemType::YVideo, 
                    unique,
                    parent.clone(), 
                ));
            }
        }

        if batch.len() > args.batch_size {
            insert_batch(&dst_cl, &mut batch).await?;
        }
    }

    if batch.len() > 0 {
        insert_batch(&dst_cl, &mut batch).await?;
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

    yvideos(args, mongo_client).await?;
    
    Ok(())
}
