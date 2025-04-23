use crate::{
    Category,
    Edition,
    KeywordData,
};


pub static DATA: KeywordData = KeywordData {
    value: "await",
    category: |edition| -> Option<Category> {
        match edition {
            Edition::Rust2015 => None,
            _ => Some(Category::Strict),
        }
    }
};
