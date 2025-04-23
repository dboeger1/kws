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
                            .map(|category| Category(category))
                            .as_ref() ==
                            Some(category)
                    )
                    .map(|keyword| Keyword(keyword))
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
        Keyword(kws::Keyword::As),
        Keyword(kws::Keyword::Break),
        Keyword(kws::Keyword::Const),
        Keyword(kws::Keyword::Continue),
        Keyword(kws::Keyword::Crate),
        Keyword(kws::Keyword::Else),
        Keyword(kws::Keyword::Enum),
        Keyword(kws::Keyword::Extern),
        Keyword(kws::Keyword::False),
        Keyword(kws::Keyword::Fn),
        Keyword(kws::Keyword::For),
        Keyword(kws::Keyword::If),
        Keyword(kws::Keyword::Impl),
        Keyword(kws::Keyword::In),
        Keyword(kws::Keyword::Let),
        Keyword(kws::Keyword::Loop),
        Keyword(kws::Keyword::Match),
        Keyword(kws::Keyword::Mod),
        Keyword(kws::Keyword::Move),
        Keyword(kws::Keyword::Mut),
        Keyword(kws::Keyword::Pub),
        Keyword(kws::Keyword::Ref),
        Keyword(kws::Keyword::Return),
        Keyword(kws::Keyword::SelfValue),
        Keyword(kws::Keyword::SelfType),
        Keyword(kws::Keyword::Static),
        Keyword(kws::Keyword::Struct),
        Keyword(kws::Keyword::Super),
        Keyword(kws::Keyword::Trait),
        Keyword(kws::Keyword::True),
        Keyword(kws::Keyword::Type),
        Keyword(kws::Keyword::Unsafe),
        Keyword(kws::Keyword::Use),
        Keyword(kws::Keyword::Where),
        Keyword(kws::Keyword::While),
    ]
        .into_iter()
        .collect()
);

static RUST_2015_KEYWORDS_RESERVED: ExpectedKeywords = LazyLock::new(||
    [
        Keyword(kws::Keyword::Abstract),
        Keyword(kws::Keyword::Become),
        Keyword(kws::Keyword::Box),
        Keyword(kws::Keyword::Do),
        Keyword(kws::Keyword::Final),
        Keyword(kws::Keyword::Macro),
        Keyword(kws::Keyword::Override),
        Keyword(kws::Keyword::Priv),
        Keyword(kws::Keyword::Typeof),
        Keyword(kws::Keyword::Unsized),
        Keyword(kws::Keyword::Virtual),
        Keyword(kws::Keyword::Yield),
    ]
        .into_iter()
        .collect()
);

static RUST_2015_KEYWORDS_WEAK: ExpectedKeywords = LazyLock::new(||
    [
        Keyword(kws::Keyword::Dyn),
        Keyword(kws::Keyword::MacroRules),
        Keyword(kws::Keyword::Raw),
        Keyword(kws::Keyword::Safe),
        Keyword(kws::Keyword::StaticLifetime),
        Keyword(kws::Keyword::Union),
    ]
        .into_iter()
        .collect()
);

static RUST_2018_KEYWORDS_STRICT: ExpectedKeywords = LazyLock::new(||
    [
        Keyword(kws::Keyword::As),
        Keyword(kws::Keyword::Async),
        Keyword(kws::Keyword::Await),
        Keyword(kws::Keyword::Break),
        Keyword(kws::Keyword::Const),
        Keyword(kws::Keyword::Continue),
        Keyword(kws::Keyword::Crate),
        Keyword(kws::Keyword::Dyn),
        Keyword(kws::Keyword::Else),
        Keyword(kws::Keyword::Enum),
        Keyword(kws::Keyword::Extern),
        Keyword(kws::Keyword::False),
        Keyword(kws::Keyword::Fn),
        Keyword(kws::Keyword::For),
        Keyword(kws::Keyword::If),
        Keyword(kws::Keyword::Impl),
        Keyword(kws::Keyword::In),
        Keyword(kws::Keyword::Let),
        Keyword(kws::Keyword::Loop),
        Keyword(kws::Keyword::Match),
        Keyword(kws::Keyword::Mod),
        Keyword(kws::Keyword::Move),
        Keyword(kws::Keyword::Mut),
        Keyword(kws::Keyword::Pub),
        Keyword(kws::Keyword::Ref),
        Keyword(kws::Keyword::Return),
        Keyword(kws::Keyword::SelfValue),
        Keyword(kws::Keyword::SelfType),
        Keyword(kws::Keyword::Static),
        Keyword(kws::Keyword::Struct),
        Keyword(kws::Keyword::Super),
        Keyword(kws::Keyword::Trait),
        Keyword(kws::Keyword::True),
        Keyword(kws::Keyword::Type),
        Keyword(kws::Keyword::Unsafe),
        Keyword(kws::Keyword::Use),
        Keyword(kws::Keyword::Where),
        Keyword(kws::Keyword::While),
    ]
        .into_iter()
        .collect()
);

