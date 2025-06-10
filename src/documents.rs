pub mod anime;
pub mod anime_search;
pub mod anime_raw;

use std::fmt::{Display, Formatter, Result as FmtResult};
use clap::ValueEnum;

#[derive(ValueEnum, Clone, Debug)]
pub enum Rating {
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

impl Rating {
    #[inline]
    pub fn as_suffix(&self, source: &str) -> String {
        let mut s = self.to_string();
        s.insert(0, '_');
        s.insert_str(0, source);
        s
    }

    #[inline]
    pub fn to_32(&self) -> anime_search::Rating32 {
        match self {
            Rating::AllAges => anime_search::Rating32::AllAges,
            Rating::Hentai => anime_search::Rating32::Hentai,
        }
    }
}
