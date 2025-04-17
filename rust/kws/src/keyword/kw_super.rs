use crate::keyword::{
    Category,
    KeywordData,
};


pub static DATA: KeywordData = KeywordData {
    value: "super",
    category: |_| -> Option<Category> {
        Some(Category::Strict)
    }
};
