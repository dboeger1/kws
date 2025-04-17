use kws::{
    Category,
    Edition,
    Keyword,
};
use std::{
    collections::{
        HashMap,
        HashSet,
    },
    sync::LazyLock,
};


#[test]
fn keywords() {
    for (edition, expected) in &*EXPECTED {
        for (category, keywords) in expected {
            assert_eq!(
                ***keywords,
                edition
                    .keywords()
                    .filter(|keyword|
                        (keyword.category)(&edition).as_ref() == Some(&category)
                    )
                    .collect::<HashSet<_>>(),
            )
        }
    }
}


type Expected = LazyLock<
    HashMap<
        Edition,
        HashMap<
            Category,
            &'static ExpectedKeywords,
        >,
    >,
>;

static EXPECTED: Expected = LazyLock::new(|| {
    [
        (
            Edition::Rust2015,
            [
                (Category::Strict,      &RUST_2015_KEYWORDS_STRICT),
                (Category::Reserved,    &RUST_2015_KEYWORDS_RESERVED),
                (Category::Weak,        &RUST_2015_KEYWORDS_WEAK),
            ].into_iter().collect(),
        ),
        (
            Edition::Rust2018,
            [
                (Category::Strict,      &RUST_2018_KEYWORDS_STRICT),
                (Category::Reserved,    &RUST_2018_KEYWORDS_RESERVED),
                (Category::Weak,        &RUST_2018_KEYWORDS_WEAK),
            ].into_iter().collect(),
        ),
        (
            Edition::Rust2021,
            [
                (Category::Strict,      &RUST_2021_KEYWORDS_STRICT),
                (Category::Reserved,    &RUST_2021_KEYWORDS_RESERVED),
                (Category::Weak,        &RUST_2021_KEYWORDS_WEAK),
            ].into_iter().collect(),
        ),
        (
            Edition::Rust2024,
            [
                (Category::Strict,      &RUST_2024_KEYWORDS_STRICT),
                (Category::Reserved,    &RUST_2024_KEYWORDS_RESERVED),
                (Category::Weak,        &RUST_2024_KEYWORDS_WEAK),
            ].into_iter().collect(),
        ),
    ].into_iter().collect()
});


type ExpectedKeywords = LazyLock<HashSet<Keyword>>;

static RUST_2015_KEYWORDS_STRICT: ExpectedKeywords = LazyLock::new(||
    [
        Keyword::As,
        Keyword::Break,
        Keyword::Const,
        Keyword::Continue,
        Keyword::Crate,
        Keyword::Else,
        Keyword::Enum,
        Keyword::Extern,
        Keyword::False,
        Keyword::Fn,
        Keyword::For,
        Keyword::If,
        Keyword::Impl,
        Keyword::In,
        Keyword::Let,
        Keyword::Loop,
        Keyword::Match,
        Keyword::Mod,
        Keyword::Move,
        Keyword::Mut,
        Keyword::Pub,
        Keyword::Ref,
        Keyword::Return,
        Keyword::SelfValue,
        Keyword::SelfType,
        Keyword::Static,
        Keyword::Struct,
        Keyword::Super,
        Keyword::Trait,
        Keyword::True,
        Keyword::Type,
        Keyword::Unsafe,
        Keyword::Use,
        Keyword::Where,
        Keyword::While,
    ].into_iter().collect()
);

static RUST_2015_KEYWORDS_RESERVED: ExpectedKeywords = LazyLock::new(||
    [
        Keyword::Abstract,
        Keyword::Become,
        Keyword::Box,
        Keyword::Do,
        Keyword::Final,
        Keyword::Macro,
        Keyword::Override,
        Keyword::Priv,
        Keyword::Typeof,
        Keyword::Unsized,
        Keyword::Virtual,
        Keyword::Yield,
    ].into_iter().collect()
);

static RUST_2015_KEYWORDS_WEAK: ExpectedKeywords = LazyLock::new(||
    [
        Keyword::Dyn,
        Keyword::MacroRules,
        Keyword::Raw,
        Keyword::Safe,
        Keyword::StaticLifetime,
        Keyword::Union,
    ].into_iter().collect()
);

static RUST_2018_KEYWORDS_STRICT: ExpectedKeywords = LazyLock::new(||
    [
        Keyword::As,
        Keyword::Async,
        Keyword::Await,
        Keyword::Break,
        Keyword::Const,
        Keyword::Continue,
        Keyword::Crate,
        Keyword::Dyn,
        Keyword::Else,
        Keyword::Enum,
        Keyword::Extern,
        Keyword::False,
        Keyword::Fn,
        Keyword::For,
        Keyword::If,
        Keyword::Impl,
        Keyword::In,
        Keyword::Let,
        Keyword::Loop,
        Keyword::Match,
        Keyword::Mod,
        Keyword::Move,
        Keyword::Mut,
        Keyword::Pub,
        Keyword::Ref,
        Keyword::Return,
        Keyword::SelfValue,
        Keyword::SelfType,
        Keyword::Static,
        Keyword::Struct,
        Keyword::Super,
        Keyword::Trait,
        Keyword::True,
        Keyword::Type,
        Keyword::Unsafe,
        Keyword::Use,
        Keyword::Where,
        Keyword::While,
    ].into_iter().collect()
);

