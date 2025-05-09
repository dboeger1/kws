mod common;


use common::{
    edition::Edition,
    keyword::{
        Category,
        Keyword,
    },
};
use strum::IntoEnumIterator;
use std::{
    collections::{
        HashMap,
        HashSet,
    },
    sync::LazyLock,
};


#[test]
fn keyword() {
    for (edition, _) in &*EXPECTED {
        for keyword in kws_rs::Keyword::iter() {
            assert_eq!(
                (keyword.category)(&edition.0)
                    .map(|category| Category(category)),
                edition.0.keyword(keyword.value)
                    .and_then(|keyword| (keyword.data().category)(&edition.0))
                    .map(|category| Category(category)),
            );
        }
    }
}

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
            Edition(kws_rs::Edition::Rust2015),
            [
                (Category(kws_rs::Category::Strict),   &RUST_2015_STRICT),
                (Category(kws_rs::Category::Reserved), &RUST_2015_RESERVED),
                (Category(kws_rs::Category::Weak),     &RUST_2015_WEAK),
            ].into_iter().collect(),
        ),
        (
            Edition(kws_rs::Edition::Rust2018),
            [
                (Category(kws_rs::Category::Strict),   &RUST_2018_STRICT),
                (Category(kws_rs::Category::Reserved), &RUST_2018_RESERVED),
                (Category(kws_rs::Category::Weak),     &RUST_2018_WEAK),
            ].into_iter().collect(),
        ),
        (
            Edition(kws_rs::Edition::Rust2021),
            [
                (Category(kws_rs::Category::Strict),   &RUST_2021_STRICT),
                (Category(kws_rs::Category::Reserved), &RUST_2021_RESERVED),
                (Category(kws_rs::Category::Weak),     &RUST_2021_WEAK),
            ].into_iter().collect(),
        ),
        (
            Edition(kws_rs::Edition::Rust2024),
            [
                (Category(kws_rs::Category::Strict),   &RUST_2024_STRICT),
                (Category(kws_rs::Category::Reserved), &RUST_2024_RESERVED),
                (Category(kws_rs::Category::Weak),     &RUST_2024_WEAK),
            ].into_iter().collect(),
        ),
    ].into_iter().collect()
});


type ExpectedKeywords = LazyLock<HashSet<Keyword>>;

static RUST_2015_STRICT: ExpectedKeywords = LazyLock::new(||
    [
        Keyword(kws_rs::Keyword::As),
        Keyword(kws_rs::Keyword::Break),
        Keyword(kws_rs::Keyword::Const),
        Keyword(kws_rs::Keyword::Continue),
        Keyword(kws_rs::Keyword::Crate),
        Keyword(kws_rs::Keyword::Else),
        Keyword(kws_rs::Keyword::Enum),
        Keyword(kws_rs::Keyword::Extern),
        Keyword(kws_rs::Keyword::False),
        Keyword(kws_rs::Keyword::Fn),
        Keyword(kws_rs::Keyword::For),
        Keyword(kws_rs::Keyword::If),
        Keyword(kws_rs::Keyword::Impl),
        Keyword(kws_rs::Keyword::In),
        Keyword(kws_rs::Keyword::Let),
        Keyword(kws_rs::Keyword::Loop),
        Keyword(kws_rs::Keyword::Match),
        Keyword(kws_rs::Keyword::Mod),
        Keyword(kws_rs::Keyword::Move),
        Keyword(kws_rs::Keyword::Mut),
        Keyword(kws_rs::Keyword::Pub),
        Keyword(kws_rs::Keyword::Ref),
        Keyword(kws_rs::Keyword::Return),
        Keyword(kws_rs::Keyword::SelfValue),
        Keyword(kws_rs::Keyword::SelfType),
        Keyword(kws_rs::Keyword::Static),
        Keyword(kws_rs::Keyword::Struct),
        Keyword(kws_rs::Keyword::Super),
        Keyword(kws_rs::Keyword::Trait),
        Keyword(kws_rs::Keyword::True),
        Keyword(kws_rs::Keyword::Type),
        Keyword(kws_rs::Keyword::Unsafe),
        Keyword(kws_rs::Keyword::Use),
        Keyword(kws_rs::Keyword::Where),
        Keyword(kws_rs::Keyword::While),
    ]
        .into_iter()
        .collect()
);

