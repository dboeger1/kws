use crate::{
    Category,
    KeywordData,
};


pub static DATA: KeywordData = KeywordData {
    value: "extern",
    category: |_| -> Option<Category> {
        Some(Category::Strict)
    }
};
