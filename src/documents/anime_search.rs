use std::fmt::Display;
use serde::{Deserialize, Serialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_with::skip_serializing_none;

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Eq, Clone, Copy, Debug)]
#[repr(i32)]
pub enum ItemType {
    Unspecified = 0,
    Anime,
    Character,
    YVideo
}

impl Display for ItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ItemType::Unspecified => Err(std::fmt::Error),
            ItemType::Anime => write!(f, "anime"),
            ItemType::Character => write!(f, "character"),
            ItemType::YVideo => write!(f, "yvideo")
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Parent {
    pub name: String,
    pub name_japanese: Option<String>
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
        item_type: ItemType,
        img: String,
        src: String,
        name: String,
        name_english: Option<String>,
        name_japanese: Option<String>
    ) -> Self {
        Self {
            updated_at,
            item_type,
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
        item_type: ItemType,
        img: String,
        src: String,
        parent: Parent,
        name: String,
        name_japanese: Option<String>
    ) -> Self {
        Self{
            updated_at,
            item_type,
            img: Some(img),
            src: Some(src),
            parent: Some(parent),
            name: Some(name),
            name_japanese,
            name_english: None,
            unique: None
        }
    }

    pub fn new_image(
        updated_at: u64,
        item_type: ItemType,
        img: String,
        src: String,
        parent: Parent
    ) -> Self {
        Self {
            updated_at,
            item_type,
            img: Some(img),
            src: Some(src),
            parent: Some(parent),
            unique: None,
            name: None,
            name_english: None,
            name_japanese: None
        }
    }

    pub fn new_yvideo(
        updated_at: u64,
        item_type: ItemType,
        unique: String,
        parent: Parent
    ) -> Self {
        Self {
            updated_at,
            item_type,
            unique: Some(unique),
            parent: Some(parent),
            img: None,
            src: None,
            name: None,
            name_english: None,
            name_japanese: None
        }
    } 
}
