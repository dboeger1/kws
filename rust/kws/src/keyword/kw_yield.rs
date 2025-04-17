use crate::keyword::{
    Category,
    KeywordData,
};


pub static DATA: KeywordData = KeywordData {
    value: "yield",
    category: |_| -> Option<Category> {
        Some(Category::Reserved)
    }
};
