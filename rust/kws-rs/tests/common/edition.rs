use std::{
    hash::Hash,
    mem::discriminant,
};


pub struct Edition(pub kws::Edition);

impl Eq for Edition {}

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
