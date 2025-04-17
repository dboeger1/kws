use crate::keyword::{
    Category,
    KeywordData,
};


pub static DATA: KeywordData = KeywordData {
    value: "mut",
    category: |_| -> Option<Category> {
        Some(Category::Strict)
    }
};
