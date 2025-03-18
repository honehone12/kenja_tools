use std::vec;
use anyhow::bail;
use chrono::{Datelike, NaiveDate};
use futures::TryStreamExt;
use mongodb::{bson::doc, Client as MongoClient};
use tracing::info;
use crate::{
    commands::{is_expected_media_type, Rating},
    documents::{
        anime::{
            AniCharaBridge, AnimeDocument, CharacterDocument, 
        }, 
        FlatDocument, ItemId, ItemType, Parent
    }
};

pub(crate) async fn flatten_main(rating: Rating, mongo_client: MongoClient) 
-> anyhow::Result<()> {

    let source_db = mongo_client.database("anime");
    let dest_db = mongo_client.database("anime_search");

    let ani_colle = source_db.collection::<AnimeDocument>(
        &format!("anime_{}", rating.to_string())
    );
    let ani_chara_colle = source_db.collection::<AniCharaBridge>("anime_chara");
    let chara_colle = source_db.collection::<CharacterDocument>("chara");
    let flat_colle = dest_db.collection::<FlatDocument>(&rating.to_string());

    info!("obtaining anime documents...");
    let mut ani_list = ani_colle
        .find(doc! {}).await?
        .try_collect::<Vec<AnimeDocument>>().await?;
    ani_list.sort_unstable_by_key(|d| d.mal_id);
    
    info!("obtaining anime-character bridge...");
    let mut ani_chara_list = ani_chara_colle
        .find(doc! {}).await?
        .try_collect::<Vec<AniCharaBridge>>().await?;

    info!("obtaining character documents...");
    let mut chara_list = chara_colle
        .find(doc! {}).await?
        .try_collect::<Vec<CharacterDocument>>().await?;

    let chrono_fmt = "%Y-%m-%dT%H:%M:%S%z";
    let Some(oldest) = NaiveDate::from_yo_opt(1965, 1) else {
        bail!("could not find a day on the calendar");
    };

    info!("start flattening");
    let mut batch = vec![];
    let mut inserted_chara_list = vec![];
    for anime in  ani_list {
        let year = match anime.aired.from {
            Some(s) => {
                let date = NaiveDate::parse_from_str(&s, &chrono_fmt)?;
                if date < oldest {
                    continue;
                }
                date.year()
            }
            None => continue
        };

        let media_type = match anime.media_type {
            Some(s) => {
                if !is_expected_media_type(&s) {
                    continue;
                }
                s
            } 
            None => continue
        };

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

                let chara = chara_list.remove(idx);
                if inserted_chara_list.contains(&chara.mal_id) {
                    continue;
                }
                
                batch.push(FlatDocument{
                    item_id: ItemId { 
                        id: chara.mal_id, 
                        document_type: ItemType::Character 
                    },
                    url: chara.url,
                    parent: Some(Parent{
                        id: anime.mal_id,
                        name: anime.title.clone(),
                        name_japanese: anime.title_japanese.clone(),
                    }),
                    tags: vec![],
                    name: chara.name,
                    name_english: None,
                    name_japanese: chara.name_kanji,
                    aliases: chara.nicknames,
                    description: chara.about,
                });
                inserted_chara_list.push(chara.mal_id);
            }
        }

        let mut tags = vec![];
        tags.push(media_type);
        tags.push(match anime.season {
            Some(s) => {
                match s.as_str() {
                    "winter" | "Winter" => format!("{year} Winter"),
                    "spring" | "Spring" => format!("{year} Spring"),
                    "summer" | "Summer" => format!("{year} Summer"),
                    "fall" | "Fall" => format!("{year} Fall"),
                    _ => year.to_string()
                }
            },
            None => year.to_string()
        });
        anime.genres.into_iter().for_each(|g| tags.push(g.name));
        anime.themes.into_iter().for_each(|g| tags.push(g.name));
        batch.push(FlatDocument{
            item_id: ItemId{
                id: anime.mal_id,
                document_type: ItemType::Anime
            },
            url: anime.url,
            parent: None,
            tags, 
            name: anime.title,
            name_english: anime.title_english,
            name_japanese: anime.title_japanese,
            aliases: anime.title_synonyms,
            description: anime.synopsis,
        });

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