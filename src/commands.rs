pub(crate) mod flatten;

use std::fmt::{Display, Formatter, Result as FmtResult};
use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(ValueEnum, Clone, Debug)]
pub(crate)  enum Rating {
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct AiredPeriod {
    pub(crate) from: Option<String>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct AnimeDocument {
    pub(crate) mal_id: i64,
    pub(crate) url: String,
    #[serde(rename = "type")]
    pub(crate) media_type: Option<String>,
    pub(crate) aired: AiredPeriod,
    pub(crate) title: String,
    pub(crate) title_english: Option<String>,
    pub(crate) title_japanese: Option<String>,
    pub(crate) title_synonyms: Vec<String>,
    pub(crate) synopsis: Option<String>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct CharacterDocument {
    pub(crate) mal_id: i64,
    pub(crate) url: String,
    pub(crate) name: String,
    pub(crate) name_kanji: Option<String>,
    pub(crate) nicknames: Vec<String>,
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
pub(crate) struct AniCharaBridge {
    pub(crate) mal_id: i64,
    pub(crate) characters: Vec<CharacterCast>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct Parent {
    pub(crate) mal_id: i64,
    pub(crate) name: String,
    pub(crate) name_japanese: Option<String>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct FlatDocument {
    pub(crate) mal_id: i64,
    pub(crate) url: String,
    pub(crate) parent: Option<Parent>,
    pub(crate) name: String,
    pub(crate) name_english: Option<String>,
    pub(crate) name_japanese: Option<String>,
    pub(crate) aliases: Vec<String>,
    pub(crate) description: Option<String>
}

pub(crate) fn is_expected_media_type(media_type: &str) -> bool {
    match media_type {
        "TV" | "Movie" | "OVA" | "ONA" => true,
        _ => false
    }
}
