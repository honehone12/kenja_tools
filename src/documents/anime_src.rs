use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AiredPeriod {
    pub from: Option<String>
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
pub struct Producer {
    pub name: String
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AnimeSrc {
    pub mal_id: i64,
    pub url: String,
    pub images: Option<Images>,
    #[serde(rename = "type")]
    pub media_type: Option<String>,
    pub aired: AiredPeriod,
    pub rating: Option<String>,
    pub title: String,
    pub title_english: Option<String>,
    pub title_japanese: Option<String>,
    pub synopsis: Option<String>,
    pub producers: Vec<Producer>,
    pub favorites: u64
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CharacterSrc {
    pub mal_id: i64,
    pub url: String,
    pub images: Option<Images>,
    pub name: String,
    pub name_kanji: Option<String>,
    pub about: Option<String>,
    pub favorites: u64
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Person {
    pub name: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Character {
    pub mal_id: i64,
    pub url: String,
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
pub struct StaffSrc {
    pub mal_id: i64,
    pub staffs: Vec<Staff>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Link {
    pub name: String,
    pub url: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LinkSrc {
    pub mal_id: i64,
    pub links: Vec<Link>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct YoutubeImages {
    pub image_url: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct YoutubeVideo {
    pub youtube_id: String,
    pub url: String,
    pub embed_url: String,
    pub images: YoutubeImages 
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PromoVideo {
    pub title: String,
    pub trailer: YoutubeVideo
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MusicVideo {
    pub title: String,
    pub video: YoutubeVideo
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VideoSrc {
    pub promo: Vec<PromoVideo>,
    pub music_videos: Vec<MusicVideo>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ImgSrc {
    pub img: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ImgExSrc {
    pub mal_id: i64,
    pub pictures: Vec<Images>
}
