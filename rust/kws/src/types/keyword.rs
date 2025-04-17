pub(crate) mod kw_abstract;
pub(crate) mod kw_as;
pub(crate) mod kw_async;
pub(crate) mod kw_await;
pub(crate) mod kw_become;
pub(crate) mod kw_box;
pub(crate) mod kw_break;
pub(crate) mod kw_const;
pub(crate) mod kw_continue;
pub(crate) mod kw_crate;
pub(crate) mod kw_do;
pub(crate) mod kw_dyn;
pub(crate) mod kw_else;
pub(crate) mod kw_enum;
pub(crate) mod kw_extern;
pub(crate) mod kw_false;
pub(crate) mod kw_final;
pub(crate) mod kw_fn;
pub(crate) mod kw_for;
pub(crate) mod kw_gen;
pub(crate) mod kw_if;
pub(crate) mod kw_impl;
pub(crate) mod kw_in;
pub(crate) mod kw_let;
pub(crate) mod kw_loop;
pub(crate) mod kw_macro;
pub(crate) mod kw_macro_rules;
pub(crate) mod kw_match;
pub(crate) mod kw_mod;
pub(crate) mod kw_move;
pub(crate) mod kw_mut;
pub(crate) mod kw_override;
pub(crate) mod kw_priv;
pub(crate) mod kw_pub;
pub(crate) mod kw_raw;
pub(crate) mod kw_ref;
pub(crate) mod kw_return;
pub(crate) mod kw_safe;
pub(crate) mod kw_self_value;
pub(crate) mod kw_self_type;
pub(crate) mod kw_static;
pub(crate) mod kw_static_lifetime;
pub(crate) mod kw_struct;
pub(crate) mod kw_super;
pub(crate) mod kw_trait;
pub(crate) mod kw_true;
pub(crate) mod kw_try;
pub(crate) mod kw_type;
pub(crate) mod kw_typeof;
pub(crate) mod kw_union;
pub(crate) mod kw_unsafe;
pub(crate) mod kw_unsized;
pub(crate) mod kw_use;
pub(crate) mod kw_virtual;
pub(crate) mod kw_where;
pub(crate) mod kw_while;
pub(crate) mod kw_yield;


use crate::{
    error::Error,
    types::keyword_data::KeywordData,
};
use std::ops::Deref;
use strum::IntoEnumIterator;
use strum_macros::{
    EnumDiscriminants,
    EnumIter,
};