static RUST_2018_KEYWORDS_RESERVED: ExpectedKeywords = LazyLock::new(||
    [
        Keyword(kws::Keyword::Abstract),
        Keyword(kws::Keyword::Become),
        Keyword(kws::Keyword::Box),
        Keyword(kws::Keyword::Do),
        Keyword(kws::Keyword::Final),
        Keyword(kws::Keyword::Macro),
        Keyword(kws::Keyword::Override),
        Keyword(kws::Keyword::Priv),
        Keyword(kws::Keyword::Try),
        Keyword(kws::Keyword::Typeof),
        Keyword(kws::Keyword::Unsized),
        Keyword(kws::Keyword::Virtual),
        Keyword(kws::Keyword::Yield),
    ]
        .into_iter()
        .collect()
);

static RUST_2018_KEYWORDS_WEAK: ExpectedKeywords = LazyLock::new(||
    [
        Keyword(kws::Keyword::MacroRules),
        Keyword(kws::Keyword::Raw),
        Keyword(kws::Keyword::Safe),
        Keyword(kws::Keyword::StaticLifetime),
        Keyword(kws::Keyword::Union),
    ]
        .into_iter()
        .collect()
);

static RUST_2021_KEYWORDS_STRICT: ExpectedKeywords = LazyLock::new(||
    [
        Keyword(kws::Keyword::As),
        Keyword(kws::Keyword::Async),
        Keyword(kws::Keyword::Await),
        Keyword(kws::Keyword::Break),
        Keyword(kws::Keyword::Const),
        Keyword(kws::Keyword::Continue),
        Keyword(kws::Keyword::Crate),
        Keyword(kws::Keyword::Dyn),
        Keyword(kws::Keyword::Else),
        Keyword(kws::Keyword::Enum),
        Keyword(kws::Keyword::Extern),
        Keyword(kws::Keyword::False),
        Keyword(kws::Keyword::Fn),
        Keyword(kws::Keyword::For),
        Keyword(kws::Keyword::If),
        Keyword(kws::Keyword::Impl),
        Keyword(kws::Keyword::In),
        Keyword(kws::Keyword::Let),
        Keyword(kws::Keyword::Loop),
        Keyword(kws::Keyword::Match),
        Keyword(kws::Keyword::Mod),
        Keyword(kws::Keyword::Move),
        Keyword(kws::Keyword::Mut),
        Keyword(kws::Keyword::Pub),
        Keyword(kws::Keyword::Ref),
        Keyword(kws::Keyword::Return),
        Keyword(kws::Keyword::SelfValue),
        Keyword(kws::Keyword::SelfType),
        Keyword(kws::Keyword::Static),
        Keyword(kws::Keyword::Struct),
        Keyword(kws::Keyword::Super),
        Keyword(kws::Keyword::Trait),
        Keyword(kws::Keyword::True),
        Keyword(kws::Keyword::Type),
        Keyword(kws::Keyword::Unsafe),
        Keyword(kws::Keyword::Use),
        Keyword(kws::Keyword::Where),
        Keyword(kws::Keyword::While),
    ]
        .into_iter()
        .collect()
);

