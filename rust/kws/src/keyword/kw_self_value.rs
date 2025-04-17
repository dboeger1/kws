use crate::keyword::{
    Category,
    KeywordData,
};


pub static DATA: KeywordData = KeywordData {
    value: "self",
    category: |_| -> Option<Category> {
        Some(Category::Strict)
    }
};
