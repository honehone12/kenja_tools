use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::skip_serializing_none;
use std::fmt::Display;

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Eq, Clone, Copy, Debug)]
#[repr(i32)]
pub enum ItemType {
    Unspecified = 0,
    Anime,
    Character,
    YVideo,
}

impl Display for ItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ItemType::Unspecified => Err(std::fmt::Error),
            ItemType::Anime => write!(f, "anime"),
            ItemType::Character => write!(f, "character"),
            ItemType::YVideo => write!(f, "yvideo"),
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Parent {
    pub name: String,
    pub name_japanese: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FlatDocument {
    // mongodb's Date obj is unix mils
    pub updated_at: u64,
    pub item_type: ItemType,
    pub unique: Option<String>,
    pub img: Option<String>,
    pub src: Option<String>,
    pub name: Option<String>,
    pub name_english: Option<String>,
    pub name_japanese: Option<String>,
    pub parent: Option<Parent>,
}

impl FlatDocument {
    pub fn new_anime(
        updated_at: u64,
        img: String,
        src: String,
        name: String,
        name_english: Option<String>,
        name_japanese: Option<String>,
    ) -> Self {
        Self {
            updated_at,
            item_type: ItemType::Anime,
            img: Some(img),
            src: Some(src),
            name: Some(name),
            name_english,
            name_japanese,
            parent: None,
            unique: None,
        }
    }

    pub fn new_character(
        updated_at: u64,
        img: String,
        src: String,
        name: String,
        name_japanese: Option<String>,
        parent: Parent,
    ) -> Self {
        Self {
            updated_at,
            item_type: ItemType::Character,
            img: Some(img),
            src: Some(src),
            name: Some(name),
            name_japanese,
            parent: Some(parent),
            name_english: None,
            unique: None,
        }
    }

    pub fn new_yvideo(updated_at: u64, unique: String, parent: Parent) -> Self {
        Self {
            updated_at,
            item_type: ItemType::YVideo,
            unique: Some(unique),
            parent: Some(parent),
            img: None,
            src: None,
            name: None,
            name_english: None,
            name_japanese: None,
        }
    }
}
