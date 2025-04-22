use std::{
    hash::Hash,
    mem::discriminant,
    //ops::Deref,
};


pub(crate) struct Edition(pub(crate) kws::Edition);

//impl Deref for Edition {
//    type Target = kws::Edition;
//
//
//    fn deref(&self) -> &Self::Target {
//        &self.0
//    }
//}

impl Eq for Edition {}

impl From<kws::Edition> for Edition {
    fn from(value: kws::Edition) -> Self {
        Self(value)
    }
}

impl Hash for Edition {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        discriminant(&self.0).hash(state);
    }
}

impl PartialEq for Edition {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (&self.0, &other.0),
            (kws::Edition::Rust2015, kws::Edition::Rust2015) |
            (kws::Edition::Rust2018, kws::Edition::Rust2018) |
            (kws::Edition::Rust2021, kws::Edition::Rust2021) |
            (kws::Edition::Rust2024, kws::Edition::Rust2024),
        )
    }
}
