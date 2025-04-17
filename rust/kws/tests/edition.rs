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
    println!("STUFF");
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
        Keyword::As(&kws::KW_AS_DATA),
        Keyword::Break(&kws::KW_BREAK_DATA),
        Keyword::Const(&kws::KW_CONST_DATA),
        Keyword::Continue(&kws::KW_CONTINUE_DATA),
        Keyword::Crate(&kws::KW_CRATE_DATA),
        Keyword::Else(&kws::KW_ELSE_DATA),
        Keyword::Enum(&kws::KW_ENUM_DATA),
        Keyword::Extern(&kws::KW_EXTERN_DATA),
        Keyword::False(&kws::KW_FALSE_DATA),
        Keyword::Fn(&kws::KW_FN_DATA),
        Keyword::For(&kws::KW_FOR_DATA),
        Keyword::If(&kws::KW_IF_DATA),
        Keyword::Impl(&kws::KW_IMPL_DATA),
        Keyword::In(&kws::KW_IN_DATA),
        Keyword::Let(&kws::KW_LET_DATA),
        Keyword::Loop(&kws::KW_LOOP_DATA),
        Keyword::Match(&kws::KW_MATCH_DATA),
        Keyword::Mod(&kws::KW_MOD_DATA),
        Keyword::Move(&kws::KW_MOVE_DATA),
        Keyword::Mut(&kws::KW_MUT_DATA),
        Keyword::Pub(&kws::KW_PUB_DATA),
        Keyword::Ref(&kws::KW_REF_DATA),
        Keyword::Return(&kws::KW_RETURN_DATA),
        Keyword::SelfValue(&kws::KW_SELF_VALUE_DATA),
        Keyword::SelfType(&kws::KW_SELF_TYPE_DATA),
        Keyword::Static(&kws::KW_STATIC_DATA),
        Keyword::Struct(&kws::KW_STRUCT_DATA),
        Keyword::Super(&kws::KW_SUPER_DATA),
        Keyword::Trait(&kws::KW_TRAIT_DATA),
        Keyword::True(&kws::KW_TRUE_DATA),
        Keyword::Type(&kws::KW_TYPE_DATA),
        Keyword::Unsafe(&kws::KW_UNSAFE_DATA),
        Keyword::Use(&kws::KW_USE_DATA),
        Keyword::Where(&kws::KW_WHERE_DATA),
        Keyword::While(&kws::KW_WHILE_DATA),
    ].into_iter().collect()
);

static RUST_2015_KEYWORDS_RESERVED: ExpectedKeywords = LazyLock::new(||
    [
        Keyword::Abstract(&kws::KW_ABSTRACT_DATA),
        Keyword::Become(&kws::KW_BECOME_DATA),
        Keyword::Box(&kws::KW_BOX_DATA),
        Keyword::Do(&kws::KW_DO_DATA),
        Keyword::Final(&kws::KW_FINAL_DATA),
        Keyword::Macro(&kws::KW_MACRO_DATA),
        Keyword::Override(&kws::KW_OVERRIDE_DATA),
        Keyword::Priv(&kws::KW_PRIV_DATA),
        Keyword::Typeof(&kws::KW_TYPEOF_DATA),
        Keyword::Unsized(&kws::KW_UNSIZED_DATA),
        Keyword::Virtual(&kws::KW_VIRTUAL_DATA),
        Keyword::Yield(&kws::KW_YIELD_DATA),
    ].into_iter().collect()
);

static RUST_2015_KEYWORDS_WEAK: ExpectedKeywords = LazyLock::new(||
    [
        Keyword::Dyn(&kws::KW_DYN_DATA),
        Keyword::MacroRules(&kws::KW_MACRO_RULES_DATA),
        Keyword::Raw(&kws::KW_RAW_DATA),
        Keyword::Safe(&kws::KW_SAFE_DATA),
        Keyword::StaticLifetime(&kws::KW_STATIC_LIFETIME_DATA),
        Keyword::Union(&kws::KW_UNION_DATA),
    ].into_iter().collect()
);

