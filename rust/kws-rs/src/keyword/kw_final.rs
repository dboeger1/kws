use crate::{
    Category,
    KeywordData,
};


pub static DATA: KeywordData = KeywordData {
    value: "final",
    category: |_| -> Option<Category> {
        Some(Category::Reserved)
    }
};
