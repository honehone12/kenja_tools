use std::{
    env, 
    path::PathBuf, 
    str::FromStr, 
    time::{SystemTime, UNIX_EPOCH}, 
    vec
};
use tokio::fs;
use clap::Parser;
use anyhow::bail;
use url::Url;
use chrono::NaiveDate;
use futures::TryStreamExt;
use mongodb::{bson::doc, Client as MongoClient};
use tracing::{info, warn};
use kenja_tools::{
    documents::{
        anime::{
            AniCharaBridge, 
            AnimeDocument, 
            CharacterDocument, 
            ImageUrls, 
            Images, 
            StaffDocument 
        }, 
        anime_search::{
            FlatDocument, 
            ItemType32, 
            Parent
        }, 
        Rating
    }, is_expected_media_type
};

#[derive(Parser)]
#[command(version)]
struct Args {
    #[arg(long, default_value_t = 1965)]
    oldest: i32,
    #[arg(long, default_value_t = 4)]
    anime_likes: u64,
    #[arg(long, default_value_t = 6)]
    chara_likes: u64,
    #[arg(long, value_enum)]
    rating: Rating,
    #[arg(long)]
    hash_img: bool
}

async fn create_new_img(
    raw_img_root: &str,
    merged_img_root: &str, 
    new_img_root: &str,
    unique_url: &str,
    img_url: &str,
) -> anyhow::Result<Option<String>> {
    
    let u = Url::parse(img_url)?;
    let mut p = u.path().to_string();
    p.remove(0);
    let path = PathBuf::from_str(raw_img_root)?.join(p);

    if !fs::try_exists(&path).await? {
        warn!("file {path:?} does not exits");
        return Ok(None);
    }
    
    let hash = blake3::hash(unique_url.as_bytes());
    let hash = hash.as_bytes();
    let hex = hex::encode(&hash[..16]);

    let new_file = format!("{hex}.jpg");
    let new_path = PathBuf::from_str(merged_img_root)?.join(&new_file);
    if fs::try_exists(new_path).await? {
        return Ok(Some(new_file))
    }
    
    let new_path = PathBuf::from_str(new_img_root)?.join(&new_file);
    fs::copy(path, new_path).await?;

    Ok(Some(new_file))
}

