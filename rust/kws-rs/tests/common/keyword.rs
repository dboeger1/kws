use std::{
    fmt::Debug,
    hash::Hash,
    mem::discriminant,
};


pub struct Keyword(pub kws_rs::Keyword);

impl Debug for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self.0 {
            kws_rs::Keyword::Abstract => "Abstract",
            kws_rs::Keyword::As => "As",
            kws_rs::Keyword::Async => "Async",
            kws_rs::Keyword::Await => "Await",
            kws_rs::Keyword::Become => "Become",
            kws_rs::Keyword::Box => "Box",
            kws_rs::Keyword::Break => "Break",
            kws_rs::Keyword::Const => "Const",
            kws_rs::Keyword::Continue => "Continue",
            kws_rs::Keyword::Crate => "Crate",
            kws_rs::Keyword::Do => "Do",
            kws_rs::Keyword::Dyn => "Dyn",
            kws_rs::Keyword::Else => "Else",
            kws_rs::Keyword::Enum => "Enum",
            kws_rs::Keyword::Extern => "Extern",
            kws_rs::Keyword::False => "False",
            kws_rs::Keyword::Final => "Final",
            kws_rs::Keyword::Fn => "Fn",
            kws_rs::Keyword::For => "For",
            kws_rs::Keyword::Gen => "Gen",
            kws_rs::Keyword::If => "If",
            kws_rs::Keyword::Impl => "Impl",
            kws_rs::Keyword::In => "In",
            kws_rs::Keyword::Let => "Let",
            kws_rs::Keyword::Loop => "Loop",
            kws_rs::Keyword::Macro => "Macro",
            kws_rs::Keyword::MacroRules => "MacroRules",
            kws_rs::Keyword::Match => "Match",
            kws_rs::Keyword::Mod => "Mod",
            kws_rs::Keyword::Move => "Move",
            kws_rs::Keyword::Mut => "Mut",
            kws_rs::Keyword::Override => "Override",
            kws_rs::Keyword::Priv => "Priv",
            kws_rs::Keyword::Pub => "Pub",
            kws_rs::Keyword::Raw => "Raw",
            kws_rs::Keyword::Ref => "Ref",
            kws_rs::Keyword::Return => "Return",
            kws_rs::Keyword::Safe => "Safe",
            kws_rs::Keyword::SelfType => "SelfType",
            kws_rs::Keyword::SelfValue => "SelfValue",
            kws_rs::Keyword::Static => "Static",
            kws_rs::Keyword::StaticLifetime => "StaticLifetime",
            kws_rs::Keyword::Struct => "Struct",
            kws_rs::Keyword::Super => "Super",
            kws_rs::Keyword::Trait => "Trait",
            kws_rs::Keyword::True => "True",
            kws_rs::Keyword::Try => "Try",
            kws_rs::Keyword::Type => "Type",
            kws_rs::Keyword::Typeof => "Typeof",
            kws_rs::Keyword::Union => "Union",
            kws_rs::Keyword::Unsafe => "Unsafe",
            kws_rs::Keyword::Unsized => "Unsized",
            kws_rs::Keyword::Use => "Use",
            kws_rs::Keyword::Virtual => "Virtual",
            kws_rs::Keyword::Where => "Where",
            kws_rs::Keyword::While => "While",
            kws_rs::Keyword::Yield => "Yield",
        })
    }
}

impl Eq for Keyword {}

impl Hash for Keyword {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        discriminant(&self.0).hash(state);
    }
}

