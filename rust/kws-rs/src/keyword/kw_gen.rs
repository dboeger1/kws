use crate::{
    Category,
    Edition,
    KeywordData,
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
