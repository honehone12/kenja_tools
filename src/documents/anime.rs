use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AiredPeriod {
    pub from: Option<String>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Genre {
    pub name: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AnimeDocument {
    pub mal_id: i64,
    pub url: String,
    #[serde(rename = "type")]
    pub media_type: Option<String>,
    pub aired: AiredPeriod,
    pub title: String,
    pub title_english: Option<String>,
    pub title_japanese: Option<String>,
    pub title_synonyms: Vec<String>,
    pub synopsis: Option<String>,
    pub season: Option<String>,
    pub genres: Vec<Genre>,
    pub themes: Vec<Genre>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CharacterDocument {
    pub mal_id: i64,
    pub url: String,
    pub name: String,
    pub name_kanji: Option<String>,
    pub nicknames: Vec<String>,
    pub about: Option<String>
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Character {
    pub mal_id: i64
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CharacterCast {
    pub character: Character
    // voice_actors etc...
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AniCharaBridge {
    pub mal_id: i64,
    pub characters: Vec<CharacterCast>
}
