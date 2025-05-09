use crate::{
    Category,
    KeywordData,
};


pub static DATA: KeywordData = KeywordData {
    value: "union",
    category: |_| -> Option<Category> {
        Some(Category::Weak)
    }
};
