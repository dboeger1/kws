use crate::types::{
    category::Category,
    keyword_data::KeywordData,
};


pub static DATA: KeywordData = KeywordData {
    value: "const",
    category: |_| -> Option<Category> {
        Some(Category::Strict)
    }
};