impl PartialEq for Keyword {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (&self.0, &other.0),
            (kws_rs::Keyword::Abstract, kws_rs::Keyword::Abstract) |
            (kws_rs::Keyword::As, kws_rs::Keyword::As) |
            (kws_rs::Keyword::Async, kws_rs::Keyword::Async) |
            (kws_rs::Keyword::Await, kws_rs::Keyword::Await) |
            (kws_rs::Keyword::Become, kws_rs::Keyword::Become) |
            (kws_rs::Keyword::Box, kws_rs::Keyword::Box) |
            (kws_rs::Keyword::Break, kws_rs::Keyword::Break) |
            (kws_rs::Keyword::Const, kws_rs::Keyword::Const) |
            (kws_rs::Keyword::Continue, kws_rs::Keyword::Continue) |
            (kws_rs::Keyword::Crate, kws_rs::Keyword::Crate) |
            (kws_rs::Keyword::Do, kws_rs::Keyword::Do) |
            (kws_rs::Keyword::Dyn, kws_rs::Keyword::Dyn) |
            (kws_rs::Keyword::Else, kws_rs::Keyword::Else) |
            (kws_rs::Keyword::Enum, kws_rs::Keyword::Enum) |
            (kws_rs::Keyword::Extern, kws_rs::Keyword::Extern) |
            (kws_rs::Keyword::False, kws_rs::Keyword::False) |
            (kws_rs::Keyword::Final, kws_rs::Keyword::Final) |
            (kws_rs::Keyword::Fn, kws_rs::Keyword::Fn) |
            (kws_rs::Keyword::For, kws_rs::Keyword::For) |
            (kws_rs::Keyword::Gen, kws_rs::Keyword::Gen) |
            (kws_rs::Keyword::If, kws_rs::Keyword::If) |
            (kws_rs::Keyword::Impl, kws_rs::Keyword::Impl) |
            (kws_rs::Keyword::In, kws_rs::Keyword::In) |
            (kws_rs::Keyword::Let, kws_rs::Keyword::Let) |
            (kws_rs::Keyword::Loop, kws_rs::Keyword::Loop) |
            (kws_rs::Keyword::Macro, kws_rs::Keyword::Macro) |
            (kws_rs::Keyword::MacroRules, kws_rs::Keyword::MacroRules) |
            (kws_rs::Keyword::Match, kws_rs::Keyword::Match) |
            (kws_rs::Keyword::Mod, kws_rs::Keyword::Mod) |
            (kws_rs::Keyword::Move, kws_rs::Keyword::Move) |
            (kws_rs::Keyword::Mut, kws_rs::Keyword::Mut) |
            (kws_rs::Keyword::Override, kws_rs::Keyword::Override) |
            (kws_rs::Keyword::Priv, kws_rs::Keyword::Priv) |
            (kws_rs::Keyword::Pub, kws_rs::Keyword::Pub) |
            (kws_rs::Keyword::Raw, kws_rs::Keyword::Raw) |
            (kws_rs::Keyword::Ref, kws_rs::Keyword::Ref) |
            (kws_rs::Keyword::Return, kws_rs::Keyword::Return) |
            (kws_rs::Keyword::Safe, kws_rs::Keyword::Safe) |
            (kws_rs::Keyword::SelfType, kws_rs::Keyword::SelfType) |
            (kws_rs::Keyword::SelfValue, kws_rs::Keyword::SelfValue) |
            (kws_rs::Keyword::Static, kws_rs::Keyword::Static) |
            (kws_rs::Keyword::StaticLifetime, kws_rs::Keyword::StaticLifetime) |
            (kws_rs::Keyword::Struct, kws_rs::Keyword::Struct) |
            (kws_rs::Keyword::Super, kws_rs::Keyword::Super) |
            (kws_rs::Keyword::Trait, kws_rs::Keyword::Trait) |
            (kws_rs::Keyword::True, kws_rs::Keyword::True) |
            (kws_rs::Keyword::Try, kws_rs::Keyword::Try) |
            (kws_rs::Keyword::Type, kws_rs::Keyword::Type) |
            (kws_rs::Keyword::Typeof, kws_rs::Keyword::Typeof) |
            (kws_rs::Keyword::Union, kws_rs::Keyword::Union) |
            (kws_rs::Keyword::Unsafe, kws_rs::Keyword::Unsafe) |
            (kws_rs::Keyword::Unsized, kws_rs::Keyword::Unsized) |
            (kws_rs::Keyword::Use, kws_rs::Keyword::Use) |
            (kws_rs::Keyword::Virtual, kws_rs::Keyword::Virtual) |
            (kws_rs::Keyword::Where, kws_rs::Keyword::Where) |
            (kws_rs::Keyword::While, kws_rs::Keyword::While) |
            (kws_rs::Keyword::Yield, kws_rs::Keyword::Yield),
        ) &&
            KeywordData(self.0.data()) == KeywordData(other.0.data())
    }
}


pub struct KeywordData(pub &'static kws_rs::KeywordData);

impl Eq for KeywordData {}

impl PartialEq for KeywordData {
    fn eq(&self, other: &Self) -> bool {
        self.0.value == other.0.value &&
            self.0.category as usize == other.0.category as usize
    }
}


pub struct Category(pub kws_rs::Category);

impl Debug for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self.0 {
            kws_rs::Category::Strict => "Strict",
            kws_rs::Category::Reserved => "Reserved",
            kws_rs::Category::Weak => "Weak",
        })
    }
}
impl Eq for Category {}

impl Hash for Category {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        discriminant(&self.0).hash(state);
    }
}

impl PartialEq for Category {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (&self.0, &other.0),
            (kws_rs::Category::Strict, kws_rs::Category::Strict) |
            (kws_rs::Category::Reserved, kws_rs::Category::Reserved) |
            (kws_rs::Category::Weak, kws_rs::Category::Weak),
        )
    }
}
