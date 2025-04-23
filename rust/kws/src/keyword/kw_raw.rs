use crate::{
    Category,
    KeywordData,
};


pub static DATA: KeywordData = KeywordData {
    value: "raw",
    category: |_| -> Option<Category> {
        Some(Category::Weak)
    }
};
