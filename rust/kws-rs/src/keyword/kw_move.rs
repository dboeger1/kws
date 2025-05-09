use crate::{
    Category,
    KeywordData,
};


pub static DATA: KeywordData = KeywordData {
    value: "move",
    category: |_| -> Option<Category> {
        Some(Category::Strict)
    }
};
