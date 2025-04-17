use crate::keyword::{
    Category,
    KeywordData,
};


pub static DATA: KeywordData = KeywordData {
    value: "virtual",
    category: |_| -> Option<Category> {
        Some(Category::Reserved)
    }
};
