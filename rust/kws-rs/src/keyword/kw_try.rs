use crate::{
    Category,
    Edition,
    KeywordData,
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
