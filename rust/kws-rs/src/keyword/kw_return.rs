use crate::{
    Category,
    KeywordData,
};


pub static DATA: KeywordData = KeywordData {
    value: "return",
    category: |_| -> Option<Category> {
        Some(Category::Strict)
    }
};
