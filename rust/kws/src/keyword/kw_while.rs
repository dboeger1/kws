use crate::{
    Category,
    KeywordData,
};


pub static DATA: KeywordData = KeywordData {
    value: "while",
    category: |_| -> Option<Category> {
        Some(Category::Strict)
    }
};
