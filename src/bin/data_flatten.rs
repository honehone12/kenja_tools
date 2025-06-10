use std::{env, vec};
use clap::Parser;
use anyhow::bail;
use chrono::NaiveDate;
use futures::TryStreamExt;
use mongodb::{bson::doc, Client as MongoClient};
use tracing::info;
use kenja_tools::{
    documents::{
        anime::{
            AniCharaBridge, AnimeDocument, CharacterDocument, 
        }, 
        anime_search::{
            FlatDocument, ItemType, Parent
        }, 
        Rating
    }, is_expected_media_type
};

#[derive(Parser)]
#[command(version)]
struct Args {
    #[arg(long, default_value_t = false)]
    include_empty: bool,
    #[arg(long, default_value_t = 1965)]
    oldest: i32,
    #[arg(long, default_value_t = 3)]
    anime_likes: u64,
    #[arg(long, default_value_t = 5)]
    chara_likes: u64,
    #[arg(long, value_enum)]
    rating: Rating
}

async fn flatten(args: Args, mongo_client: MongoClient) 
-> anyhow::Result<()> {
    let source_db = mongo_client.database(&env::var("POOL_DB")?);
    let dest_db = mongo_client.database(&env::var("SEARCH_DB")?);

    let ani = args.rating.as_suffix(&env::var("ANI_CL")?);
    let ani_colle = source_db.collection::<AnimeDocument>(&ani);
    let ani_chara = env::var("ANI_CHARA_CL")?;
    let ani_chara_colle = source_db.collection::<AniCharaBridge>(&ani_chara);
    let chara = env::var("CHARA_CL")?;
    let chara_colle = source_db.collection::<CharacterDocument>(&chara);
    let mut flat = args.rating.as_suffix(&env::var("FLAT_CL")?);
    if args.include_empty {
        flat.push_str("_null");
    }
    let flat_colle = dest_db.collection::<FlatDocument>(&flat);

    info!("obtaining {ani} documents...");
    let mut ani_list = ani_colle.find(doc! {}).await?
        .try_collect::<Vec<AnimeDocument>>().await?;
    ani_list.sort_unstable_by_key(|d| d.mal_id);
    info!("{} anime documents", ani_list.len());
    
    info!("obtaining {ani_chara} bridge...");
    let mut ani_chara_list = ani_chara_colle.find(doc! {}).await?
        .try_collect::<Vec<AniCharaBridge>>().await?;
    info!("{} anime-chara bridges", ani_chara_list.len());

    info!("obtaining {chara} documents...");
    let mut chara_list = chara_colle.find(doc! {}).await?
        .try_collect::<Vec<CharacterDocument>>().await?;
    info!("{} character documets", chara_list.len());

    let chrono_fmt = "%Y-%m-%dT%H:%M:%S%z";
    let Some(oldest) = NaiveDate::from_yo_opt(args.oldest, 1) else {
        bail!("could not find a day on the calendar");
    };

    info!("start flattening");
    let mut batch = vec![];
    let mut inserted_chara_list = vec![];
    for mut anime in ani_list {
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
            Some(s) => {
                if !is_expected_media_type(&s) {
                    continue;
                }
            } 
            None => continue
        };

        if !args.include_empty {
            match &anime.synopsis {
                Some(s) => {
                    if s.len() == 0 {
                        continue;
                    }
                }
                None => continue
            }
        }

        let img = match anime.images {
            Some(i) => {
                let Some(u) = i.jpg else {
                    continue
                };

                match u.image_url {
                    Some(url) => url,
                    None => continue
                }
            }
            None => continue
        };

        if anime.favorites < args.anime_likes {
            continue;
        }

        if let Some(s) = &mut anime.synopsis {
            s.retain(|c| {
                if c.is_whitespace() {
                    return c == ' ';
                }
                true
            });
        }

        let res = flat_colle.insert_one(FlatDocument{
            item_type: ItemType::Anime,
            url: anime.url,
            img,
            parent: None,
            name: anime.title.clone(),
            name_english: anime.title_english,
            name_japanese: anime.title_japanese.clone(),
            aliases: anime.title_synonyms,
            description: anime.synopsis,
        }).await?;

        let Some(parent_id) = res.inserted_id.as_object_id() else {
            bail!("inserted object id is empty")
        };

        info!("inserted a item");

        if let Some(idx) = ani_chara_list
            .iter_mut()
            .position(|b| b.mal_id == anime.mal_id)
        {
            let bridge = ani_chara_list.remove(idx);
            for cc in bridge.characters.iter() {
                let Some(idx) = chara_list
                    .iter_mut()
                    .position(|c| c.mal_id == cc.character.mal_id)
                else {
                    continue;
                };

                let mut chara = chara_list.remove(idx);
                if inserted_chara_list.contains(&chara.mal_id) {
                    continue;
                }
                
                if !args.include_empty {
                    match &chara.about {
                        Some(s) => {
                            if s.len() == 0 {
                                continue;
                            }
                        }
                        None => continue
                    }
                }

                let img = match chara.images {
                    Some(i) => {
                        let Some(u) = i.jpg else {
                            continue
                        };

                        match u.image_url {
                            Some(url) => url,
                            None => continue
                        }
                    }
                    None => continue
                };

                if chara.favorites < args.chara_likes {
                    continue;
                }

                if let Some(s) = &mut chara.about {
                    s.retain(|c| {
                        if c.is_whitespace() {
                            return c == ' ';
                        }
                        true
                    });
                }

                batch.push(FlatDocument{
                    item_type: ItemType::Character,
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
                    description: chara.about,
                });
                inserted_chara_list.push(chara.mal_id);
            }
        }

        if batch.len() >= 100 {
            let result = flat_colle.insert_many(&batch).await?;
            info!("inserted {}items", result.inserted_ids.len());
            batch.clear();    
        }
    }
    
    if !batch.is_empty() {
        let result = flat_colle.insert_many(&batch).await?;
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
