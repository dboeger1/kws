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
use strum_macros::EnumIter;


#[derive(Debug, EnumIter, Eq, Hash, PartialEq)]
pub enum Keyword {
    Abstract,
    As,
    Async,
    Await,
    Become,
    Box,
    Break,
    Const,
    Continue,
    Crate,
    Do,
    Dyn,
    Else,
    Enum,
    Extern,
    False,
    Final,
    Fn,
    For,
    Gen,
    If,
    Impl,
    In,
    Let,
    Loop,
    Macro,
    MacroRules,
    Match,
    Mod,
    Move,
    Mut,
    Override,
    Priv,
    Pub,
    Raw,
    Ref,
    Return,
    Safe,
    SelfValue,
    SelfType,
    Static,
    StaticLifetime,
    Struct,
    Super,
    Trait,
    True,
    Try,
    Type,
    Typeof,
    Union,
    Unsafe,
    Unsized,
    Use,
    Virtual,
    Where,
    While,
    Yield,
}

impl Keyword {
    pub fn data(&self) -> &'static KeywordData {
        match self {
            Self::Abstract => &kw_abstract::DATA,
            Self::As => &kw_as::DATA,
            Self::Async => &kw_async::DATA,
            Self::Await => &kw_await::DATA,
            Self::Become => &kw_become::DATA,
            Self::Box => &kw_box::DATA,
            Self::Break => &kw_break::DATA,
            Self::Const => &kw_const::DATA,
            Self::Continue => &kw_continue::DATA,
            Self::Crate => &kw_crate::DATA,
            Self::Do => &kw_do::DATA,
            Self::Dyn => &kw_dyn::DATA,
            Self::Else => &kw_else::DATA,
            Self::Enum => &kw_enum::DATA,
            Self::Extern => &kw_extern::DATA,
            Self::False => &kw_false::DATA,
            Self::Final => &kw_final::DATA,
            Self::Fn => &kw_fn::DATA,
            Self::For => &kw_for::DATA,
            Self::Gen => &kw_gen::DATA,
            Self::If => &kw_if::DATA,
            Self::Impl => &kw_impl::DATA,
            Self::In => &kw_in::DATA,
            Self::Let => &kw_let::DATA,
            Self::Loop => &kw_loop::DATA,
            Self::Macro => &kw_macro::DATA,
            Self::MacroRules => &kw_macro_rules::DATA,
            Self::Match => &kw_match::DATA,
            Self::Mod => &kw_mod::DATA,
            Self::Move => &kw_move::DATA,
            Self::Mut => &kw_mut::DATA,
            Self::Override => &kw_override::DATA,
            Self::Priv => &kw_priv::DATA,
            Self::Pub => &kw_pub::DATA,
            Self::Raw => &kw_raw::DATA,
            Self::Ref => &kw_ref::DATA,
            Self::Return => &kw_return::DATA,
            Self::Safe => &kw_safe::DATA,
            Self::SelfValue => &kw_self_value::DATA,
            Self::SelfType => &kw_self_type::DATA,
            Self::Static => &kw_static::DATA,
            Self::StaticLifetime => &kw_static_lifetime::DATA,
            Self::Struct => &kw_struct::DATA,
            Self::Super => &kw_super::DATA,
            Self::Trait => &kw_trait::DATA,
            Self::True => &kw_true::DATA,
            Self::Try => &kw_try::DATA,
            Self::Type => &kw_type::DATA,
            Self::Typeof => &kw_typeof::DATA,
            Self::Union => &kw_union::DATA,
            Self::Unsafe => &kw_unsafe::DATA,
            Self::Unsized => &kw_unsized::DATA,
            Self::Use => &kw_use::DATA,
            Self::Virtual => &kw_virtual::DATA,
            Self::Where => &kw_where::DATA,
            Self::While => &kw_while::DATA,
            Self::Yield => &kw_yield::DATA,
        }
    }
}

impl Deref for Keyword {
    type Target = KeywordData;


    fn deref(&self) -> &Self::Target {
        self.data()
    }
}


