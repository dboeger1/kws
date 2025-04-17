use crate::keyword::{
    Category,
    KeywordData,
};


pub static DATA: KeywordData = KeywordData {
    value: "box",
    category: |_| -> Option<Category> {
        Some(Category::Reserved)
    }
};
