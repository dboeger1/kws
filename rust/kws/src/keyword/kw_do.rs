use crate::{
    Category,
    KeywordData,
};


pub static DATA: KeywordData = KeywordData {
    value: "do",
    category: |_| -> Option<Category> {
        Some(Category::Reserved)
    }
};
