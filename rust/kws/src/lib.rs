mod edition;
mod keyword;


pub use edition::Edition;
pub use keyword::{
    Category,
    Keyword,
    KeywordData,
};


#[derive(Debug, thiserror::Error)]
#[error("{message}")]
pub struct Error {
    message: String,
    source: Option<Box<dyn std::error::Error>>,
}
