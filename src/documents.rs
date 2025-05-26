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
        let mut s = source.to_string();
        s.push_str(&self.to_string());
        s
    }
}
