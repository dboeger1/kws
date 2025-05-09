use crate::{
    Category,
    Keyword,
};
use strum::IntoEnumIterator;


pub enum Edition {
    Rust2015,
    Rust2018,
    Rust2021,
    Rust2024,
}

impl Edition {
    pub fn category(&self, keyword: &Keyword) -> Option<Category> {
        (keyword.category)(self)
    }

    pub fn keyword(&self, value: &str) -> Option<Keyword> {
        Keyword::try_from(value).ok().and_then(|keyword| {
            (keyword.category)(self).map(|_| keyword)
        })
    }

    pub fn keywords(&self) -> impl Iterator<Item = Keyword> {
        Keyword::iter()
            .filter(|keyword| (keyword.category)(self).is_some())
    }
}
