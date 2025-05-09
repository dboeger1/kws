use crate::{
    Category,
    KeywordData,
};


pub static DATA: KeywordData = KeywordData {
    value: "for",
    category: |_| -> Option<Category> {
        Some(Category::Strict)
    }
};
