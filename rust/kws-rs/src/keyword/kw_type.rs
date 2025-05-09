use crate::{
    Category,
    KeywordData,
};


pub static DATA: KeywordData = KeywordData {
    value: "type",
    category: |_| -> Option<Category> {
        Some(Category::Strict)
    }
};