static RUST_2018_KEYWORDS_RESERVED: ExpectedKeywords = LazyLock::new(||
    [
        Keyword::Abstract,
        Keyword::Become,
        Keyword::Box,
        Keyword::Do,
        Keyword::Final,
        Keyword::Macro,
        Keyword::Override,
        Keyword::Priv,
        Keyword::Try,
        Keyword::Typeof,
        Keyword::Unsized,
        Keyword::Virtual,
        Keyword::Yield,
    ].into_iter().collect()
);

static RUST_2018_KEYWORDS_WEAK: ExpectedKeywords = LazyLock::new(||
    [
        Keyword::MacroRules,
        Keyword::Raw,
        Keyword::Safe,
        Keyword::StaticLifetime,
        Keyword::Union,
    ].into_iter().collect()
);

static RUST_2021_KEYWORDS_STRICT: ExpectedKeywords = LazyLock::new(||
    [
        Keyword::As,
        Keyword::Async,
        Keyword::Await,
        Keyword::Break,
        Keyword::Const,
        Keyword::Continue,
        Keyword::Crate,
        Keyword::Dyn,
        Keyword::Else,
        Keyword::Enum,
        Keyword::Extern,
        Keyword::False,
        Keyword::Fn,
        Keyword::For,
        Keyword::If,
        Keyword::Impl,
        Keyword::In,
        Keyword::Let,
        Keyword::Loop,
        Keyword::Match,
        Keyword::Mod,
        Keyword::Move,
        Keyword::Mut,
        Keyword::Pub,
        Keyword::Ref,
        Keyword::Return,
        Keyword::SelfValue,
        Keyword::SelfType,
        Keyword::Static,
        Keyword::Struct,
        Keyword::Super,
        Keyword::Trait,
        Keyword::True,
        Keyword::Type,
        Keyword::Unsafe,
        Keyword::Use,
        Keyword::Where,
        Keyword::While,
    ].into_iter().collect()
);

static RUST_2021_KEYWORDS_RESERVED: ExpectedKeywords = LazyLock::new(||
    [
        Keyword::Abstract,
        Keyword::Become,
        Keyword::Box,
        Keyword::Do,
        Keyword::Final,
        Keyword::Macro,
        Keyword::Override,
        Keyword::Priv,
        Keyword::Try,
        Keyword::Typeof,
        Keyword::Unsized,
        Keyword::Virtual,
        Keyword::Yield,
    ].into_iter().collect()
);

static RUST_2021_KEYWORDS_WEAK: ExpectedKeywords = LazyLock::new(||
    [
        Keyword::MacroRules,
        Keyword::Raw,
        Keyword::Safe,
        Keyword::StaticLifetime,
        Keyword::Union,
    ].into_iter().collect()
);

static RUST_2024_KEYWORDS_STRICT: ExpectedKeywords = LazyLock::new(||
    [
        Keyword::As,
        Keyword::Async,
        Keyword::Await,
        Keyword::Break,
        Keyword::Const,
        Keyword::Continue,
        Keyword::Crate,
        Keyword::Dyn,
        Keyword::Else,
        Keyword::Enum,
        Keyword::Extern,
        Keyword::False,
        Keyword::Fn,
        Keyword::For,
        Keyword::If,
        Keyword::Impl,
        Keyword::In,
        Keyword::Let,
        Keyword::Loop,
        Keyword::Match,
        Keyword::Mod,
        Keyword::Move,
        Keyword::Mut,
        Keyword::Pub,
        Keyword::Ref,
        Keyword::Return,
        Keyword::SelfValue,
        Keyword::SelfType,
        Keyword::Static,
        Keyword::Struct,
        Keyword::Super,
        Keyword::Trait,
        Keyword::True,
        Keyword::Type,
        Keyword::Unsafe,
        Keyword::Use,
        Keyword::Where,
        Keyword::While,
    ].into_iter().collect()
);

static RUST_2024_KEYWORDS_RESERVED: ExpectedKeywords = LazyLock::new(||
    [
        Keyword::Abstract,
        Keyword::Become,
        Keyword::Box,
        Keyword::Do,
        Keyword::Final,
        Keyword::Gen,
        Keyword::Macro,
        Keyword::Override,
        Keyword::Priv,
        Keyword::Try,
        Keyword::Typeof,
        Keyword::Unsized,
        Keyword::Virtual,
        Keyword::Yield,
    ].into_iter().collect()
);

static RUST_2024_KEYWORDS_WEAK: ExpectedKeywords = LazyLock::new(||
    [
        Keyword::MacroRules,
        Keyword::Raw,
        Keyword::Safe,
        Keyword::StaticLifetime,
        Keyword::Union,
    ].into_iter().collect()
);
