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
    #[arg(long, default_value_t = 3)]
    anime_likes: u64,
    #[arg(long, default_value_t = 3)]
    chara_likes: u64,
    #[arg(long, value_enum)]
    rating: Rating
}

async fn flatten(args: Args, mongo_client: MongoClient) 
-> anyhow::Result<()> {
    let src_db = mongo_client.database(&env::var("POOL_DB")?);
    let dst_db = mongo_client.database(&env::var("SEARCH_DB")?);

    let ani = args.rating.as_suffix(&env::var("ANI_CL")?);
    let ani_cl = src_db.collection::<AnimeDocument>(&ani);
    let ani_chara_cl = src_db.collection::<AniCharaBridge>(&env::var("ANI_CHARA_CL")?);
    let chara_cl = src_db.collection::<CharacterDocument>(&env::var("CHARA_CL")?);
    let staff_cl = src_db.collection::<StaffDocument>(&env::var("STAFF_CL")?);
    let flat_cl = dst_db.collection::<FlatDocument>(&env::var("FLAT_CL")?);

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

        let synopsis = match anime.synopsis {
            Some(s) if !s.is_empty() => s,
            _ => continue
        };

        let img = match anime.images {
            Some(Images{jpg: Some(ImageUrls{image_url: Some(s)})}) => s,
            _ => continue
        };

        if anime.favorites < args.anime_likes {
            continue;
        }

        let Some(idx) = staff_list.iter().position(|s| s.mal_id == anime.mal_id) else {
            continue;
        };
        let staff = staff_list.remove(idx);
        if staff.staffs.is_empty() {
            continue;
        }
        let flat_staff = staff.staffs.iter()
            .map(|s| s.person.name.replace(',', ""))
            .collect::<Vec<String>>().join(". ");

        let studios = anime.studios.iter().map(|s| s.name.clone())
            .collect::<Vec<String>>();

        let res = flat_cl.insert_one(FlatDocument{
            item_type: ItemType32::Anime,
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
            for cc in bridge.characters.iter() {
                let Some(idx) = chara_list.iter_mut()
                    .position(|c| c.mal_id == cc.character.mal_id)
                else {
                    continue;
                };

                let chara = chara_list.remove(idx);
                if inserted_chara_list.contains(&chara.mal_id) {
                    continue;
                }
                
                let about = match chara.about {
                    Some(s) if !s.is_empty() => s,
                    _ => continue
                };

                let img = match chara.images {
                    Some(Images{jpg: Some(ImageUrls{image_url: Some(s)})}) => s,
                    _ => continue
                };

                if chara.favorites < args.chara_likes {
                    continue;
                }

                if cc.voice_actors.is_empty() {
                    continue;
                }
                let flat_voice_actor = cc.voice_actors.iter()
                    .map(|v| v.person.name.replace(',', ""))
                    .collect::<Vec<String>>().join(" . ");

                batch.push(FlatDocument{
                    item_type: ItemType32::Character,
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
