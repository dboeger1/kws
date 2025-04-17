use crate::keyword::{
    Category,
    KeywordData,
};


pub static DATA: KeywordData = KeywordData {
    value: "for",
    category: |_| -> Option<Category> {
        Some(Category::Strict)
    }
};
