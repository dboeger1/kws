use crate::{
    Category,
    KeywordData,
};


pub static DATA: KeywordData = KeywordData {
    value: "mod",
    category: |_| -> Option<Category> {
        Some(Category::Strict)
    }
};
