mod common;


use common::keyword::Keyword;
use kws_rs::Error;
use std::collections::HashSet;
use strum::IntoEnumIterator;


#[test]
fn values_unique() {
    let mut seen: HashSet<&'static str> = HashSet::new();
    for keyword in kws_rs::Keyword::iter() {
        assert!(!seen.contains(keyword.value));
        seen.insert(keyword.value);
    }
}

#[test]
fn values_consistent() -> Result<(), Error> {
    for keyword in kws_rs::Keyword::iter() {
        let value = keyword.value;
        assert_eq!(
            Keyword(keyword),
            Keyword(kws_rs::Keyword::try_from(value)?),
        );
    }

    Ok(())
}

#[test]
fn value_invalid() {
    assert!(kws_rs::Keyword::try_from("invalid").is_err());
}
