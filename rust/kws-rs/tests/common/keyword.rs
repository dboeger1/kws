use std::{
    fmt::Debug,
    hash::Hash,
    mem::discriminant,
};


pub struct Keyword(pub kws::Keyword);

impl Debug for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self.0 {
            kws::Keyword::Abstract => "Abstract",
            kws::Keyword::As => "As",
            kws::Keyword::Async => "Async",
            kws::Keyword::Await => "Await",
            kws::Keyword::Become => "Become",
            kws::Keyword::Box => "Box",
            kws::Keyword::Break => "Break",
            kws::Keyword::Const => "Const",
            kws::Keyword::Continue => "Continue",
            kws::Keyword::Crate => "Crate",
            kws::Keyword::Do => "Do",
            kws::Keyword::Dyn => "Dyn",
            kws::Keyword::Else => "Else",
            kws::Keyword::Enum => "Enum",
            kws::Keyword::Extern => "Extern",
            kws::Keyword::False => "False",
            kws::Keyword::Final => "Final",
            kws::Keyword::Fn => "Fn",
            kws::Keyword::For => "For",
            kws::Keyword::Gen => "Gen",
            kws::Keyword::If => "If",
            kws::Keyword::Impl => "Impl",
            kws::Keyword::In => "In",
            kws::Keyword::Let => "Let",
            kws::Keyword::Loop => "Loop",
            kws::Keyword::Macro => "Macro",
            kws::Keyword::MacroRules => "MacroRules",
            kws::Keyword::Match => "Match",
            kws::Keyword::Mod => "Mod",
            kws::Keyword::Move => "Move",
            kws::Keyword::Mut => "Mut",
            kws::Keyword::Override => "Override",
            kws::Keyword::Priv => "Priv",
            kws::Keyword::Pub => "Pub",
            kws::Keyword::Raw => "Raw",
            kws::Keyword::Ref => "Ref",
            kws::Keyword::Return => "Return",
            kws::Keyword::Safe => "Safe",
            kws::Keyword::SelfType => "SelfType",
            kws::Keyword::SelfValue => "SelfValue",
            kws::Keyword::Static => "Static",
            kws::Keyword::StaticLifetime => "StaticLifetime",
            kws::Keyword::Struct => "Struct",
            kws::Keyword::Super => "Super",
            kws::Keyword::Trait => "Trait",
            kws::Keyword::True => "True",
            kws::Keyword::Try => "Try",
            kws::Keyword::Type => "Type",
            kws::Keyword::Typeof => "Typeof",
            kws::Keyword::Union => "Union",
            kws::Keyword::Unsafe => "Unsafe",
            kws::Keyword::Unsized => "Unsized",
            kws::Keyword::Use => "Use",
            kws::Keyword::Virtual => "Virtual",
            kws::Keyword::Where => "Where",
            kws::Keyword::While => "While",
            kws::Keyword::Yield => "Yield",
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
            (kws::Keyword::Abstract, kws::Keyword::Abstract) |
            (kws::Keyword::As, kws::Keyword::As) |
            (kws::Keyword::Async, kws::Keyword::Async) |
            (kws::Keyword::Await, kws::Keyword::Await) |
            (kws::Keyword::Become, kws::Keyword::Become) |
            (kws::Keyword::Box, kws::Keyword::Box) |
            (kws::Keyword::Break, kws::Keyword::Break) |
            (kws::Keyword::Const, kws::Keyword::Const) |
            (kws::Keyword::Continue, kws::Keyword::Continue) |
            (kws::Keyword::Crate, kws::Keyword::Crate) |
            (kws::Keyword::Do, kws::Keyword::Do) |
            (kws::Keyword::Dyn, kws::Keyword::Dyn) |
            (kws::Keyword::Else, kws::Keyword::Else) |
            (kws::Keyword::Enum, kws::Keyword::Enum) |
            (kws::Keyword::Extern, kws::Keyword::Extern) |
            (kws::Keyword::False, kws::Keyword::False) |
            (kws::Keyword::Final, kws::Keyword::Final) |
            (kws::Keyword::Fn, kws::Keyword::Fn) |
            (kws::Keyword::For, kws::Keyword::For) |
            (kws::Keyword::Gen, kws::Keyword::Gen) |
            (kws::Keyword::If, kws::Keyword::If) |
            (kws::Keyword::Impl, kws::Keyword::Impl) |
            (kws::Keyword::In, kws::Keyword::In) |
            (kws::Keyword::Let, kws::Keyword::Let) |
            (kws::Keyword::Loop, kws::Keyword::Loop) |
            (kws::Keyword::Macro, kws::Keyword::Macro) |
            (kws::Keyword::MacroRules, kws::Keyword::MacroRules) |
            (kws::Keyword::Match, kws::Keyword::Match) |
            (kws::Keyword::Mod, kws::Keyword::Mod) |
            (kws::Keyword::Move, kws::Keyword::Move) |
            (kws::Keyword::Mut, kws::Keyword::Mut) |
            (kws::Keyword::Override, kws::Keyword::Override) |
            (kws::Keyword::Priv, kws::Keyword::Priv) |
            (kws::Keyword::Pub, kws::Keyword::Pub) |
            (kws::Keyword::Raw, kws::Keyword::Raw) |
            (kws::Keyword::Ref, kws::Keyword::Ref) |
            (kws::Keyword::Return, kws::Keyword::Return) |
            (kws::Keyword::Safe, kws::Keyword::Safe) |
            (kws::Keyword::SelfType, kws::Keyword::SelfType) |
            (kws::Keyword::SelfValue, kws::Keyword::SelfValue) |
            (kws::Keyword::Static, kws::Keyword::Static) |
            (kws::Keyword::StaticLifetime, kws::Keyword::StaticLifetime) |
            (kws::Keyword::Struct, kws::Keyword::Struct) |
            (kws::Keyword::Super, kws::Keyword::Super) |
            (kws::Keyword::Trait, kws::Keyword::Trait) |
            (kws::Keyword::True, kws::Keyword::True) |
            (kws::Keyword::Try, kws::Keyword::Try) |
            (kws::Keyword::Type, kws::Keyword::Type) |
            (kws::Keyword::Typeof, kws::Keyword::Typeof) |
            (kws::Keyword::Union, kws::Keyword::Union) |
            (kws::Keyword::Unsafe, kws::Keyword::Unsafe) |
            (kws::Keyword::Unsized, kws::Keyword::Unsized) |
            (kws::Keyword::Use, kws::Keyword::Use) |
            (kws::Keyword::Virtual, kws::Keyword::Virtual) |
            (kws::Keyword::Where, kws::Keyword::Where) |
            (kws::Keyword::While, kws::Keyword::While) |
            (kws::Keyword::Yield, kws::Keyword::Yield),
        ) &&
            KeywordData(self.0.data()) == KeywordData(other.0.data())
    }
}


pub struct KeywordData(pub &'static kws::KeywordData);

impl Eq for KeywordData {}

impl PartialEq for KeywordData {
    fn eq(&self, other: &Self) -> bool {
        self.0.value == other.0.value &&
            self.0.category as usize == other.0.category as usize
    }
}


pub struct Category(pub kws::Category);

impl Debug for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self.0 {
            kws::Category::Strict => "Strict",
            kws::Category::Reserved => "Reserved",
            kws::Category::Weak => "Weak",
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
            (kws::Category::Strict, kws::Category::Strict) |
            (kws::Category::Reserved, kws::Category::Reserved) |
            (kws::Category::Weak, kws::Category::Weak),
        )
    }
}
