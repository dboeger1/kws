use crate::keyword::{
    Category,
    KeywordData,
};


pub static DATA: KeywordData = KeywordData {
    value: "pub",
    category: |_| -> Option<Category> {
        Some(Category::Strict)
    }
};
