use crate::keyword::{
    Category,
    KeywordData,
};


pub static DATA: KeywordData = KeywordData {
    value: "abstract",
    category: |_| -> Option<Category> {
        Some(Category::Reserved)
    }
};
