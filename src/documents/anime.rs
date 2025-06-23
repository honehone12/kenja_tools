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
pub struct ImageUrls {
    pub image_url: Option<String>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Images {
    pub jpg: Option<ImageUrls>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Studio {
    pub name: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AnimeDocument {
    pub mal_id: i64,
    pub url: String,
    pub images: Option<Images>,
    #[serde(rename = "type")]
    pub media_type: Option<String>,
    pub aired: AiredPeriod,
    pub title: String,
    pub title_english: Option<String>,
    pub title_japanese: Option<String>,
    pub title_synonyms: Vec<String>,
    pub synopsis: Option<String>,
    pub season: Option<String>,
    pub studios: Vec<Studio>,
    pub favorites: u64
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CharacterDocument {
    pub mal_id: i64,
    pub url: String,
    pub images: Option<Images>,
    pub name: String,
    pub name_kanji: Option<String>,
    pub nicknames: Vec<String>,
    pub about: Option<String>,
    pub favorites: u64
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Person {
    pub name: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Character {
    pub mal_id: i64
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VoiceActor {
    pub person: Person
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CharacterCast {
    pub character: Character,
    pub voice_actors: Vec<VoiceActor>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AniCharaBridge {
    pub mal_id: i64,
    pub characters: Vec<CharacterCast>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Staff {
    pub person: Person,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StaffDocument {
    pub mal_id: i64,
    pub staffs: Vec<Staff>
}
