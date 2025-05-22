use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct AnimeCharacters {
    pub mal_id: i64,
    pub characters: Vec<Value>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Season {
    pub year: u32,
    pub seasons: Vec<String>
}