#[derive(Debug, EnumDiscriminants, Eq, Hash, PartialEq)]
#[strum_discriminants(derive(EnumIter))]
pub enum Keyword {
    Abstract(&'static KeywordData),
    As(&'static KeywordData),
    Async(&'static KeywordData),
    Await(&'static KeywordData),
    Become(&'static KeywordData),
    Box(&'static KeywordData),
    Break(&'static KeywordData),
    Const(&'static KeywordData),
    Continue(&'static KeywordData),
    Crate(&'static KeywordData),
    Do(&'static KeywordData),
    Dyn(&'static KeywordData),
    Else(&'static KeywordData),
    Enum(&'static KeywordData),
    Extern(&'static KeywordData),
    False(&'static KeywordData),
    Final(&'static KeywordData),
    Fn(&'static KeywordData),
    For(&'static KeywordData),
    Gen(&'static KeywordData),
    If(&'static KeywordData),
    Impl(&'static KeywordData),
    In(&'static KeywordData),
    Let(&'static KeywordData),
    Loop(&'static KeywordData),
    Macro(&'static KeywordData),
    MacroRules(&'static KeywordData),
    Match(&'static KeywordData),
    Mod(&'static KeywordData),
    Move(&'static KeywordData),
    Mut(&'static KeywordData),
    Override(&'static KeywordData),
    Priv(&'static KeywordData),
    Pub(&'static KeywordData),
    Raw(&'static KeywordData),
    Ref(&'static KeywordData),
    Return(&'static KeywordData),
    Safe(&'static KeywordData),
    SelfValue(&'static KeywordData),
    SelfType(&'static KeywordData),
    Static(&'static KeywordData),
    StaticLifetime(&'static KeywordData),
    Struct(&'static KeywordData),
    Super(&'static KeywordData),
    Trait(&'static KeywordData),
    True(&'static KeywordData),
    Try(&'static KeywordData),
    Type(&'static KeywordData),
    Typeof(&'static KeywordData),
    Union(&'static KeywordData),
    Unsafe(&'static KeywordData),
    Unsized(&'static KeywordData),
    Use(&'static KeywordData),
    Virtual(&'static KeywordData),
    Where(&'static KeywordData),
    While(&'static KeywordData),
    Yield(&'static KeywordData),
}

impl Keyword {
    pub fn data(&self) -> &'static KeywordData {
        match self {
            Self::Abstract(data) |
            Self::As(data) |
            Self::Async(data) |
            Self::Await(data) |
            Self::Become(data) |
            Self::Box(data) |
            Self::Break(data) |
            Self::Const(data) |
            Self::Continue(data) |
            Self::Crate(data) |
            Self::Do(data) |
            Self::Dyn(data) |
            Self::Else(data) |
            Self::Enum(data) |
            Self::Extern(data) |
            Self::False(data) |
            Self::Final(data) |
            Self::Fn(data) |
            Self::For(data) |
            Self::Gen(data) |
            Self::If(data) |
            Self::Impl(data) |
            Self::In(data) |
            Self::Let(data) |
            Self::Loop(data) |
            Self::Macro(data) |
            Self::MacroRules(data) |
            Self::Match(data) |
            Self::Mod(data) |
            Self::Move(data) |
            Self::Mut(data) |
            Self::Override(data) |
            Self::Priv(data) |
            Self::Pub(data) |
            Self::Raw(data) |
            Self::Ref(data) |
            Self::Return(data) |
            Self::Safe(data) |
            Self::SelfValue(data) |
            Self::SelfType(data) |
            Self::Static(data) |
            Self::StaticLifetime(data) |
            Self::Struct(data) |
            Self::Super(data) |
            Self::Trait(data) |
            Self::True(data) |
            Self::Try(data) |
            Self::Type(data) |
            Self::Typeof(data) |
            Self::Union(data) |
            Self::Unsafe(data) |
            Self::Unsized(data) |
            Self::Use(data) |
            Self::Virtual(data) |
            Self::Where(data) |
            Self::While(data) |
            Self::Yield(data) => data,
        }
    }

    pub fn iter() -> impl Iterator<Item = Self> {
        KeywordDiscriminants::iter()
            .map(|discriminant| (&discriminant).into())
    }
}

impl Deref for Keyword {
    type Target = KeywordData;


    fn deref(&self) -> &Self::Target {
        self.data()
    }
}

impl From<&KeywordDiscriminants> for Keyword {
    fn from(value: &KeywordDiscriminants) -> Self {
        match value {
            KeywordDiscriminants::Abstract =>
                Self::Abstract(&kw_abstract::DATA),
            KeywordDiscriminants::As =>
                Self::As(&kw_as::DATA),
            KeywordDiscriminants::Async =>
                Self::Async(&kw_async::DATA),
            KeywordDiscriminants::Await =>
                Self::Await(&kw_await::DATA),
            KeywordDiscriminants::Become =>
                Self::Become(&kw_become::DATA),
            KeywordDiscriminants::Box =>
                Self::Box(&kw_box::DATA),
            KeywordDiscriminants::Break =>
                Self::Break(&kw_break::DATA),
            KeywordDiscriminants::Const =>
                Self::Const(&kw_const::DATA),
            KeywordDiscriminants::Continue =>
                Self::Continue(&kw_continue::DATA),
            KeywordDiscriminants::Crate =>
                Self::Crate(&kw_crate::DATA),
            KeywordDiscriminants::Do =>
                Self::Do(&kw_do::DATA),
            KeywordDiscriminants::Dyn =>
                Self::Dyn(&kw_dyn::DATA),
            KeywordDiscriminants::Else =>
                Self::Else(&kw_else::DATA),
            KeywordDiscriminants::Enum =>
                Self::Enum(&kw_enum::DATA),
            KeywordDiscriminants::Extern =>
                Self::Extern(&kw_extern::DATA),
            KeywordDiscriminants::False =>
                Self::False(&kw_false::DATA),
            KeywordDiscriminants::Final =>
                Self::Final(&kw_final::DATA),
            KeywordDiscriminants::Fn =>
                Self::Fn(&kw_fn::DATA),
            KeywordDiscriminants::For =>
                Self::For(&kw_for::DATA),
            KeywordDiscriminants::Gen =>
                Self::Gen(&kw_gen::DATA),
            KeywordDiscriminants::If =>
                Self::If(&kw_if::DATA),
            KeywordDiscriminants::Impl =>
                Self::Impl(&kw_impl::DATA),
            KeywordDiscriminants::In =>
                Self::In(&kw_in::DATA),
            KeywordDiscriminants::Let =>
                Self::Let(&kw_let::DATA),
            KeywordDiscriminants::Loop =>
                Self::Loop(&kw_loop::DATA),
            KeywordDiscriminants::Macro =>
                Self::Macro(&kw_macro::DATA),
            KeywordDiscriminants::MacroRules =>
                Self::MacroRules(&kw_macro_rules::DATA),
            KeywordDiscriminants::Match =>
                Self::Match(&kw_match::DATA),
            KeywordDiscriminants::Mod =>
                Self::Mod(&kw_mod::DATA),
            KeywordDiscriminants::Move =>
                Self::Move(&kw_move::DATA),
            KeywordDiscriminants::Mut =>
                Self::Mut(&kw_mut::DATA),
            KeywordDiscriminants::Override =>
                Self::Override(&kw_override::DATA),
            KeywordDiscriminants::Priv =>
                Self::Priv(&kw_priv::DATA),
            KeywordDiscriminants::Pub =>
                Self::Pub(&kw_pub::DATA),
            KeywordDiscriminants::Raw =>
                Self::Raw(&kw_raw::DATA),
            KeywordDiscriminants::Ref =>
                Self::Ref(&kw_ref::DATA),
            KeywordDiscriminants::Return =>
                Self::Return(&kw_return::DATA),
            KeywordDiscriminants::Safe =>
                Self::Safe(&kw_safe::DATA),
            KeywordDiscriminants::SelfValue =>
                Self::SelfValue(&kw_self_value::DATA),
            KeywordDiscriminants::SelfType =>
                Self::SelfType(&kw_self_type::DATA),
            KeywordDiscriminants::Static =>
                Self::Static(&kw_static::DATA),
            KeywordDiscriminants::StaticLifetime =>
                Self::StaticLifetime(&kw_static_lifetime::DATA),
            KeywordDiscriminants::Struct =>
                Self::Struct(&kw_struct::DATA),
            KeywordDiscriminants::Super =>
                Self::Super(&kw_super::DATA),
            KeywordDiscriminants::Trait =>
                Self::Trait(&kw_trait::DATA),
            KeywordDiscriminants::True =>
                Self::True(&kw_true::DATA),
            KeywordDiscriminants::Try =>
                Self::Try(&kw_try::DATA),
            KeywordDiscriminants::Type =>
                Self::Type(&kw_type::DATA),
            KeywordDiscriminants::Typeof =>
                Self::Typeof(&kw_typeof::DATA),
            KeywordDiscriminants::Union =>
                Self::Union(&kw_union::DATA),
            KeywordDiscriminants::Unsafe =>
                Self::Unsafe(&kw_unsafe::DATA),
            KeywordDiscriminants::Unsized =>
                Self::Unsized(&kw_unsized::DATA),
            KeywordDiscriminants::Use =>
                Self::Use(&kw_use::DATA),
            KeywordDiscriminants::Virtual =>
                Self::Virtual(&kw_virtual::DATA),
            KeywordDiscriminants::Where =>
                Self::Where(&kw_where::DATA),
            KeywordDiscriminants::While =>
                Self::While(&kw_while::DATA),
            KeywordDiscriminants::Yield =>
                Self::Yield(&kw_yield::DATA),
        }
    }
}

impl TryFrom<&str> for Keyword {
    type Error = Error;


    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            value if value == kw_abstract::DATA.value =>
                Ok(Self::Abstract(&kw_abstract::DATA)),
            value if value == kw_as::DATA.value =>
                Ok(Self::As(&kw_as::DATA)),
            value if value == kw_async::DATA.value =>
                Ok(Self::Async(&kw_async::DATA)),
            value if value == kw_await::DATA.value =>
                Ok(Self::Await(&kw_await::DATA)),
            value if value == kw_become::DATA.value =>
                Ok(Self::Become(&kw_become::DATA)),
            value if value == kw_box::DATA.value =>
                Ok(Self::Box(&kw_box::DATA)),
            value if value == kw_break::DATA.value =>
                Ok(Self::Break(&kw_break::DATA)),
            value if value == kw_const::DATA.value =>
                Ok(Self::Const(&kw_const::DATA)),
            value if value == kw_continue::DATA.value =>
                Ok(Self::Continue(&kw_continue::DATA)),
            value if value == kw_crate::DATA.value =>
                Ok(Self::Crate(&kw_crate::DATA)),
            value if value == kw_do::DATA.value =>
                Ok(Self::Do(&kw_do::DATA)),
            value if value == kw_dyn::DATA.value =>
                Ok(Self::Dyn(&kw_dyn::DATA)),
            value if value == kw_else::DATA.value =>
                Ok(Self::Else(&kw_else::DATA)),
            value if value == kw_enum::DATA.value =>
                Ok(Self::Enum(&kw_enum::DATA)),
            value if value == kw_extern::DATA.value =>
                Ok(Self::Extern(&kw_extern::DATA)),
            value if value == kw_false::DATA.value =>
                Ok(Self::False(&kw_false::DATA)),
            value if value == kw_final::DATA.value =>
                Ok(Self::Final(&kw_final::DATA)),
            value if value == kw_fn::DATA.value =>
                Ok(Self::Fn(&kw_fn::DATA)),
            value if value == kw_for::DATA.value =>
                Ok(Self::For(&kw_for::DATA)),
            value if value == kw_gen::DATA.value =>
                Ok(Self::Gen(&kw_gen::DATA)),
            value if value == kw_if::DATA.value =>
                Ok(Self::If(&kw_if::DATA)),
            value if value == kw_impl::DATA.value =>
                Ok(Self::Impl(&kw_impl::DATA)),
            value if value == kw_in::DATA.value =>
                Ok(Self::In(&kw_in::DATA)),
            value if value == kw_let::DATA.value =>
                Ok(Self::Let(&kw_let::DATA)),
            value if value == kw_loop::DATA.value =>
                Ok(Self::Loop(&kw_loop::DATA)),
            value if value == kw_macro::DATA.value =>
                Ok(Self::Macro(&kw_macro::DATA)),
            value if value == kw_macro_rules::DATA.value =>
                Ok(Self::MacroRules(&kw_macro_rules::DATA)),
            value if value == kw_match::DATA.value =>
                Ok(Self::Match(&kw_match::DATA)),
            value if value == kw_mod::DATA.value =>
                Ok(Self::Mod(&kw_mod::DATA)),
            value if value == kw_move::DATA.value =>
                Ok(Self::Move(&kw_move::DATA)),
            value if value == kw_mut::DATA.value =>
                Ok(Self::Mut(&kw_mut::DATA)),
            value if value == kw_override::DATA.value =>
                Ok(Self::Override(&kw_override::DATA)),
            value if value == kw_priv::DATA.value =>
                Ok(Self::Priv(&kw_priv::DATA)),
            value if value == kw_pub::DATA.value =>
                Ok(Self::Pub(&kw_pub::DATA)),
            value if value == kw_raw::DATA.value =>
                Ok(Self::Raw(&kw_raw::DATA)),
            value if value == kw_ref::DATA.value =>
                Ok(Self::Ref(&kw_ref::DATA)),
            value if value == kw_return::DATA.value =>
                Ok(Self::Return(&kw_return::DATA)),
            value if value == kw_safe::DATA.value =>
                Ok(Self::Safe(&kw_safe::DATA)),
            value if value == kw_self_value::DATA.value =>
                Ok(Self::SelfValue(&kw_self_value::DATA)),
            value if value == kw_self_type::DATA.value =>
                Ok(Self::SelfType(&kw_self_type::DATA)),
            value if value == kw_static::DATA.value =>
                Ok(Self::Static(&kw_static::DATA)),
            value if value == kw_static_lifetime::DATA.value =>
                Ok(Self::StaticLifetime(&kw_static_lifetime::DATA)),
            value if value == kw_struct::DATA.value =>
                Ok(Self::Struct(&kw_struct::DATA)),
            value if value == kw_super::DATA.value =>
                Ok(Self::Super(&kw_super::DATA)),
            value if value == kw_trait::DATA.value =>
                Ok(Self::Trait(&kw_trait::DATA)),
            value if value == kw_true::DATA.value =>
                Ok(Self::True(&kw_true::DATA)),
            value if value == kw_try::DATA.value =>
                Ok(Self::Try(&kw_try::DATA)),
            value if value == kw_type::DATA.value =>
                Ok(Self::Type(&kw_type::DATA)),
            value if value == kw_typeof::DATA.value =>
                Ok(Self::Typeof(&kw_typeof::DATA)),
            value if value == kw_union::DATA.value =>
                Ok(Self::Union(&kw_union::DATA)),
            value if value == kw_unsafe::DATA.value =>
                Ok(Self::Unsafe(&kw_unsafe::DATA)),
            value if value == kw_unsized::DATA.value =>
                Ok(Self::Unsized(&kw_unsized::DATA)),
            value if value == kw_use::DATA.value =>
                Ok(Self::Use(&kw_use::DATA)),
            value if value == kw_virtual::DATA.value =>
                Ok(Self::Virtual(&kw_virtual::DATA)),
            value if value == kw_where::DATA.value =>
                Ok(Self::Where(&kw_where::DATA)),
            value if value == kw_while::DATA.value =>
                Ok(Self::While(&kw_while::DATA)),
            value if value == kw_yield::DATA.value =>
                Ok(Self::Yield(&kw_yield::DATA)),
            _ => Err(Error {
                message: format!("Not a keyword: {value}"),
                source: None,
            }),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    pub fn discriminants_consistent() {
        for keyword in Keyword::iter() {
            let discriminant = KeywordDiscriminants::from(&keyword);
            assert_eq!(keyword, Keyword::from(&discriminant));
        }
    }
}
