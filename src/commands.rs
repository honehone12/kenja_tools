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
pub(crate) struct AnimeSimpleDocument {
    pub(crate) mal_id: i64,
    pub(crate) url: String,
    #[serde(rename = "type")]
    pub(crate) media_type: String,
    pub(crate) aired: AiredPeriod,
    pub(crate) title: String,
    pub(crate) title_english: Option<String>,
    pub(crate) title_japanese: Option<String>,
    pub(crate) synopsis: Option<String>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct CharacterSimpleDocument {
    pub(crate) mal_id: i64,
    pub(crate) url: String,
    pub(crate) name: String,
    pub(crate) name_kanji: Option<String>,
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
pub(crate) struct AnimeCharacters {
    pub(crate) mal_id: i64,
    pub(crate) characters: Vec<CharacterCast>
}
