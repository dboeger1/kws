use crate::keyword::{
    Category,
    KeywordData,
};


pub static DATA: KeywordData = KeywordData {
    value: "safe",
    category: |_| -> Option<Category> {
        Some(Category::Weak)
    }
};
