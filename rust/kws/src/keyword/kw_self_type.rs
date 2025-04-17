use crate::keyword::{
    Category,
    KeywordData,
};


pub static DATA: KeywordData = KeywordData {
    value: "Self",
    category: |_| -> Option<Category> {
        Some(Category::Strict)
    }
};
