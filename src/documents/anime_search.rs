use serde::{Deserialize, Serialize};
use serde_repr::{Serialize_repr, Deserialize_repr};

#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug)]
#[repr(i32)]
pub enum ItemType {
    Anime = 1,
    Character = 2
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ItemId {
    pub id: i64,
    pub item_type: ItemType
}

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
    pub parent: Option<Parent>,
    pub tags: Vec<String>,
    pub name: String,
    pub name_english: Option<String>,
    pub name_japanese: Option<String>,
    pub aliases: Vec<String>,
    pub description: Option<String>
}
