# kws-rs

kws-rs is a small helper library for identifying [keywords] in Rust.

## Usage

The `Keyword` enum contains all keywords across all Rust editions. You can
specify a variant directly or attempt to get one from a `&str`:

```rust
use kws::Keyword;

assert!(Keyword::try_from("not a keyword").is_err());
assert!(matches!(
    Keyword::try_from("enum"),
    Ok(Keyword::Enum),
));
```

To get the keyword category for a particular edition, you must specify the
edition:
 
```rust
use kws::{Category, Edition, Keyword};

let keyword = Keyword::Async;
assert!((keyword.category)(&Edition::Rust2015).is_none());
assert!(matches!(
    (keyword.category)(&Edition::Rust2018),
    Some(Category::Strict),
));
```

Most of the time, you'll probably only care about whichever edition you're using
to build your programs. The `Edition` enum provides some helper functions which
may minimize verbosity:

```rust
use kws::{Category, Edition, Keyword};

let edition = Edition::Rust2021;
assert!(edition.keyword("match").is_some());
assert!(edition.keyword("gen").is_none());
```

[keywords]: https://doc.rust-lang.org/reference/keywords.html
