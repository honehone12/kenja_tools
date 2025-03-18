use serde::{Deserialize, Serialize};

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
    // voice_actors etc...
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct AniCharaBridge {
    pub(crate) mal_id: i64,
    pub(crate) characters: Vec<CharacterCast>
}
