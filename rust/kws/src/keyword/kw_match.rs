use crate::keyword::{
    Category,
    KeywordData,
};


pub static DATA: KeywordData = KeywordData {
    value: "match",
    category: |_| -> Option<Category> {
        Some(Category::Strict)
    }
};
