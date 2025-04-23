use crate::{
    Category,
    KeywordData,
};


pub static DATA: KeywordData = KeywordData {
    value: "become",
    category: |_| -> Option<Category> {
        Some(Category::Reserved)
    }
};
