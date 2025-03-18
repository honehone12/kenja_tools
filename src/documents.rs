pub(crate) mod anime;

use serde::{Deserialize, Serialize};

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
