use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Season {
    pub year: u32,
    pub seasons: Vec<String>
}