static RUST_2018_KEYWORDS_STRICT: ExpectedKeywords = LazyLock::new(||
    [
        Keyword::As(&kws::KW_AS_DATA),
        Keyword::Async(&kws::KW_ASYNC_DATA),
        Keyword::Await(&kws::KW_AWAIT_DATA),
        Keyword::Break(&kws::KW_BREAK_DATA),
        Keyword::Const(&kws::KW_CONST_DATA),
        Keyword::Continue(&kws::KW_CONTINUE_DATA),
        Keyword::Crate(&kws::KW_CRATE_DATA),
        Keyword::Dyn(&kws::KW_DYN_DATA),
        Keyword::Else(&kws::KW_ELSE_DATA),
        Keyword::Enum(&kws::KW_ENUM_DATA),
        Keyword::Extern(&kws::KW_EXTERN_DATA),
        Keyword::False(&kws::KW_FALSE_DATA),
        Keyword::Fn(&kws::KW_FN_DATA),
        Keyword::For(&kws::KW_FOR_DATA),
        Keyword::If(&kws::KW_IF_DATA),
        Keyword::Impl(&kws::KW_IMPL_DATA),
        Keyword::In(&kws::KW_IN_DATA),
        Keyword::Let(&kws::KW_LET_DATA),
        Keyword::Loop(&kws::KW_LOOP_DATA),
        Keyword::Match(&kws::KW_MATCH_DATA),
        Keyword::Mod(&kws::KW_MOD_DATA),
        Keyword::Move(&kws::KW_MOVE_DATA),
        Keyword::Mut(&kws::KW_MUT_DATA),
        Keyword::Pub(&kws::KW_PUB_DATA),
        Keyword::Ref(&kws::KW_REF_DATA),
        Keyword::Return(&kws::KW_RETURN_DATA),
        Keyword::SelfValue(&kws::KW_SELF_VALUE_DATA),
        Keyword::SelfType(&kws::KW_SELF_TYPE_DATA),
        Keyword::Static(&kws::KW_STATIC_DATA),
        Keyword::Struct(&kws::KW_STRUCT_DATA),
        Keyword::Super(&kws::KW_SUPER_DATA),
        Keyword::Trait(&kws::KW_TRAIT_DATA),
        Keyword::True(&kws::KW_TRUE_DATA),
        Keyword::Type(&kws::KW_TYPE_DATA),
        Keyword::Unsafe(&kws::KW_UNSAFE_DATA),
        Keyword::Use(&kws::KW_USE_DATA),
        Keyword::Where(&kws::KW_WHERE_DATA),
        Keyword::While(&kws::KW_WHILE_DATA),
    ].into_iter().collect()
);

static RUST_2018_KEYWORDS_RESERVED: ExpectedKeywords = LazyLock::new(||
    [
        Keyword::Abstract(&kws::KW_ABSTRACT_DATA),
        Keyword::Become(&kws::KW_BECOME_DATA),
        Keyword::Box(&kws::KW_BOX_DATA),
        Keyword::Do(&kws::KW_DO_DATA),
        Keyword::Final(&kws::KW_FINAL_DATA),
        Keyword::Macro(&kws::KW_MACRO_DATA),
        Keyword::Override(&kws::KW_OVERRIDE_DATA),
        Keyword::Priv(&kws::KW_PRIV_DATA),
        Keyword::Try(&kws::KW_TRY_DATA),
        Keyword::Typeof(&kws::KW_TYPEOF_DATA),
        Keyword::Unsized(&kws::KW_UNSIZED_DATA),
        Keyword::Virtual(&kws::KW_VIRTUAL_DATA),
        Keyword::Yield(&kws::KW_YIELD_DATA),
    ].into_iter().collect()
);

static RUST_2018_KEYWORDS_WEAK: ExpectedKeywords = LazyLock::new(||
    [
        Keyword::MacroRules(&kws::KW_MACRO_RULES_DATA),
        Keyword::Raw(&kws::KW_RAW_DATA),
        Keyword::Safe(&kws::KW_SAFE_DATA),
        Keyword::StaticLifetime(&kws::KW_STATIC_LIFETIME_DATA),
        Keyword::Union(&kws::KW_UNION_DATA),
    ].into_iter().collect()
);

