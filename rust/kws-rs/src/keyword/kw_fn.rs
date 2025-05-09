use crate::{
    Category,
    KeywordData,
};


pub static DATA: KeywordData = KeywordData {
    value: "fn",
    category: |_| -> Option<Category> {
        Some(Category::Strict)
    }
};
