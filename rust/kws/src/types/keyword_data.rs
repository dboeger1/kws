use crate::types::{
    category::Category,
    edition::Edition,
};


#[derive(Debug, Eq, Hash, PartialEq)]
pub struct KeywordData {
    pub value: &'static str,
    pub category: fn (edition: &Edition) -> Option<Category>,
}
