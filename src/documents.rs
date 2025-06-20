pub mod anime;
pub mod anime_search;
pub mod anime_raw;

use std::fmt::{Display, Formatter, Result as FmtResult};
use clap::ValueEnum;

use crate::documents::anime_search::ItemType32;

#[derive(ValueEnum, Clone, Debug)]
pub enum Rating {
    AllAges,
    Hentai
}

impl Display for Rating {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Rating::AllAges => f.write_str("all_ages"),
            Rating::Hentai => f.write_str("hentai")
        }
    }
}

impl Rating {
    #[inline]
    pub fn as_suffix(&self, source: &str) -> String {
        let mut s = self.to_string();
        s.insert(0, '_');
        s.insert_str(0, source);
        s
    }

    #[inline]
    pub fn to_32(&self) -> anime_search::Rating32 {
        match self {
            Rating::AllAges => anime_search::Rating32::AllAges,
            Rating::Hentai => anime_search::Rating32::Hentai,
        }
    }
}

#[derive(ValueEnum, Clone, Debug)]
pub enum ItemType {
    Anime,
    Chara
}

impl Display for ItemType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            ItemType::Anime => f.write_str("anime"),
            ItemType::Chara=> f.write_str("character")
        }
    }
}

impl ItemType {
    #[inline]
    pub fn to_32(&self) -> anime_search::ItemType32 {
        match self {
            ItemType::Anime => ItemType32::Anime,
            ItemType::Chara => ItemType32::Character
        }
    }
}
