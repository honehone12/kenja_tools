use std::{env, time::{SystemTime, UNIX_EPOCH}, vec};
use clap::Parser;
use futures::TryStreamExt;
use mongodb::{bson::doc, Client as MongoClient};
use tokio::fs;
use tracing::info;
use kenja_tools::{
    data::{ImgRoots, create_new_img, insert_batch}, 
    documents::{
        anime_search::{FlatDocument, ItemType}, 
        anime_src::{AnimeSrc, ImageUrls, ImgExSrc}
    }
};

#[derive(Parser)]
#[command(version)]
struct Args {
    #[arg(long, default_value_t = 100)]
    batch_size: usize,
    #[arg(long)]
    list: String
}

async fn pics(args: Args, mongo_client: MongoClient) -> anyhow::Result<()> {
    let json = fs::read_to_string(&args.list).await?;
    let src_list = serde_json::from_str::<Vec<i64>>(&json)?;

    let src_db = mongo_client.database(&env::var("DATA_SRC_DB")?);
    let dst_db = mongo_client.database(&env::var("DATA_DST_DB")?);

    info!("obtaining data. this will take a while.");
    let anime_cl = src_db.collection::<AnimeSrc>(&env::var("DATA_SRC_ANI_CL")?);
    let anime_list = anime_cl.find(doc! {}).await?.try_collect::<Vec<AnimeSrc>>().await?;

    let pic_cl = src_db.collection::<ImgExSrc>(&env::var("DATA_SRC_PICS_CL")?);
    let mut pic_list = pic_cl.find(doc! {}).await?.try_collect::<Vec<ImgExSrc>>().await?;

    let dst_cl = dst_db.collection::<FlatDocument>(&env::var("DATA_DST_CL")?);

    let img_roots = ImgRoots{
        raw_img_root: &env::var("RAW_IMG_ROOT")?,
        exist_img_root: &env::var("EXIST_IMG_ROOT")?,
        new_img_root: &env::var("NEW_IMG_ROOT")?,
    };

    let mut batch = vec![];
    for anime_id in src_list {
        let Some(anime) = anime_list.iter().find(|a| a.mal_id == anime_id) else {
            continue;
        };

        let Some(idx) = pic_list.iter().position(|v| v.mal_id == anime_id) else {
            continue;
        };
        let pic_src = pic_list.remove(idx);

        let updated_at = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as u64;

        for imgs in pic_src.pictures {
            let img_url = match imgs.jpg {
                Some(ImageUrls{image_url: Some(s)}) => s,
                _ => continue
            };

            let Some(img) = create_new_img(&img_roots, &img_url, ItemType::Anime).await? else {
                continue;
            };

            batch.push(FlatDocument::new_anime(
                updated_at,
                img,
                anime.url.clone(),
                anime.title.clone(),
                anime.title_english.clone(),
                anime.title_japanese.clone(),
            ));
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

    pics(args, mongo_client).await?;
    
    Ok(())
}
