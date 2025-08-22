pub mod anime_src;
pub mod anime_search;
pub mod anime_raw;

use std::fmt::{Display, Formatter, Result as FmtResult};
use clap::ValueEnum;
use crate::documents::anime_search::ItemType32;

const RX: &'static str = "Rx";
const HENTAI: &'static str = "Hentai";

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
    pub const fn to_32(&self) -> anime_search::Rating32 {
        match self {
            Rating::AllAges => anime_search::Rating32::G,
            Rating::Hentai => anime_search::Rating32::X,
        }
    }

    #[inline]
    pub fn is_hentai_str(rating_str: &str) -> bool {
        return rating_str.contains(RX) || rating_str.contains(HENTAI)
    }

    #[inline]
    pub fn match_str(&self, rating_str: &str) -> bool {
        match self {
            Rating::AllAges => !Self::is_hentai_str(rating_str),
            Rating::Hentai => Self::is_hentai_str(rating_str)
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
    pub const fn to_32(&self) -> anime_search::ItemType32 {
        match self {
            ItemType::Anime => ItemType32::Anime,
            ItemType::Chara => ItemType32::Character
        }
    }
}