static RUST_2021_KEYWORDS_STRICT: ExpectedKeywords = LazyLock::new(||
    [
        Keyword::As(&kws::KW_AS_DATA),
        Keyword::Async(&kws::KW_ASYNC_DATA),
        Keyword::Await(&kws::KW_AWAIT_DATA),
        Keyword::Break(&kws::KW_BREAK_DATA),
        Keyword::Const(&kws::KW_CONST_DATA),
        Keyword::Continue(&kws::KW_CONTINUE_DATA),
        Keyword::Crate(&kws::KW_CRATE_DATA),
        Keyword::Dyn(&kws::KW_DYN_DATA),
        Keyword::Else(&kws::KW_ELSE_DATA),
        Keyword::Enum(&kws::KW_ENUM_DATA),
        Keyword::Extern(&kws::KW_EXTERN_DATA),
        Keyword::False(&kws::KW_FALSE_DATA),
        Keyword::Fn(&kws::KW_FN_DATA),
        Keyword::For(&kws::KW_FOR_DATA),
        Keyword::If(&kws::KW_IF_DATA),
        Keyword::Impl(&kws::KW_IMPL_DATA),
        Keyword::In(&kws::KW_IN_DATA),
        Keyword::Let(&kws::KW_LET_DATA),
        Keyword::Loop(&kws::KW_LOOP_DATA),
        Keyword::Match(&kws::KW_MATCH_DATA),
        Keyword::Mod(&kws::KW_MOD_DATA),
        Keyword::Move(&kws::KW_MOVE_DATA),
        Keyword::Mut(&kws::KW_MUT_DATA),
        Keyword::Pub(&kws::KW_PUB_DATA),
        Keyword::Ref(&kws::KW_REF_DATA),
        Keyword::Return(&kws::KW_RETURN_DATA),
        Keyword::SelfValue(&kws::KW_SELF_VALUE_DATA),
        Keyword::SelfType(&kws::KW_SELF_TYPE_DATA),
        Keyword::Static(&kws::KW_STATIC_DATA),
        Keyword::Struct(&kws::KW_STRUCT_DATA),
        Keyword::Super(&kws::KW_SUPER_DATA),
        Keyword::Trait(&kws::KW_TRAIT_DATA),
        Keyword::True(&kws::KW_TRUE_DATA),
        Keyword::Type(&kws::KW_TYPE_DATA),
        Keyword::Unsafe(&kws::KW_UNSAFE_DATA),
        Keyword::Use(&kws::KW_USE_DATA),
        Keyword::Where(&kws::KW_WHERE_DATA),
        Keyword::While(&kws::KW_WHILE_DATA),
    ].into_iter().collect()
);

static RUST_2021_KEYWORDS_RESERVED: ExpectedKeywords = LazyLock::new(||
    [
        Keyword::Abstract(&kws::KW_ABSTRACT_DATA),
        Keyword::Become(&kws::KW_BECOME_DATA),
        Keyword::Box(&kws::KW_BOX_DATA),
        Keyword::Do(&kws::KW_DO_DATA),
        Keyword::Final(&kws::KW_FINAL_DATA),
        Keyword::Macro(&kws::KW_MACRO_DATA),
        Keyword::Override(&kws::KW_OVERRIDE_DATA),
        Keyword::Priv(&kws::KW_PRIV_DATA),
        Keyword::Try(&kws::KW_TRY_DATA),
        Keyword::Typeof(&kws::KW_TYPEOF_DATA),
        Keyword::Unsized(&kws::KW_UNSIZED_DATA),
        Keyword::Virtual(&kws::KW_VIRTUAL_DATA),
        Keyword::Yield(&kws::KW_YIELD_DATA),
    ].into_iter().collect()
);

static RUST_2021_KEYWORDS_WEAK: ExpectedKeywords = LazyLock::new(||
    [
        Keyword::MacroRules(&kws::KW_MACRO_RULES_DATA),
        Keyword::Raw(&kws::KW_RAW_DATA),
        Keyword::Safe(&kws::KW_SAFE_DATA),
        Keyword::StaticLifetime(&kws::KW_STATIC_LIFETIME_DATA),
        Keyword::Union(&kws::KW_UNION_DATA),
    ].into_iter().collect()
);

