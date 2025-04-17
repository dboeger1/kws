use crate::keyword::{
    Category,
    KeywordData,
};


pub static DATA: KeywordData = KeywordData {
    value: "ref",
    category: |_| -> Option<Category> {
        Some(Category::Strict)
    }
};