static RUST_2015_RESERVED: ExpectedKeywords = LazyLock::new(||
    [
        Keyword(kws_rs::Keyword::Abstract),
        Keyword(kws_rs::Keyword::Become),
        Keyword(kws_rs::Keyword::Box),
        Keyword(kws_rs::Keyword::Do),
        Keyword(kws_rs::Keyword::Final),
        Keyword(kws_rs::Keyword::Macro),
        Keyword(kws_rs::Keyword::Override),
        Keyword(kws_rs::Keyword::Priv),
        Keyword(kws_rs::Keyword::Typeof),
        Keyword(kws_rs::Keyword::Unsized),
        Keyword(kws_rs::Keyword::Virtual),
        Keyword(kws_rs::Keyword::Yield),
    ]
        .into_iter()
        .collect()
);

static RUST_2015_WEAK: ExpectedKeywords = LazyLock::new(||
    [
        Keyword(kws_rs::Keyword::Dyn),
        Keyword(kws_rs::Keyword::MacroRules),
        Keyword(kws_rs::Keyword::Raw),
        Keyword(kws_rs::Keyword::Safe),
        Keyword(kws_rs::Keyword::StaticLifetime),
        Keyword(kws_rs::Keyword::Union),
    ]
        .into_iter()
        .collect()
);

static RUST_2018_STRICT: ExpectedKeywords = LazyLock::new(||
    [
        Keyword(kws_rs::Keyword::As),
        Keyword(kws_rs::Keyword::Async),
        Keyword(kws_rs::Keyword::Await),
        Keyword(kws_rs::Keyword::Break),
        Keyword(kws_rs::Keyword::Const),
        Keyword(kws_rs::Keyword::Continue),
        Keyword(kws_rs::Keyword::Crate),
        Keyword(kws_rs::Keyword::Dyn),
        Keyword(kws_rs::Keyword::Else),
        Keyword(kws_rs::Keyword::Enum),
        Keyword(kws_rs::Keyword::Extern),
        Keyword(kws_rs::Keyword::False),
        Keyword(kws_rs::Keyword::Fn),
        Keyword(kws_rs::Keyword::For),
        Keyword(kws_rs::Keyword::If),
        Keyword(kws_rs::Keyword::Impl),
        Keyword(kws_rs::Keyword::In),
        Keyword(kws_rs::Keyword::Let),
        Keyword(kws_rs::Keyword::Loop),
        Keyword(kws_rs::Keyword::Match),
        Keyword(kws_rs::Keyword::Mod),
        Keyword(kws_rs::Keyword::Move),
        Keyword(kws_rs::Keyword::Mut),
        Keyword(kws_rs::Keyword::Pub),
        Keyword(kws_rs::Keyword::Ref),
        Keyword(kws_rs::Keyword::Return),
        Keyword(kws_rs::Keyword::SelfValue),
        Keyword(kws_rs::Keyword::SelfType),
        Keyword(kws_rs::Keyword::Static),
        Keyword(kws_rs::Keyword::Struct),
        Keyword(kws_rs::Keyword::Super),
        Keyword(kws_rs::Keyword::Trait),
        Keyword(kws_rs::Keyword::True),
        Keyword(kws_rs::Keyword::Type),
        Keyword(kws_rs::Keyword::Unsafe),
        Keyword(kws_rs::Keyword::Use),
        Keyword(kws_rs::Keyword::Where),
        Keyword(kws_rs::Keyword::While),
    ]
        .into_iter()
        .collect()
);

