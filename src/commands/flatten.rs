use mongodb::Database;
use tracing::info;
use super::{
    AniCharaDocument, 
    AnimeDocument, 
    CharacterDocument, 
    Rating
};

pub(crate)  async fn flatten_main(rating: Rating, db: Database) 
-> anyhow::Result<()> {
    let ani_colle = format!("anime_{}", rating.to_string());
    let ani_colle = db.collection::<AnimeDocument>(&ani_colle);
    let ani_chara_colle = db.collection::<AniCharaDocument>("anime_chara");
    let chara_colle = db.collection::<CharacterDocument>("chara");
    let flat_colle = format!("flat_ani_chara_{}", rating.to_string());

    Ok(())
}