use crate::types::{
    category::Category,
    edition::Edition,
    keyword_data::KeywordData,
};


pub static DATA: KeywordData = KeywordData {
    value: "dyn",
    category: |edition| -> Option<Category> {
        match edition {
            Edition::Rust2015 => Some(Category::Weak),
            _ => Some(Category::Strict),
        }
    }
};
