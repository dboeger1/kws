use crate::{
    Category,
    KeywordData,
};


pub static DATA: KeywordData = KeywordData {
    value: "break",
    category: |_| -> Option<Category> {
        Some(Category::Strict)
    }
};
