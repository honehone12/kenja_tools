use std::fmt::{Display, Formatter, Result as FmtResult};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use clap::ValueEnum;

#[derive(Serialize, Deserialize, Debug)]
pub struct AnimeCharacters {
    pub mal_id: i64,
    pub characters: Vec<Value>
}

#[derive(Serialize, Deserialize, Debug)]

pub struct AnimeStaffs {
    pub mal_id: i64,
    pub staffs: Vec<Value>
}

#[derive(Serialize, Deserialize, Debug)]

pub struct AnimeLinks {
    pub mal_id: i64,
    pub links: Vec<Value>
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Season {
    Winter,
    Spring,
    Summer,
    Fall
}

impl Display for Season {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Season::Winter => f.write_str("winter"),
            Season::Spring => f.write_str("spring"),
            Season::Summer => f.write_str("summer"),
            Season::Fall => f.write_str("fall"),
        }
    }
}
