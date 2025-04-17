use crate::{
    edition::Edition,
    keyword::{
        Category,
        KeywordData,
    },
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
