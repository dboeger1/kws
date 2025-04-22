mod common;


use common::{
    edition::Edition,
    keyword::{
        Category,
        Keyword,
    },
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
                    .0
                    .keywords()
                    .filter(|keyword|
                        (keyword.category)(&edition.0)
                            .map(|category| Category::from(category))
                            .as_ref() ==
                            Some(category)
                    )
                    .map(|keyword| keyword.into())
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
            Edition(kws::Edition::Rust2015),
            [
                (Category(kws::Category::Strict),      &RUST_2015_KEYWORDS_STRICT),
                (Category(kws::Category::Reserved),    &RUST_2015_KEYWORDS_RESERVED),
                (Category(kws::Category::Weak),        &RUST_2015_KEYWORDS_WEAK),
            ].into_iter().collect(),
        ),
        (
            Edition(kws::Edition::Rust2018),
            [
                (Category(kws::Category::Strict),      &RUST_2018_KEYWORDS_STRICT),
                (Category(kws::Category::Reserved),    &RUST_2018_KEYWORDS_RESERVED),
                (Category(kws::Category::Weak),        &RUST_2018_KEYWORDS_WEAK),
            ].into_iter().collect(),
        ),
        (
            Edition(kws::Edition::Rust2021),
            [
                (Category(kws::Category::Strict),      &RUST_2021_KEYWORDS_STRICT),
                (Category(kws::Category::Reserved),    &RUST_2021_KEYWORDS_RESERVED),
                (Category(kws::Category::Weak),        &RUST_2021_KEYWORDS_WEAK),
            ].into_iter().collect(),
        ),
        (
            Edition(kws::Edition::Rust2024),
            [
                (Category(kws::Category::Strict),      &RUST_2024_KEYWORDS_STRICT),
                (Category(kws::Category::Reserved),    &RUST_2024_KEYWORDS_RESERVED),
                (Category(kws::Category::Weak),        &RUST_2024_KEYWORDS_WEAK),
            ].into_iter().collect(),
        ),
    ].into_iter().collect()
});


type ExpectedKeywords = LazyLock<HashSet<Keyword>>;

static RUST_2015_KEYWORDS_STRICT: ExpectedKeywords = LazyLock::new(||
    [
        kws::Keyword::As,
        kws::Keyword::Break,
        kws::Keyword::Const,
        kws::Keyword::Continue,
        kws::Keyword::Crate,
        kws::Keyword::Else,
        kws::Keyword::Enum,
        kws::Keyword::Extern,
        kws::Keyword::False,
        kws::Keyword::Fn,
        kws::Keyword::For,
        kws::Keyword::If,
        kws::Keyword::Impl,
        kws::Keyword::In,
        kws::Keyword::Let,
        kws::Keyword::Loop,
        kws::Keyword::Match,
        kws::Keyword::Mod,
        kws::Keyword::Move,
        kws::Keyword::Mut,
        kws::Keyword::Pub,
        kws::Keyword::Ref,
        kws::Keyword::Return,
        kws::Keyword::SelfValue,
        kws::Keyword::SelfType,
        kws::Keyword::Static,
        kws::Keyword::Struct,
        kws::Keyword::Super,
        kws::Keyword::Trait,
        kws::Keyword::True,
        kws::Keyword::Type,
        kws::Keyword::Unsafe,
        kws::Keyword::Use,
        kws::Keyword::Where,
        kws::Keyword::While,
    ]
        .into_iter()
        .map(|keyword| keyword.into())
        .collect()
);

static RUST_2015_KEYWORDS_RESERVED: ExpectedKeywords = LazyLock::new(||
    [
        kws::Keyword::Abstract,
        kws::Keyword::Become,
        kws::Keyword::Box,
        kws::Keyword::Do,
        kws::Keyword::Final,
        kws::Keyword::Macro,
        kws::Keyword::Override,
        kws::Keyword::Priv,
        kws::Keyword::Typeof,
        kws::Keyword::Unsized,
        kws::Keyword::Virtual,
        kws::Keyword::Yield,
    ]
        .into_iter()
        .map(|keyword| keyword.into())
        .collect()
);

