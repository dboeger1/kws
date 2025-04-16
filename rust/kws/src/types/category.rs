use strum_macros::EnumIter;


#[derive(Eq, EnumIter, Hash, PartialEq)]
pub enum Category {
    Strict,
    Reserved,
    Weak,
}
