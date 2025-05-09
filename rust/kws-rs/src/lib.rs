//! # kws-rs
//!
//! kws-rs is a small helper library for identifying [keywords] in rust.
//!
//!
//! ## Usage
//!
//! The `Keyword` enum contains all keywords across all rust editions. You can
//! specify a variant directly or attempt to get one from a `&str`:
//!
//! ```rust
//! use kws_rs::Keyword;
//!
//! assert!(Keyword::try_from("not a keyword").is_err());
//! assert!(matches!(
//!     Keyword::try_from("enum"),
//!     Ok(Keyword::Enum),
//! ));
//! ````
//!
//! To get the keyword category for a particular edition, you must specify the
//! edition:
//!  
//! ```rust
//! use kws_rs::{Category, Edition, Keyword};
//!
//! let keyword = Keyword::Async;
//! assert!((keyword.category)(&Edition::Rust2015).is_none());
//! assert!(matches!(
//!     (keyword.category)(&Edition::Rust2018),
//!     Some(Category::Strict),
//! ));
//! ```
//!
//! Most of the time, you'll probably only care about whichever edition you're
//! using to build your programs. The `Edition` enum provides some helper
//! functions which may minimize verbosity:
//!
//! ```rust
//! use kws_rs::{Category, Edition, Keyword};
//!
//! let edition = Edition::Rust2021;
//! assert!(edition.keyword("match").is_some());
//! assert!(edition.keyword("gen").is_none());
//! ```
//!
//!
//! [keywords]: https://doc.rust-lang.org/reference/keywords.html


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
