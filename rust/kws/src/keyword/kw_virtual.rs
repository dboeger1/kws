use crate::{
    Category,
    KeywordData,
};


pub static DATA: KeywordData = KeywordData {
    value: "virtual",
    category: |_| -> Option<Category> {
        Some(Category::Reserved)
    }
};
