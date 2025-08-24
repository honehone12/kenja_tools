use std::fmt::{Display, Formatter, Result as FmtResult};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use clap::ValueEnum;
use crate::api::AnimeApiRawDocument;

#[derive(Serialize, Deserialize, Debug)]
pub struct AnimeCharacters {
    pub mal_id: i64,
    pub characters: Vec<Value>
}

impl AnimeApiRawDocument for AnimeCharacters {
    fn from_value_list(mal_id: i64, val: Vec<Value>) -> Self {
        Self {
            mal_id,
            characters: val
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnimeStaffs {
    pub mal_id: i64,
    pub staffs: Vec<Value>
}

impl AnimeApiRawDocument for AnimeStaffs {
    fn from_value_list(mal_id: i64, val: Vec<Value>) -> Self {
        Self {
            mal_id,
            staffs: val
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnimeLinks {
    pub mal_id: i64,
    pub links: Vec<Value>
}

impl AnimeApiRawDocument for AnimeLinks {
    fn from_value_list(mal_id: i64, val: Vec<Value>) -> Self {
        Self {
            mal_id,
            links: val
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnimeVideos {
    pub mal_id: i64,
    pub videos: Vec<Value>
}

impl AnimeApiRawDocument for AnimeVideos {
    fn from_value_list(mal_id: i64, val: Vec<Value>) -> Self {
        Self {
            mal_id,
            videos: val
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnimePictures {
    pub mal_id: i64,
    pub pictures: Vec<Value>
}

impl AnimeApiRawDocument for AnimePictures {
    fn from_value_list(mal_id: i64, val: Vec<Value>) -> Self {
        Self {
            mal_id,
            pictures: val
        }
    }
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
