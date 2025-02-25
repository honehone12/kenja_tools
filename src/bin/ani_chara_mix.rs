use std::env;
use futures::stream::TryStreamExt;
use anirs_dev::{
    AnimeCharacters, 
    AnimeSimple, 
    AnimeText, 
    CharacterSimple
};
use mongodb::{bson::doc, Client as MongoClient};
use tracing::{debug, info, warn};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();
    dotenvy::dotenv()?;

    let mongo_uri = env::var("MONGO_URI")?;
    let mongo_client = MongoClient::with_uri_str(mongo_uri).await?;
    let db = mongo_client.database("anime");
    let ani_colle = db.collection::<AnimeSimple>("anime_all_ages");
    let ani_chara_colle = db.collection::<AnimeCharacters>("anime_chara");
    let chara_colle = db.collection::<CharacterSimple>("chara");
    let ani_text_colle = db.collection::<AnimeText>("anime_text_all_ages"); 

    info!("getting anime list...");
    let mut ani_list = ani_colle.find(doc! {}).await?;
    info!("getting anime-character list...");
    let ani_chara_list = ani_chara_colle
        .find(doc! {}).await?
        .try_collect::<Vec<AnimeCharacters>>().await?;
    info!("getting character list...");
    let chara_list = chara_colle
        .find(doc! {}).await?
        .try_collect::<Vec<CharacterSimple>>().await?;

    info!("start iteration");
    let mut i = 0;
    let mut batch = vec![];
    while let Some(anime) = ani_list.try_next().await? {
        let mut characters = vec![];
        
        if let Some(ani_chara) = ani_chara_list
            .iter()
            .find(|ac| ac.mal_id == anime.mal_id)
        {
            for c in ani_chara.characters.iter() {
                let Some(chara_props) = c["character"].as_object() else {
                    warn!("unexpected property [character]");
                    continue;
                };
                
                let Some(chara_mal_id) = chara_props["mal_id"].as_i64() else {
                    warn!("unexpected property [mal_id]");
                    continue;
                };
    
                let Some(chara) = chara_list
                    .iter()
                    .find(|c| c.mal_id == chara_mal_id)
                else {
                    debug!("could not find character:mal_id {chara_mal_id}");
                    continue;
                };
    
                characters.push(chara.clone());
            }
        } else {
            debug!("could not find anime mal_id:{}", anime.mal_id);
        };

        let anime_text = AnimeText{
            anime,
            characters
        };
        batch.push(anime_text);
        i += 1;

        if batch.len() >= 100 {
            let result = ani_text_colle.insert_many(&batch).await?;
            info!("inserted {}items", result.inserted_ids.len());
            info!("current iteration {i}");
            batch.clear();
        }
    }

    let result = ani_text_colle.insert_many(&batch).await?;
    info!("inserted {}items", result.inserted_ids.len());
    info!("done");
    Ok(())
}
