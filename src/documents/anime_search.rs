use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize, };
use serde_with::skip_serializing_none;

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
    pub url: String,
    pub img: String,
    pub parent: Option<Parent>,
    pub name: String,
    pub name_english: Option<String>,
    pub name_japanese: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub aliases: Vec<String>,
    pub description: Option<String>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Img {
    pub img: String,
}
