use crate::{
    Category,
    KeywordData,
};


pub static DATA: KeywordData = KeywordData {
    value: "let",
    category: |_| -> Option<Category> {
        Some(Category::Strict)
    }
};