impl TryFrom<&str> for Keyword {
    type Error = Error;


    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            value if value == kw_abstract::DATA.value =>
                Ok(Self::Abstract),
            value if value == kw_as::DATA.value =>
                Ok(Self::As),
            value if value == kw_async::DATA.value =>
                Ok(Self::Async),
            value if value == kw_await::DATA.value =>
                Ok(Self::Await),
            value if value == kw_become::DATA.value =>
                Ok(Self::Become),
            value if value == kw_box::DATA.value =>
                Ok(Self::Box),
            value if value == kw_break::DATA.value =>
                Ok(Self::Break),
            value if value == kw_const::DATA.value =>
                Ok(Self::Const),
            value if value == kw_continue::DATA.value =>
                Ok(Self::Continue),
            value if value == kw_crate::DATA.value =>
                Ok(Self::Crate),
            value if value == kw_do::DATA.value =>
                Ok(Self::Do),
            value if value == kw_dyn::DATA.value =>
                Ok(Self::Dyn),
            value if value == kw_else::DATA.value =>
                Ok(Self::Else),
            value if value == kw_enum::DATA.value =>
                Ok(Self::Enum),
            value if value == kw_extern::DATA.value =>
                Ok(Self::Extern),
            value if value == kw_false::DATA.value =>
                Ok(Self::False),
            value if value == kw_final::DATA.value =>
                Ok(Self::Final),
            value if value == kw_fn::DATA.value =>
                Ok(Self::Fn),
            value if value == kw_for::DATA.value =>
                Ok(Self::For),
            value if value == kw_gen::DATA.value =>
                Ok(Self::Gen),
            value if value == kw_if::DATA.value =>
                Ok(Self::If),
            value if value == kw_impl::DATA.value =>
                Ok(Self::Impl),
            value if value == kw_in::DATA.value =>
                Ok(Self::In),
            value if value == kw_let::DATA.value =>
                Ok(Self::Let),
            value if value == kw_loop::DATA.value =>
                Ok(Self::Loop),
            value if value == kw_macro::DATA.value =>
                Ok(Self::Macro),
            value if value == kw_macro_rules::DATA.value =>
                Ok(Self::MacroRules),
            value if value == kw_match::DATA.value =>
                Ok(Self::Match),
            value if value == kw_mod::DATA.value =>
                Ok(Self::Mod),
            value if value == kw_move::DATA.value =>
                Ok(Self::Move),
            value if value == kw_mut::DATA.value =>
                Ok(Self::Mut),
            value if value == kw_override::DATA.value =>
                Ok(Self::Override),
            value if value == kw_priv::DATA.value =>
                Ok(Self::Priv),
            value if value == kw_pub::DATA.value =>
                Ok(Self::Pub),
            value if value == kw_raw::DATA.value =>
                Ok(Self::Raw),
            value if value == kw_ref::DATA.value =>
                Ok(Self::Ref),
            value if value == kw_return::DATA.value =>
                Ok(Self::Return),
            value if value == kw_safe::DATA.value =>
                Ok(Self::Safe),
            value if value == kw_self_value::DATA.value =>
                Ok(Self::SelfValue),
            value if value == kw_self_type::DATA.value =>
                Ok(Self::SelfType),
            value if value == kw_static::DATA.value =>
                Ok(Self::Static),
            value if value == kw_static_lifetime::DATA.value =>
                Ok(Self::StaticLifetime),
            value if value == kw_struct::DATA.value =>
                Ok(Self::Struct),
            value if value == kw_super::DATA.value =>
                Ok(Self::Super),
            value if value == kw_trait::DATA.value =>
                Ok(Self::Trait),
            value if value == kw_true::DATA.value =>
                Ok(Self::True),
            value if value == kw_try::DATA.value =>
                Ok(Self::Try),
            value if value == kw_type::DATA.value =>
                Ok(Self::Type),
            value if value == kw_typeof::DATA.value =>
                Ok(Self::Typeof),
            value if value == kw_union::DATA.value =>
                Ok(Self::Union),
            value if value == kw_unsafe::DATA.value =>
                Ok(Self::Unsafe),
            value if value == kw_unsized::DATA.value =>
                Ok(Self::Unsized),
            value if value == kw_use::DATA.value =>
                Ok(Self::Use),
            value if value == kw_virtual::DATA.value =>
                Ok(Self::Virtual),
            value if value == kw_where::DATA.value =>
                Ok(Self::Where),
            value if value == kw_while::DATA.value =>
                Ok(Self::While),
            value if value == kw_yield::DATA.value =>
                Ok(Self::Yield),
            _ => Err(Error {
                message: format!("Not a keyword: {value}"),
                source: None,
            }),
        }
    }
}
