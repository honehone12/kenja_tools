use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize, };
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::skip_serializing_none;

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Eq, Clone, Copy, Debug)]
#[repr(i32)]
pub enum ItemType32 {
    Unspecified = 0,
    Anime = 1,
    Character = 2
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Eq, Clone, Copy, Debug)]
#[repr(i32)]
pub enum Rating32 {
    Unspecified = 0,
    AllAges = 1,
    Hentai = 2
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Parent {
    pub id: ObjectId,
    pub name: String,
    pub name_japanese: Option<String>
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FlatDocument {
    // mongodb's Date obj is unix mils
    pub updated_at: u64,
    pub item_type: ItemType32,
    pub rating: Rating32,
    pub url: String,
    pub img: String,
    pub parent: Option<Parent>,
    pub name: String,
    pub name_english: Option<String>,
    pub name_japanese: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub aliases: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub studios: Vec<String>,
    pub staff: String,
    pub description: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Img {
    pub img: String,
}
