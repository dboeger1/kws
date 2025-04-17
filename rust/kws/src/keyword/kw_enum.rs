use crate::keyword::{
    Category,
    KeywordData,
};


pub static DATA: KeywordData = KeywordData {
    value: "enum",
    category: |_| -> Option<Category> {
        Some(Category::Strict)
    }
};
