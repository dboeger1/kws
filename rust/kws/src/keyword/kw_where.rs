use crate::{
    Category,
    KeywordData,
};


pub static DATA: KeywordData = KeywordData {
    value: "where",
    category: |_| -> Option<Category> {
        Some(Category::Strict)
    }
};
