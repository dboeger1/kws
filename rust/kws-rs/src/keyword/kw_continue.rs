use crate::{
    Category,
    KeywordData,
};


pub static DATA: KeywordData = KeywordData {
    value: "continue",
    category: |_| -> Option<Category> {
        Some(Category::Strict)
    }
};