async fn flatten(args: Args, mongo_client: MongoClient) 
-> anyhow::Result<()> {
    let raw_img_root = env::var("RAW_IMG_ROOT")?;
    let merged_img_root = env::var("MERGED_IMG_ROOT")?;
    let new_img_root = env::var("NEW_IMG_ROOT")?;

    let src_db = mongo_client.database(&env::var("MERGED_DB")?);
    let dst_db = mongo_client.database(&env::var("SEARCH_DB")?);

    let ani_cl = src_db.collection::<AnimeDocument>(&env::var("MERGED_ANI_CL")?);
    let ani_chara_cl = src_db.collection::<AniCharaBridge>(&env::var("MERGED_ANI_CHARA_CL")?);
    let chara_cl = src_db.collection::<CharacterDocument>(&env::var("MERGED_CHARA_CL")?);
    let staff_cl = src_db.collection::<StaffDocument>(&env::var("MERGED_STAFF_CL")?);
    
    let mut flat_cl = env::var("FLAT_CL")?;
    if matches!(args.rating, Rating::Hentai) {
        flat_cl = args.rating.as_suffix(&flat_cl);
    }
    let flat_cl = dst_db.collection::<FlatDocument>(&flat_cl);

    let mut ani_list = ani_cl.find(doc! {}).await?
        .try_collect::<Vec<AnimeDocument>>().await?;
    ani_list.sort_unstable_by_key(|d| d.mal_id);
    info!("{} anime documents", ani_list.len());
    
    let mut ani_chara_list = ani_chara_cl.find(doc! {}).await?
        .try_collect::<Vec<AniCharaBridge>>().await?;
    info!("{} anime-chara bridges", ani_chara_list.len());

    let mut chara_list = chara_cl.find(doc! {}).await?
        .try_collect::<Vec<CharacterDocument>>().await?;
    info!("{} character documets", chara_list.len());

    let mut staff_list = staff_cl.find(doc! {}).await?
        .try_collect::<Vec<StaffDocument>>().await?;
    info!("{} staff documets", staff_list.len());

    let allow_empty_text = matches!(args.rating, Rating::Hentai);
    let chrono_fmt = "%Y-%m-%dT%H:%M:%S%z";
    let Some(oldest) = NaiveDate::from_yo_opt(args.oldest, 1) else {
        bail!("could not find a day on the calendar");
    };

    info!("start flattening");
    let mut batch = vec![];
    let mut inserted_chara_list = vec![];
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
            Some(s) => {
                if !args.rating.match_str(&s) {
                    continue;
                }
            }
            None => continue
        }

        if anime.favorites < args.anime_likes {
            continue;
        }

        let synopsis = match anime.synopsis {
            Some(s) if !s.is_empty() => Some(s),
            _ => {
                if allow_empty_text {
                    None
                } else {
                    continue;
                }
            }
        };

        let staffs = match staff_list.iter().position(|s| s.mal_id == anime.mal_id) {
            Some(idx) => {
                let staffs = staff_list.remove(idx).staffs;
                if staffs.is_empty() {
                    None
                } else {
                    Some(staffs)
                }
            },
            None => {
                if allow_empty_text {
                    None
                } else {
                    continue;
                }
            }
        };

        let flat_staff = match staffs {
            Some(stfs) => Some(stfs.iter()
                .map(|s| s.person.name.replace(',', "").replace('.', ""))
                .collect::<Vec<String>>().join(". ")),
            None => {
                if allow_empty_text {
                    None
                } else {
                    continue;
                }
            }
        };

        let studios = anime.studios.iter().map(|s| s.name.clone())
            .collect::<Vec<String>>();

        let img = match anime.images {
            Some(Images{jpg: Some(ImageUrls{image_url: Some(s)})}) => {
                if args.hash_img {
                    let Some(img) = create_new_img(
                        &raw_img_root,
                        &merged_img_root, 
                        &new_img_root,
                        &anime.url, 
                        &s
                    ).await? else {
                        continue;
                    };
                    img
                } else {
                    s
                }
            }
            _ => continue
        };

        let item_type = match synopsis {
            Some(_) => ItemType32::Anime,
            None => ItemType32::AnimeImgOnly
        };

        let updated_at = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as u64;

        let res = flat_cl.insert_one(FlatDocument{
            updated_at,
            item_type,
            rating: args.rating.to_32(),
            url: anime.url,
            img,
            parent: None,
            name: anime.title.clone(),
            name_english: anime.title_english,
            name_japanese: anime.title_japanese.clone(),
            aliases: anime.title_synonyms,
            studios,
            staff: flat_staff,
            description: synopsis,
        }).await?;

        let Some(parent_id) = res.inserted_id.as_object_id() else {
            bail!("inserted object id is empty")
        };

        info!("inserted a item");

        if let Some(idx) = ani_chara_list.iter_mut()
            .position(|b| b.mal_id == anime.mal_id)
        {
            let bridge = ani_chara_list.remove(idx);
            for cc in bridge.characters {
                let Some(idx) = chara_list.iter_mut()
                    .position(|c| c.mal_id == cc.character.mal_id)
                else {
                    continue;
                };

                let chara = chara_list.remove(idx);

                if inserted_chara_list.contains(&chara.mal_id) {
                    continue;
                }
                
                if chara.favorites < args.chara_likes {
                    continue;
                }

                let about = match chara.about {
                    Some(s) if !s.is_empty() => Some(s),
                    _ => {
                        if allow_empty_text {
                            None
                        } else {
                            continue;
                        }
                    }
                };

                let voice_actors = if cc.voice_actors.is_empty() {
                    if allow_empty_text {
                        None
                    } else {
                        continue;
                    }
                } else {
                    Some(cc.voice_actors)
                };

                let flat_voice_actor = match voice_actors {
                    Some(vos) => {
                        Some(vos.iter()
                            .map(|v| v.person.name.replace(',', "").replace('.', ""))
                            .collect::<Vec<String>>().join(". "))
                    }
                    None => {
                        if allow_empty_text {
                            None
                        } else {
                            continue;
                        }
                    }
                };

                let img = match chara.images {
                    Some(Images{jpg: Some(ImageUrls{image_url: Some(s)})}) => {
                        if args.hash_img {
                            let Some(img) = create_new_img(
                                &raw_img_root,
                                &merged_img_root, 
                                &new_img_root,
                                &chara.url, 
                                &s
                            ).await? else {
                                continue;
                            };
                            img
                        } else {
                            s
                        }
                    }
                    _ => continue
                };

                let item_type = match about {
                    Some(_) => ItemType32::Character,
                    None => ItemType32::CharacterImgOnly
                };

                batch.push(FlatDocument{
                    updated_at,
                    item_type,
                    rating: args.rating.to_32(),
                    url: chara.url,
                    img,
                    parent: Some(Parent{
                        id: parent_id,
                        name: anime.title.clone(),
                        name_japanese: anime.title_japanese.clone(),
                    }),
                    name: chara.name,
                    name_english: None,
                    name_japanese: chara.name_kanji,
                    aliases: chara.nicknames,
                    studios: vec![],
                    staff: flat_voice_actor,
                    description: about,
                });
                inserted_chara_list.push(chara.mal_id);
            }
        }

        if batch.len() >= 100 {
            let result = flat_cl.insert_many(&batch).await?;
            info!("inserted {}items", result.inserted_ids.len());
            batch.clear();    
        }
    }
    
    if !batch.is_empty() {
        let result = flat_cl.insert_many(&batch).await?;
        info!("inserted {}items", result.inserted_ids.len());
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

    flatten(args, mongo_client).await?;
    
    Ok(())
}
