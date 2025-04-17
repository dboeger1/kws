use std::collections::HashSet;
use strum::IntoEnumIterator;
use kws::{
    error::Error,
    keyword::Keyword,

};


#[test]
fn values_unique() {
    let mut seen: HashSet<&'static str> = HashSet::new();
    for keyword in Keyword::iter() {
        assert!(!seen.contains(keyword.value));
        seen.insert(keyword.value);
    }
}

#[test]
fn values_consistent() -> Result<(), Error> {
    for keyword in Keyword::iter() {
        let value = keyword.value;
        assert_eq!(keyword, Keyword::try_from(value)?);
    }

    Ok(())
}

#[test]
fn value_invalid() {
    assert!(Keyword::try_from("invalid").is_err());
}
