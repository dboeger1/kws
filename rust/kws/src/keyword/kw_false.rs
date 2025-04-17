use crate::keyword::{
    Category,
    KeywordData,
};


pub static DATA: KeywordData = KeywordData {
    value: "false",
    category: |_| -> Option<Category> {
        Some(Category::Strict)
    }
};