static RUST_2015_KEYWORDS_WEAK: ExpectedKeywords = LazyLock::new(||
    [
        kws::Keyword::Dyn,
        kws::Keyword::MacroRules,
        kws::Keyword::Raw,
        kws::Keyword::Safe,
        kws::Keyword::StaticLifetime,
        kws::Keyword::Union,
    ]
        .into_iter()
        .map(|keyword| keyword.into())
        .collect()
);

static RUST_2018_KEYWORDS_STRICT: ExpectedKeywords = LazyLock::new(||
    [
        kws::Keyword::As,
        kws::Keyword::Async,
        kws::Keyword::Await,
        kws::Keyword::Break,
        kws::Keyword::Const,
        kws::Keyword::Continue,
        kws::Keyword::Crate,
        kws::Keyword::Dyn,
        kws::Keyword::Else,
        kws::Keyword::Enum,
        kws::Keyword::Extern,
        kws::Keyword::False,
        kws::Keyword::Fn,
        kws::Keyword::For,
        kws::Keyword::If,
        kws::Keyword::Impl,
        kws::Keyword::In,
        kws::Keyword::Let,
        kws::Keyword::Loop,
        kws::Keyword::Match,
        kws::Keyword::Mod,
        kws::Keyword::Move,
        kws::Keyword::Mut,
        kws::Keyword::Pub,
        kws::Keyword::Ref,
        kws::Keyword::Return,
        kws::Keyword::SelfValue,
        kws::Keyword::SelfType,
        kws::Keyword::Static,
        kws::Keyword::Struct,
        kws::Keyword::Super,
        kws::Keyword::Trait,
        kws::Keyword::True,
        kws::Keyword::Type,
        kws::Keyword::Unsafe,
        kws::Keyword::Use,
        kws::Keyword::Where,
        kws::Keyword::While,
    ]
        .into_iter()
        .map(|keyword| keyword.into())
        .collect()
);

static RUST_2018_KEYWORDS_RESERVED: ExpectedKeywords = LazyLock::new(||
    [
        kws::Keyword::Abstract,
        kws::Keyword::Become,
        kws::Keyword::Box,
        kws::Keyword::Do,
        kws::Keyword::Final,
        kws::Keyword::Macro,
        kws::Keyword::Override,
        kws::Keyword::Priv,
        kws::Keyword::Try,
        kws::Keyword::Typeof,
        kws::Keyword::Unsized,
        kws::Keyword::Virtual,
        kws::Keyword::Yield,
    ]
        .into_iter()
        .map(|keyword| keyword.into())
        .collect()
);

static RUST_2018_KEYWORDS_WEAK: ExpectedKeywords = LazyLock::new(||
    [
        kws::Keyword::MacroRules,
        kws::Keyword::Raw,
        kws::Keyword::Safe,
        kws::Keyword::StaticLifetime,
        kws::Keyword::Union,
    ]
        .into_iter()
        .map(|keyword| keyword.into())
        .collect()
);

static RUST_2021_KEYWORDS_STRICT: ExpectedKeywords = LazyLock::new(||
    [
        kws::Keyword::As,
        kws::Keyword::Async,
        kws::Keyword::Await,
        kws::Keyword::Break,
        kws::Keyword::Const,
        kws::Keyword::Continue,
        kws::Keyword::Crate,
        kws::Keyword::Dyn,
        kws::Keyword::Else,
        kws::Keyword::Enum,
        kws::Keyword::Extern,
        kws::Keyword::False,
        kws::Keyword::Fn,
        kws::Keyword::For,
        kws::Keyword::If,
        kws::Keyword::Impl,
        kws::Keyword::In,
        kws::Keyword::Let,
        kws::Keyword::Loop,
        kws::Keyword::Match,
        kws::Keyword::Mod,
        kws::Keyword::Move,
        kws::Keyword::Mut,
        kws::Keyword::Pub,
        kws::Keyword::Ref,
        kws::Keyword::Return,
        kws::Keyword::SelfValue,
        kws::Keyword::SelfType,
        kws::Keyword::Static,
        kws::Keyword::Struct,
        kws::Keyword::Super,
        kws::Keyword::Trait,
        kws::Keyword::True,
        kws::Keyword::Type,
        kws::Keyword::Unsafe,
        kws::Keyword::Use,
        kws::Keyword::Where,
        kws::Keyword::While,
    ]
        .into_iter()
        .map(|keyword| keyword.into())
        .collect()
);

