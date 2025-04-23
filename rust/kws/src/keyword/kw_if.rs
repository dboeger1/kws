use crate::{
    Category,
    KeywordData,
};


pub static DATA: KeywordData = KeywordData {
    value: "if",
    category: |_| -> Option<Category> {
        Some(Category::Strict)
    }
};
