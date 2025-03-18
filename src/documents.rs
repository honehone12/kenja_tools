pub(crate) mod anime;

use serde::{Deserialize, Serialize};
use serde_repr::{Serialize_repr, Deserialize_repr};

#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug)]
#[repr(i32)]
pub(crate) enum ItemType {
    Anime = 1,
    Character = 2
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct ItemId {
    pub(crate) id: i64,
    pub(crate) document_type: ItemType
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct Parent {
    pub(crate) id: i64,
    pub(crate) name: String,
    pub(crate) name_japanese: Option<String>
}

#[derive(Serialize_repr, Deserialize_repr, Clone, Debug)]
#[repr(i32)]
pub(crate) enum TagType {
    Normal,
    Spoiler,
    Sinsi,
    Syukujo
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct Tag {
    pub(crate) name: String,
    pub(crate) tag_type: TagType
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct FlatDocument {
    pub(crate) item_id: ItemId,
    pub(crate) url: String,
    pub(crate) parent: Option<Parent>,
    pub(crate) tags: Vec<Tag>,
    pub(crate) name: String,
    pub(crate) name_english: Option<String>,
    pub(crate) name_japanese: Option<String>,
    pub(crate) aliases: Vec<String>,
    pub(crate) description: Option<String>
}
