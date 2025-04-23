use crate::{
    Category,
    KeywordData,
};


pub static DATA: KeywordData = KeywordData {
    value: "unsafe",
    category: |_| -> Option<Category> {
        Some(Category::Strict)
    }
};