static RUST_2018_RESERVED: ExpectedKeywords = LazyLock::new(||
    [
        Keyword(kws_rs::Keyword::Abstract),
        Keyword(kws_rs::Keyword::Become),
        Keyword(kws_rs::Keyword::Box),
        Keyword(kws_rs::Keyword::Do),
        Keyword(kws_rs::Keyword::Final),
        Keyword(kws_rs::Keyword::Macro),
        Keyword(kws_rs::Keyword::Override),
        Keyword(kws_rs::Keyword::Priv),
        Keyword(kws_rs::Keyword::Try),
        Keyword(kws_rs::Keyword::Typeof),
        Keyword(kws_rs::Keyword::Unsized),
        Keyword(kws_rs::Keyword::Virtual),
        Keyword(kws_rs::Keyword::Yield),
    ]
        .into_iter()
        .collect()
);

static RUST_2018_WEAK: ExpectedKeywords = LazyLock::new(||
    [
        Keyword(kws_rs::Keyword::MacroRules),
        Keyword(kws_rs::Keyword::Raw),
        Keyword(kws_rs::Keyword::Safe),
        Keyword(kws_rs::Keyword::StaticLifetime),
        Keyword(kws_rs::Keyword::Union),
    ]
        .into_iter()
        .collect()
);

static RUST_2021_STRICT: ExpectedKeywords = LazyLock::new(||
    [
        Keyword(kws_rs::Keyword::As),
        Keyword(kws_rs::Keyword::Async),
        Keyword(kws_rs::Keyword::Await),
        Keyword(kws_rs::Keyword::Break),
        Keyword(kws_rs::Keyword::Const),
        Keyword(kws_rs::Keyword::Continue),
        Keyword(kws_rs::Keyword::Crate),
        Keyword(kws_rs::Keyword::Dyn),
        Keyword(kws_rs::Keyword::Else),
        Keyword(kws_rs::Keyword::Enum),
        Keyword(kws_rs::Keyword::Extern),
        Keyword(kws_rs::Keyword::False),
        Keyword(kws_rs::Keyword::Fn),
        Keyword(kws_rs::Keyword::For),
        Keyword(kws_rs::Keyword::If),
        Keyword(kws_rs::Keyword::Impl),
        Keyword(kws_rs::Keyword::In),
        Keyword(kws_rs::Keyword::Let),
        Keyword(kws_rs::Keyword::Loop),
        Keyword(kws_rs::Keyword::Match),
        Keyword(kws_rs::Keyword::Mod),
        Keyword(kws_rs::Keyword::Move),
        Keyword(kws_rs::Keyword::Mut),
        Keyword(kws_rs::Keyword::Pub),
        Keyword(kws_rs::Keyword::Ref),
        Keyword(kws_rs::Keyword::Return),
        Keyword(kws_rs::Keyword::SelfValue),
        Keyword(kws_rs::Keyword::SelfType),
        Keyword(kws_rs::Keyword::Static),
        Keyword(kws_rs::Keyword::Struct),
        Keyword(kws_rs::Keyword::Super),
        Keyword(kws_rs::Keyword::Trait),
        Keyword(kws_rs::Keyword::True),
        Keyword(kws_rs::Keyword::Type),
        Keyword(kws_rs::Keyword::Unsafe),
        Keyword(kws_rs::Keyword::Use),
        Keyword(kws_rs::Keyword::Where),
        Keyword(kws_rs::Keyword::While),
    ]
        .into_iter()
        .collect()
);

static RUST_2021_RESERVED: ExpectedKeywords = LazyLock::new(||
    [
        Keyword(kws_rs::Keyword::Abstract),
        Keyword(kws_rs::Keyword::Become),
        Keyword(kws_rs::Keyword::Box),
        Keyword(kws_rs::Keyword::Do),
        Keyword(kws_rs::Keyword::Final),
        Keyword(kws_rs::Keyword::Macro),
        Keyword(kws_rs::Keyword::Override),
        Keyword(kws_rs::Keyword::Priv),
        Keyword(kws_rs::Keyword::Try),
        Keyword(kws_rs::Keyword::Typeof),
        Keyword(kws_rs::Keyword::Unsized),
        Keyword(kws_rs::Keyword::Virtual),
        Keyword(kws_rs::Keyword::Yield),
    ]
        .into_iter()
        .collect()
);