static RUST_2021_KEYWORDS_RESERVED: ExpectedKeywords = LazyLock::new(||
    [
        Keyword(kws::Keyword::Abstract),
        Keyword(kws::Keyword::Become),
        Keyword(kws::Keyword::Box),
        Keyword(kws::Keyword::Do),
        Keyword(kws::Keyword::Final),
        Keyword(kws::Keyword::Macro),
        Keyword(kws::Keyword::Override),
        Keyword(kws::Keyword::Priv),
        Keyword(kws::Keyword::Try),
        Keyword(kws::Keyword::Typeof),
        Keyword(kws::Keyword::Unsized),
        Keyword(kws::Keyword::Virtual),
        Keyword(kws::Keyword::Yield),
    ]
        .into_iter()
        .collect()
);

static RUST_2021_KEYWORDS_WEAK: ExpectedKeywords = LazyLock::new(||
    [
        Keyword(kws::Keyword::MacroRules),
        Keyword(kws::Keyword::Raw),
        Keyword(kws::Keyword::Safe),
        Keyword(kws::Keyword::StaticLifetime),
        Keyword(kws::Keyword::Union),
    ]
        .into_iter()
        .collect()
);

static RUST_2024_KEYWORDS_STRICT: ExpectedKeywords = LazyLock::new(||
    [
        Keyword(kws::Keyword::As),
        Keyword(kws::Keyword::Async),
        Keyword(kws::Keyword::Await),
        Keyword(kws::Keyword::Break),
        Keyword(kws::Keyword::Const),
        Keyword(kws::Keyword::Continue),
        Keyword(kws::Keyword::Crate),
        Keyword(kws::Keyword::Dyn),
        Keyword(kws::Keyword::Else),
        Keyword(kws::Keyword::Enum),
        Keyword(kws::Keyword::Extern),
        Keyword(kws::Keyword::False),
        Keyword(kws::Keyword::Fn),
        Keyword(kws::Keyword::For),
        Keyword(kws::Keyword::If),
        Keyword(kws::Keyword::Impl),
        Keyword(kws::Keyword::In),
        Keyword(kws::Keyword::Let),
        Keyword(kws::Keyword::Loop),
        Keyword(kws::Keyword::Match),
        Keyword(kws::Keyword::Mod),
        Keyword(kws::Keyword::Move),
        Keyword(kws::Keyword::Mut),
        Keyword(kws::Keyword::Pub),
        Keyword(kws::Keyword::Ref),
        Keyword(kws::Keyword::Return),
        Keyword(kws::Keyword::SelfValue),
        Keyword(kws::Keyword::SelfType),
        Keyword(kws::Keyword::Static),
        Keyword(kws::Keyword::Struct),
        Keyword(kws::Keyword::Super),
        Keyword(kws::Keyword::Trait),
        Keyword(kws::Keyword::True),
        Keyword(kws::Keyword::Type),
        Keyword(kws::Keyword::Unsafe),
        Keyword(kws::Keyword::Use),
        Keyword(kws::Keyword::Where),
        Keyword(kws::Keyword::While),
    ]
        .into_iter()
        .collect()
);

static RUST_2024_KEYWORDS_RESERVED: ExpectedKeywords = LazyLock::new(||
    [
        Keyword(kws::Keyword::Abstract),
        Keyword(kws::Keyword::Become),
        Keyword(kws::Keyword::Box),
        Keyword(kws::Keyword::Do),
        Keyword(kws::Keyword::Final),
        Keyword(kws::Keyword::Gen),
        Keyword(kws::Keyword::Macro),
        Keyword(kws::Keyword::Override),
        Keyword(kws::Keyword::Priv),
        Keyword(kws::Keyword::Try),
        Keyword(kws::Keyword::Typeof),
        Keyword(kws::Keyword::Unsized),
        Keyword(kws::Keyword::Virtual),
        Keyword(kws::Keyword::Yield),
    ]
        .into_iter()
        .collect()
);

static RUST_2024_KEYWORDS_WEAK: ExpectedKeywords = LazyLock::new(||
    [
        Keyword(kws::Keyword::MacroRules),
        Keyword(kws::Keyword::Raw),
        Keyword(kws::Keyword::Safe),
        Keyword(kws::Keyword::StaticLifetime),
        Keyword(kws::Keyword::Union),
    ]
        .into_iter()
        .collect()
);
