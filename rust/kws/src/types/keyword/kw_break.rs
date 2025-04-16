use crate::types::{
    category::Category,
    keyword_data::KeywordData,
};


pub static DATA: KeywordData = KeywordData {
    value: "break",
    category: |_| -> Option<Category> {
        Some(Category::Strict)
    }
};