static RUST_2021_WEAK: ExpectedKeywords = LazyLock::new(||
    [
        Keyword(kws_rs::Keyword::MacroRules),
        Keyword(kws_rs::Keyword::Raw),
        Keyword(kws_rs::Keyword::Safe),
        Keyword(kws_rs::Keyword::StaticLifetime),
        Keyword(kws_rs::Keyword::Union),
    ]
        .into_iter()
        .collect()
);

static RUST_2024_STRICT: ExpectedKeywords = LazyLock::new(||
    [
        Keyword(kws_rs::Keyword::As),
        Keyword(kws_rs::Keyword::Async),
        Keyword(kws_rs::Keyword::Await),
        Keyword(kws_rs::Keyword::Break),
        Keyword(kws_rs::Keyword::Const),
        Keyword(kws_rs::Keyword::Continue),
        Keyword(kws_rs::Keyword::Crate),
        Keyword(kws_rs::Keyword::Dyn),
        Keyword(kws_rs::Keyword::Else),
        Keyword(kws_rs::Keyword::Enum),
        Keyword(kws_rs::Keyword::Extern),
        Keyword(kws_rs::Keyword::False),
        Keyword(kws_rs::Keyword::Fn),
        Keyword(kws_rs::Keyword::For),
        Keyword(kws_rs::Keyword::If),
        Keyword(kws_rs::Keyword::Impl),
        Keyword(kws_rs::Keyword::In),
        Keyword(kws_rs::Keyword::Let),
        Keyword(kws_rs::Keyword::Loop),
        Keyword(kws_rs::Keyword::Match),
        Keyword(kws_rs::Keyword::Mod),
        Keyword(kws_rs::Keyword::Move),
        Keyword(kws_rs::Keyword::Mut),
        Keyword(kws_rs::Keyword::Pub),
        Keyword(kws_rs::Keyword::Ref),
        Keyword(kws_rs::Keyword::Return),
        Keyword(kws_rs::Keyword::SelfValue),
        Keyword(kws_rs::Keyword::SelfType),
        Keyword(kws_rs::Keyword::Static),
        Keyword(kws_rs::Keyword::Struct),
        Keyword(kws_rs::Keyword::Super),
        Keyword(kws_rs::Keyword::Trait),
        Keyword(kws_rs::Keyword::True),
        Keyword(kws_rs::Keyword::Type),
        Keyword(kws_rs::Keyword::Unsafe),
        Keyword(kws_rs::Keyword::Use),
        Keyword(kws_rs::Keyword::Where),
        Keyword(kws_rs::Keyword::While),
    ]
        .into_iter()
        .collect()
);

static RUST_2024_RESERVED: ExpectedKeywords = LazyLock::new(||
    [
        Keyword(kws_rs::Keyword::Abstract),
        Keyword(kws_rs::Keyword::Become),
        Keyword(kws_rs::Keyword::Box),
        Keyword(kws_rs::Keyword::Do),
        Keyword(kws_rs::Keyword::Final),
        Keyword(kws_rs::Keyword::Gen),
        Keyword(kws_rs::Keyword::Macro),
        Keyword(kws_rs::Keyword::Override),
        Keyword(kws_rs::Keyword::Priv),
        Keyword(kws_rs::Keyword::Try),
        Keyword(kws_rs::Keyword::Typeof),
        Keyword(kws_rs::Keyword::Unsized),
        Keyword(kws_rs::Keyword::Virtual),
        Keyword(kws_rs::Keyword::Yield),
    ]
        .into_iter()
        .collect()
);

static RUST_2024_WEAK: ExpectedKeywords = LazyLock::new(||
    [
        Keyword(kws_rs::Keyword::MacroRules),
        Keyword(kws_rs::Keyword::Raw),
        Keyword(kws_rs::Keyword::Safe),
        Keyword(kws_rs::Keyword::StaticLifetime),
        Keyword(kws_rs::Keyword::Union),
    ]
        .into_iter()
        .collect()
);
