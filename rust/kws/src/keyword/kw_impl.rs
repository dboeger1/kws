use crate::keyword::{
    Category,
    KeywordData,
};


pub static DATA: KeywordData = KeywordData {
    value: "impl",
    category: |_| -> Option<Category> {
        Some(Category::Strict)
    }
};