static RUST_2021_KEYWORDS_RESERVED: ExpectedKeywords = LazyLock::new(||
    [
        kws::Keyword::Abstract,
        kws::Keyword::Become,
        kws::Keyword::Box,
        kws::Keyword::Do,
        kws::Keyword::Final,
        kws::Keyword::Macro,
        kws::Keyword::Override,
        kws::Keyword::Priv,
        kws::Keyword::Try,
        kws::Keyword::Typeof,
        kws::Keyword::Unsized,
        kws::Keyword::Virtual,
        kws::Keyword::Yield,
    ]
        .into_iter()
        .map(|keyword| keyword.into())
        .collect()
);

static RUST_2021_KEYWORDS_WEAK: ExpectedKeywords = LazyLock::new(||
    [
        kws::Keyword::MacroRules,
        kws::Keyword::Raw,
        kws::Keyword::Safe,
        kws::Keyword::StaticLifetime,
        kws::Keyword::Union,
    ]
        .into_iter()
        .map(|keyword| keyword.into())
        .collect()
);

static RUST_2024_KEYWORDS_STRICT: ExpectedKeywords = LazyLock::new(||
    [
        kws::Keyword::As,
        kws::Keyword::Async,
        kws::Keyword::Await,
        kws::Keyword::Break,
        kws::Keyword::Const,
        kws::Keyword::Continue,
        kws::Keyword::Crate,
        kws::Keyword::Dyn,
        kws::Keyword::Else,
        kws::Keyword::Enum,
        kws::Keyword::Extern,
        kws::Keyword::False,
        kws::Keyword::Fn,
        kws::Keyword::For,
        kws::Keyword::If,
        kws::Keyword::Impl,
        kws::Keyword::In,
        kws::Keyword::Let,
        kws::Keyword::Loop,
        kws::Keyword::Match,
        kws::Keyword::Mod,
        kws::Keyword::Move,
        kws::Keyword::Mut,
        kws::Keyword::Pub,
        kws::Keyword::Ref,
        kws::Keyword::Return,
        kws::Keyword::SelfValue,
        kws::Keyword::SelfType,
        kws::Keyword::Static,
        kws::Keyword::Struct,
        kws::Keyword::Super,
        kws::Keyword::Trait,
        kws::Keyword::True,
        kws::Keyword::Type,
        kws::Keyword::Unsafe,
        kws::Keyword::Use,
        kws::Keyword::Where,
        kws::Keyword::While,
    ]
        .into_iter()
        .map(|keyword| keyword.into())
        .collect()
);

static RUST_2024_KEYWORDS_RESERVED: ExpectedKeywords = LazyLock::new(||
    [
        kws::Keyword::Abstract,
        kws::Keyword::Become,
        kws::Keyword::Box,
        kws::Keyword::Do,
        kws::Keyword::Final,
        kws::Keyword::Gen,
        kws::Keyword::Macro,
        kws::Keyword::Override,
        kws::Keyword::Priv,
        kws::Keyword::Try,
        kws::Keyword::Typeof,
        kws::Keyword::Unsized,
        kws::Keyword::Virtual,
        kws::Keyword::Yield,
    ]
        .into_iter()
        .map(|keyword| keyword.into())
        .collect()
);

static RUST_2024_KEYWORDS_WEAK: ExpectedKeywords = LazyLock::new(||
    [
        kws::Keyword::MacroRules,
        kws::Keyword::Raw,
        kws::Keyword::Safe,
        kws::Keyword::StaticLifetime,
        kws::Keyword::Union,
    ]
        .into_iter()
        .map(|keyword| keyword.into())
        .collect()
);
