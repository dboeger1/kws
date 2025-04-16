use crate::types::{
    category::Category,
    edition::Edition,
    keyword_data::KeywordData,
};


pub static DATA: KeywordData = KeywordData {
    value: "gen",
    category: |edition| -> Option<Category> {
        match edition {
            Edition::Rust2024 => Some(Category::Reserved),
            _ => None,
        }
    }
};
