use serde::{Deserialize, Serialize};
use super::id::ItemId;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Parent {
    pub id: i64,
    pub name: String,
    pub name_japanese: Option<String>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FlatDocument {
    pub item_id: ItemId,
    pub url: String,
    pub img: String,
    pub parent: Option<Parent>,
    pub name: String,
    pub name_english: Option<String>,
    pub name_japanese: Option<String>,
    pub aliases: Vec<String>,
    pub description: Option<String>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Img {
    pub item_id: ItemId,
    pub img: String,
}
