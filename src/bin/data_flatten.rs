use std::{env, time::{SystemTime, UNIX_EPOCH}, vec};
use clap::Parser;
use anyhow::bail;
use chrono::NaiveDate;
use futures::TryStreamExt;
use mongodb::{bson::doc, Client as MongoClient};
use tokio::fs;
use tracing::info;
use kenja_tools::{
    data::{create_new_img, insert_batch, is_expected_media_type, is_hentai_str}, 
    documents::{
        anime_search::{
            FlatDocument, ItemType, OfficialSiteDocument, Parent
        }, 
        anime_src::{
            AniCharaBridge, AnimeSrc, CharacterSrc, ImageUrls, Images, LinkSrc,
        }
    }
};

#[derive(Parser)]
#[command(version)]
struct Args {
    #[arg(long, default_value_t = 100)]
    batch_size: usize,
    #[arg(long, default_value_t = 1965)]
    oldest: i32,
    #[arg(long, default_value_t = 10)]
    min_anime_likes: u64,
    #[arg(long, default_value_t = u64::MAX)]
    max_anime_likes: u64,
    #[arg(long, default_value_t = 5)]
    min_chara_likes: u64,
    #[arg(long, default_value_t = u64::MAX)]
    max_chara_likes: u64,
    #[arg(long)]
    new_img: bool
}

