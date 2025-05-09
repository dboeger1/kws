use crate::{
    Category,
    KeywordData,
};


pub static DATA: KeywordData = KeywordData {
    value: "true",
    category: |_| -> Option<Category> {
        Some(Category::Strict)
    }
};
