use crate::keyword::{
    Category,
    KeywordData,
};


pub static DATA: KeywordData = KeywordData {
    value: "crate",
    category: |_| -> Option<Category> {
        Some(Category::Strict)
    }
};