async fn flatten(args: Args, mongo_client: MongoClient) 
-> anyhow::Result<()> {
    let raw_img_root = env::var("RAW_IMG_ROOT")?;
    let exist_img_root = env::var("EXIST_IMG_ROOT")?;
    let new_img_root = env::var("NEW_IMG_ROOT")?;

    let src_db = mongo_client.database(&env::var("DATA_SRC_DB")?);
    let dst_db = mongo_client.database(&env::var("DATA_DST_DB")?);

    let ani_cl = src_db.collection::<AnimeSrc>(&env::var("DATA_SRC_ANI_CL")?);
    let ani_chara_cl = src_db.collection::<AniCharaBridge>(&env::var("DATA_SRC_ANI_CHARA_CL")?);
    let chara_cl = src_db.collection::<CharacterSrc>(&env::var("DATA_SRC_CHARA_CL")?);
    let links_cl = src_db.collection::<LinkSrc>(&env::var("DATA_SRC_LINKS_CL")?);

    let flat_cl = dst_db.collection::<FlatDocument>(&env::var("DATA_DST_CL")?);
    let official_cl = dst_db.collection::<OfficialSiteDocument>(&env::var("DATA_DST_OFFICIAL_CL")?);

    info!("obtaining data. this will take a while.");
    let mut ani_list = ani_cl.find(doc! {}).await?.try_collect::<Vec<AnimeSrc>>().await?;
    ani_list.sort_unstable_by_key(|d| d.mal_id);
    
    let mut ani_chara_list = ani_chara_cl.find(doc! {}).await?.try_collect::<Vec<AniCharaBridge>>().await?;
    let mut chara_list = chara_cl.find(doc! {}).await?.try_collect::<Vec<CharacterSrc>>().await?;
    let mut links_list = links_cl.find(doc! {}).await?.try_collect::<Vec<LinkSrc>>().await?;

    let chrono_fmt = "%Y-%m-%dT%H:%M:%S%z";
    let Some(oldest) = NaiveDate::from_yo_opt(args.oldest, 1) else {
        bail!("could not find a day on the calendar");
    };

    info!("start flattening");
    let mut batch = vec![];
    let mut official_batch = vec![];
    let mut inserted_chara_list = vec![];
    let mut inserted_ani_list = vec![];
    for anime in ani_list {
        match anime.aired.from {
            Some(s) => {
                let date = NaiveDate::parse_from_str(&s, &chrono_fmt)?;
                if date < oldest {
                    continue;
                }
            }
            None => continue
        };

        match anime.media_type {
            Some(s) if is_expected_media_type(&s) => (), 
            _ => continue
        };

        match anime.rating {
            Some(s) if !is_hentai_str(&s) => (),
            _ => continue
        }

        if anime.favorites < args.min_anime_likes 
            || anime.favorites > args.max_anime_likes 
        {
            continue;
        }

        if anime.synopsis.is_none_or(|s| s.is_empty()) {
            continue;
        }
          
        let img = match anime.images {
            Some(Images{jpg: Some(ImageUrls{image_url: Some(s)})}) => {
                if args.new_img {
                    match create_new_img(
                        &raw_img_root,
                        &exist_img_root, 
                        &new_img_root,
                        &s,
                        ItemType::Anime
                    ).await? {
                        Some(s) => s,
                        None => continue
                    }
                } else {
                    s
                }
            }
            _ => continue
        };

        let official = match links_list.iter().position(|l| l.mal_id == anime.mal_id) {
            Some(idx) => {
                let mut links = links_list.remove(idx);
                match links.links.iter().position(|l| l.name == "Official Site") {
                    Some(idx) => {
                        let link = links.links.remove(idx);
                        Some(link.url)
                    }
                    None => None
                }
            }
            None => None
        };

        let updated_at = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as u64;

        batch.push(FlatDocument::new_anime(
            updated_at,
            img,
            anime.url,
            anime.title.clone(),
            anime.title_english,
            anime.title_japanese.clone(),
        ));

        if let Some(url) = official {
            official_batch.push(OfficialSiteDocument{
                mal_id: anime.mal_id,
                url,
                parent: Parent{
                    name: anime.title.clone(),
                    name_japanese: anime.title_japanese.clone()
                },
            });
        }

        inserted_ani_list.push(anime.mal_id);

        if let Some(idx) = ani_chara_list.iter().position(|b| b.mal_id == anime.mal_id) {
            let bridge = ani_chara_list.remove(idx);
            for cc in bridge.characters {
                let chara = match chara_list.iter().position(|c| c.mal_id == cc.character.mal_id) {
                    Some(idx) => chara_list.remove(idx),
                    None => continue
                };

                if inserted_chara_list.contains(&chara.mal_id) {
                    continue;
                }

                if chara.favorites < args.min_chara_likes 
                    || chara.favorites > args.max_chara_likes 
                {
                    continue;    
                }

                if chara.about.is_none_or(|a| a.is_empty()) {
                    continue;
                } 

                let img = match chara.images {
                    Some(Images{jpg: Some(ImageUrls{image_url: Some(s)})}) => {
                        if args.new_img {
                            match create_new_img(
                                &raw_img_root,
                                &exist_img_root, 
                                &new_img_root,
                                &s,
                                ItemType::Character
                            ).await? {
                                Some(s) => s,
                                None => continue
                            }
                        } else {
                            s
                        }
                    }
                    _ => continue
                };

                batch.push(FlatDocument::new_character(
                    updated_at,
                    img,
                    chara.url,
                    chara.name,
                    chara.name_kanji,
                    Parent{
                        name: anime.title.clone(),
                        name_japanese: anime.title_japanese.clone()
                    }
                ));

                inserted_chara_list.push(chara.mal_id);
            }
        }

        if batch.len() >= args.batch_size {
            insert_batch(&flat_cl, &mut batch).await?    
        }
        if official_batch.len() >= args.batch_size {
            insert_batch(&official_cl, &mut official_batch).await?    
        }
    }
    
    if !batch.is_empty() {
        insert_batch(&flat_cl, &mut batch).await?  
    }
    if !official_batch.is_empty() {
        insert_batch(&official_cl, &mut official_batch).await?  
    }

    let ani_list_json = serde_json::to_string(&inserted_ani_list)?;
    fs::write("inserted_anime_list.json", ani_list_json).await?;

    let chara_list_json = serde_json::to_string(&inserted_chara_list)?;
    fs::write("inserted_chara_list.json", chara_list_json).await?;

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

    flatten(args, mongo_client).await?;
    
    Ok(())
}
