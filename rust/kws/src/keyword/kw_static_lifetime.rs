use crate::{
    Category,
    KeywordData,
};


pub static DATA: KeywordData = KeywordData {
    value: "'static",
    category: |_| -> Option<Category> {
        Some(Category::Weak)
    }
};
