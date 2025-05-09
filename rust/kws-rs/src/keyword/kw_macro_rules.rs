use crate::{
    Category,
    KeywordData,
};


pub static DATA: KeywordData = KeywordData {
    value: "macro_rules",
    category: |_| -> Option<Category> {
        Some(Category::Weak)
    }
};
