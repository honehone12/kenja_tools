pub(crate) mod flatten;

use clap::ValueEnum;

#[derive(ValueEnum, Clone, Debug)]
pub(crate)  enum Rating {
    AllAges,
    Hentai
}
