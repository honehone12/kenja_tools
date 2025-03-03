pub(crate) mod flatten;

use std::fmt::{Display, Formatter, Result};
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use bson::serde_helpers::chrono_datetime_as_bson_datetime;

#[derive(ValueEnum, Clone, Debug)]
pub(crate)  enum Rating {
    AllAges,
    Hentai
}

impl Display for Rating {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Rating::AllAges => f.write_str("all_ages"),
            Rating::Hentai => f.write_str("hentai")
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct AiredPeriod {
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub(crate) from: DateTime<Utc>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct AnimeDocument {
    pub(crate) mal_id: i64,
    pub(crate) url: String,
    #[serde(rename = "type")]
    pub(crate) media_type: String,
    pub(crate) aired: AiredPeriod,
    pub(crate) title: String,
    pub(crate) title_english: Option<String>,
    pub(crate) title_japanese: String,
    pub(crate) synopsis: Option<String>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct CharacterDocument {
    pub(crate) mal_id: i64,
    pub(crate) url: String,
    pub(crate) name: String,
    pub(crate) name_kanji: String,
    pub(crate) about: Option<String>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct Character {
    pub(crate) mal_id: i64
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct CharacterCast {
    pub(crate) character: Character
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct AniCharaDocument {
    pub(crate) mal_id: i64,
    pub(crate) characters: Vec<CharacterCast>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) enum DocumentType {
    #[serde(rename = "anime")]
    Anime,
    #[serde(rename = "character")]
    Character
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct FlatDocument {
    pub(crate) mal_id: i64,
    pub(crate) doc_type: DocumentType,
    pub(crate) url: String,
    pub(crate) name: String,
    pub(crate) name_english: Option<String>,
    pub(crate) name_japanese: String,
    pub(crate) description: Option<String>
}

pub(crate) fn is_expected_media_type(media_type: &str) -> bool {
    match media_type {
        "TV" | "Movie" | "OVA" | "ONA" => true,
        _ => false
    }
}
