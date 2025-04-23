use crate::{
    Category,
    KeywordData,
};


pub static DATA: KeywordData = KeywordData {
    value: "struct",
    category: |_| -> Option<Category> {
        Some(Category::Strict)
    }
};
