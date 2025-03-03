use anyhow::bail;
use chrono::NaiveDate;
use futures::TryStreamExt;
use mongodb::{Database, bson::doc};
use tracing::{info, warn};
use crate::commands::is_expected_media_type;

use super::{
    AniCharaBridge, 
    AnimeDocument, 
    CharacterDocument, 
    FlatDocument, 
    DocumentType,
    Rating
};

pub(crate) async fn flatten_main(rating: Rating, db: Database) 
-> anyhow::Result<()> {

    let ani_colle = db.collection::<AnimeDocument>(
        &format!("anime_{}", rating.to_string())
    );
    let ani_chara_colle = db.collection::<AniCharaBridge>("anime_chara");
    let chara_colle = db.collection::<CharacterDocument>("chara");
    let flat_colle = db.collection::<FlatDocument>(
        &format!("flat_ani_chara_{}", rating.to_string())
    );

    info!("obtaining anime documents...");
    let mut ani_stream = ani_colle.find(doc! {}).await?;
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
    while let Some(anime) = ani_stream.try_next().await? {
        match anime.aired.from {
            Some(s) => {
                let date = NaiveDate::parse_from_str(&s, &chrono_fmt)?;
                if date < oldest {
                    continue;
                }
            }
            None => continue
        }

        match anime.media_type {
            Some(s) => {
                if !is_expected_media_type(&s) {
                    continue;
                }
            } 
            None => continue
        }
        
        batch.push(FlatDocument{
            mal_id: anime.mal_id,
            doc_type: DocumentType::Anime,
            url: anime.url,
            name: anime.title,
            name_english: anime.title_english,
            name_japanese: anime.title_japanese,
            description: anime.synopsis,
        });

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
                batch.push(FlatDocument{
                    mal_id: chara.mal_id,
                    doc_type: DocumentType::Character,
                    url: chara.url,
                    name: chara.name,
                    name_english: None,
                    name_japanese: chara.name_kanji,
                    description: chara.about,
                });
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