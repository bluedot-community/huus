// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Tests of `conversions` module.

#[macro_use]
extern crate maplit;

use std::collections::{BTreeMap, HashMap};

use bson::{bson, doc};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum TestEnum {
    Abc,
    Def,
}

impl huus::conversions::HuusKey for TestEnum {
    fn from_str(string: &str) -> Result<Self, huus::errors::ConversionError> {
        match string.as_ref() {
            "abc" => Ok(TestEnum::Abc),
            "def" => Ok(TestEnum::Def),
            _ => Err(huus::errors::ConversionError::incorrect_value(string.to_string())),
        }
    }

    fn to_str(&self) -> &'static str {
        match self {
            TestEnum::Abc => "abc",
            TestEnum::Def => "def",
        }
    }
}

#[test]
fn test_conversion_from_doc_to_btree_map() {
    use huus::conversions::FromDoc;

    type TestMap = BTreeMap<TestEnum, String>;

    let document = doc! {
        "abc": "cba",
        "def": "fed",
    };

    let map = maplit::btreemap! {
        TestEnum::Abc => "cba".to_string(),
        TestEnum::Def => "fed".to_string(),
    };

    assert_eq!(TestMap::from_doc(document).unwrap(), map);
}

#[test]
fn test_conversion_from_doc_to_hash() {
    use huus::conversions::FromDoc;

    type TestMap = HashMap<TestEnum, String>;

    let document = doc! {
        "abc": "cba",
        "def": "fed",
    };

    let result = TestMap::from_doc(document).unwrap();
    assert_eq!(result.len(), 2);
    assert_eq!(result.get(&TestEnum::Abc).unwrap(), "cba");
    assert_eq!(result.get(&TestEnum::Def).unwrap(), "fed");
}
