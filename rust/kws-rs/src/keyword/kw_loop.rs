use crate::{
    Category,
    KeywordData,
};


pub static DATA: KeywordData = KeywordData {
    value: "loop",
    category: |_| -> Option<Category> {
        Some(Category::Strict)
    }
};
