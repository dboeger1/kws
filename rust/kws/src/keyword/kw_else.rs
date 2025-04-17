use crate::keyword::{
    Category,
    KeywordData,
};


pub static DATA: KeywordData = KeywordData {
    value: "else",
    category: |_| -> Option<Category> {
        Some(Category::Strict)
    }
};
