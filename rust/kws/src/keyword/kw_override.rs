use crate::keyword::{
    Category,
    KeywordData,
};


pub static DATA: KeywordData = KeywordData {
    value: "override",
    category: |_| -> Option<Category> {
        Some(Category::Reserved)
    }
};
