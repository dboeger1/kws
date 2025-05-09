use crate::{
    Category,
    KeywordData,
};


pub static DATA: KeywordData = KeywordData {
    value: "use",
    category: |_| -> Option<Category> {
        Some(Category::Strict)
    }
};
