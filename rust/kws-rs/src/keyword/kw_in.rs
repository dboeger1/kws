use crate::{
    Category,
    KeywordData,
};


pub static DATA: KeywordData = KeywordData {
    value: "in",
    category: |_| -> Option<Category> {
        Some(Category::Strict)
    }
};
