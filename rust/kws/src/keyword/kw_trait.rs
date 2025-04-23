use crate::{
    Category,
    KeywordData,
};


pub static DATA: KeywordData = KeywordData {
    value: "trait",
    category: |_| -> Option<Category> {
        Some(Category::Strict)
    }
};
