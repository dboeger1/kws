//! # kws-rs
//!
//! kws-rs is a small helper library for identifying keywords in rust.
//!
//!
//! ## Keywords
//!
//! ```rust
//! use kws_rs::{
//!     Category,
//!     Edition,
//!     Keyword,
//! };
//!
//! let keyword = Keyword::Dyn;
//! assert_eq!("dyn", keyword.value);
//! assert!(matches!(
//!     (keyword.category)(&Edition::Rust2015),
//!     Some(Category::Weak),
//! ));
//! assert!(matches!(
//!     (keyword.category)(&Edition::Rust2018),
//!     Some(Category::Strict),
//! ));
//! ```
//!
//!
//! ## Editions
//!
//!
//! ## Motivation
//!
//! There are similar alternatives, but they often have limitations which
//! prevent them from being used for certain use cases. The main one is that
//! they tend to filter out keywords by edition at compile time, such that a
//! keyword will only be identified if it is a keyword in the edition with which
//! the program is built.
//!
//! kws-rs, on the other hand, provides information about all editions at
//! runtime regardless of which edition the program is built with. This allows
//! you to build code in one edition with name-related logic for other editions.
//! A practical example of when you might want this would be generating a hint
//! message which tells the user which edition they need to upgrade to in order
//! to use a keyword.
//!
//! Still only care about a single edition? No problem! kws-rs still makes that
//! just as easy.


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
