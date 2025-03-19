pub(crate) mod flatten;

use std::fmt::{Display, Formatter, Result as FmtResult};
use clap::ValueEnum;

#[derive(ValueEnum, Clone, Debug)]
pub(crate)  enum Rating {
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
