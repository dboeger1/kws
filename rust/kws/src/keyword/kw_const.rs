use crate::{
    Category,
    KeywordData,
};


pub static DATA: KeywordData = KeywordData {
    value: "const",
    category: |_| -> Option<Category> {
        Some(Category::Strict)
    }
};
