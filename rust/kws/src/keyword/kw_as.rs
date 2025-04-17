use crate::keyword::{
    Category,
    KeywordData,
};


pub static DATA: KeywordData = KeywordData {
    value: "as",
    category: |_| -> Option<Category> {
        Some(Category::Strict)
    }
};
