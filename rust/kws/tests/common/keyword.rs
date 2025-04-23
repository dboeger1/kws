use std::{
    hash::Hash,
    mem::discriminant,
};


#[derive(Debug)]
pub struct Keyword(pub kws::Keyword);

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