static RUST_2024_KEYWORDS_STRICT: ExpectedKeywords = LazyLock::new(||
    [
        Keyword::As(&kws::KW_AS_DATA),
        Keyword::Async(&kws::KW_ASYNC_DATA),
        Keyword::Await(&kws::KW_AWAIT_DATA),
        Keyword::Break(&kws::KW_BREAK_DATA),
        Keyword::Const(&kws::KW_CONST_DATA),
        Keyword::Continue(&kws::KW_CONTINUE_DATA),
        Keyword::Crate(&kws::KW_CRATE_DATA),
        Keyword::Dyn(&kws::KW_DYN_DATA),
        Keyword::Else(&kws::KW_ELSE_DATA),
        Keyword::Enum(&kws::KW_ENUM_DATA),
        Keyword::Extern(&kws::KW_EXTERN_DATA),
        Keyword::False(&kws::KW_FALSE_DATA),
        Keyword::Fn(&kws::KW_FN_DATA),
        Keyword::For(&kws::KW_FOR_DATA),
        Keyword::If(&kws::KW_IF_DATA),
        Keyword::Impl(&kws::KW_IMPL_DATA),
        Keyword::In(&kws::KW_IN_DATA),
        Keyword::Let(&kws::KW_LET_DATA),
        Keyword::Loop(&kws::KW_LOOP_DATA),
        Keyword::Match(&kws::KW_MATCH_DATA),
        Keyword::Mod(&kws::KW_MOD_DATA),
        Keyword::Move(&kws::KW_MOVE_DATA),
        Keyword::Mut(&kws::KW_MUT_DATA),
        Keyword::Pub(&kws::KW_PUB_DATA),
        Keyword::Ref(&kws::KW_REF_DATA),
        Keyword::Return(&kws::KW_RETURN_DATA),
        Keyword::SelfValue(&kws::KW_SELF_VALUE_DATA),
        Keyword::SelfType(&kws::KW_SELF_TYPE_DATA),
        Keyword::Static(&kws::KW_STATIC_DATA),
        Keyword::Struct(&kws::KW_STRUCT_DATA),
        Keyword::Super(&kws::KW_SUPER_DATA),
        Keyword::Trait(&kws::KW_TRAIT_DATA),
        Keyword::True(&kws::KW_TRUE_DATA),
        Keyword::Type(&kws::KW_TYPE_DATA),
        Keyword::Unsafe(&kws::KW_UNSAFE_DATA),
        Keyword::Use(&kws::KW_USE_DATA),
        Keyword::Where(&kws::KW_WHERE_DATA),
        Keyword::While(&kws::KW_WHILE_DATA),
    ].into_iter().collect()
);

static RUST_2024_KEYWORDS_RESERVED: ExpectedKeywords = LazyLock::new(||
    [
        Keyword::Abstract(&kws::KW_ABSTRACT_DATA),
        Keyword::Become(&kws::KW_BECOME_DATA),
        Keyword::Box(&kws::KW_BOX_DATA),
        Keyword::Do(&kws::KW_DO_DATA),
        Keyword::Final(&kws::KW_FINAL_DATA),
        Keyword::Gen(&kws::KW_GEN_DATA),
        Keyword::Macro(&kws::KW_MACRO_DATA),
        Keyword::Override(&kws::KW_OVERRIDE_DATA),
        Keyword::Priv(&kws::KW_PRIV_DATA),
        Keyword::Try(&kws::KW_TRY_DATA),
        Keyword::Typeof(&kws::KW_TYPEOF_DATA),
        Keyword::Unsized(&kws::KW_UNSIZED_DATA),
        Keyword::Virtual(&kws::KW_VIRTUAL_DATA),
        Keyword::Yield(&kws::KW_YIELD_DATA),
    ].into_iter().collect()
);

static RUST_2024_KEYWORDS_WEAK: ExpectedKeywords = LazyLock::new(||
    [
        Keyword::MacroRules(&kws::KW_MACRO_RULES_DATA),
        Keyword::Raw(&kws::KW_RAW_DATA),
        Keyword::Safe(&kws::KW_SAFE_DATA),
        Keyword::StaticLifetime(&kws::KW_STATIC_LIFETIME_DATA),
        Keyword::Union(&kws::KW_UNION_DATA),
    ].into_iter().collect()
);
