use crate::types::keyword::Keyword;
use strum::IntoEnumIterator;
use super::keyword::KeywordDiscriminants;


#[derive(Eq, Hash, PartialEq)]
pub enum Edition {
    Rust2015,
    Rust2018,
    Rust2021,
    Rust2024,
}

impl Edition {
    pub fn keyword(&self, value: &str) -> Option<Keyword> {
        Keyword::try_from(value).ok().and_then(|keyword| {
            (keyword.category)(self).map(|_| keyword)
        })
    }

    pub fn keywords(&self) -> impl Iterator<Item = Keyword> {
        KeywordDiscriminants::iter().filter_map(|discriminant| {
            let keyword = Keyword::from(discriminant);
            (keyword.category)(self).map(|_| keyword)
        })
    }
}
