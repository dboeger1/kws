use crate::types::{
    category::Category,
    edition::Edition,
    keyword_data::KeywordData,
};


pub static DATA: KeywordData = KeywordData {
    value: "try",
    category: |edition| -> Option<Category> {
        match edition {
            Edition::Rust2015 => None,
            _ => Some(Category::Reserved),
        }
    }
};
