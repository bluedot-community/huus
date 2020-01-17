// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Tests for `huus_macros` crate.

#![feature(proc_macro_hygiene)]

use bson::{bson, doc};
use chrono::offset::TimeZone;

use huus::models::prelude::*;

huus_macros::define_from!("test");

// -------------------------------------------------------------------------------------------------
// Query formulation

/// Check if `huus_macros::data` generates the code as expected.
#[test]
fn data_formulation() {
    let data = Doc1Data { integer: Some(3), string: "hello".to_string(), array: None };
    let epoch = chrono::Utc.ymd(1970, 1, 1).and_hms(0, 0, 0);

    let query = huus_macros::data! { ("coll_3")
        _id: "243423323458458728644937",
        data: (data),
        array: (vec![
            Doc1Data { integer: Some(2), string: "def".to_string(), array: None },
            Doc1Data { integer: Some(3), string: "ghi".to_string(), array: None },
        ]),
        simple_map: (maplit::btreemap! {
            "choice_1".to_string() => "one".to_string(),
            "choice_2".to_string() => "two".to_string(),
        }),
        nested_map: (maplit::btreemap! {
            Enum1Data::Choice1 => Doc1Data {
                integer: Some(4),
                string: "jkl".to_string(),
                array: None,
            },
            Enum1Data::Choice2 => Doc1Data {
                integer: Some(5),
                string: "mno".to_string(),
                array: None,
            },
        }),
        boolean: (true),
        date: (epoch),
        indexed: "hi",
        integers: (vec![3.14, 2.72].iter().map(|&e| e as i64).collect()),
        choice: (Enum1Data::Choice1),
        union: (Union1Data::Choice1(Doc1Data {
            integer: Some(6),
            string: "pqr".to_string(),
            array: None,
        })),
        bson: (doc! { "a": 1, "b": "hi" }),
    };

    let expectation = doc! {
        "_id": bson::oid::ObjectId::with_string("243423323458458728644937").unwrap(),
        "data": {
            "int": 3,
            "str": "hello",
        },
        "array": [
            { "int": 2i32, "str": "def" },
            { "int": 3i32, "str": "ghi" },
        ],
        "simple_map": {
            "choice_1": "one",
            "choice_2": "two",
        },
        "nested_map": {
            "choice_1": { "int": 4i32, "str": "jkl" },
            "choice_2": { "int": 5i32, "str": "mno" },
        },
        "boolean": true,
        "date": epoch,
        "indexed": "hi",
        "integers": [3i64, 2i64],
        "choice": "choice_1",
        "union": {
            "int": 6i32,
            "str": "pqr",
            "_huus_variant": "choice_1"
        },
        "bson": { "a": 1, "b": "hi" },
    };

    assert_eq!(query.into_doc(), expectation);
}

/// Check if `huus_macros::filter` generates the code as expected.
#[test]
fn filter_formulation() {
    let data = Doc1Data { integer: Some(3), string: "hello".to_string(), array: None };
    let hi = "hi".to_string();
    let epoch = chrono::Utc.ymd(1970, 1, 1).and_hms(0, 0, 0);

    let query1 = huus_macros::filter! { ("coll_3")
        "_id": "243423323458458728644937",
        "data": {
            "int": { "$lt": 3 },
            "str": "hello"
        },
        "boolean": { "$ne": true },
        "date": "1970-01-01T00:00:00Z",
        "indexed": "hi",
        "union.int": 5,
    };

    let expectation1 = doc! {
        "_id": bson::oid::ObjectId::with_string("243423323458458728644937").unwrap(),
        "data": {
            "int": { "$lt": 3 },
            "str": "hello",
        },
        "boolean": { "$ne": true },
        "date": epoch,
        "indexed": "hi",
        "union.int": 5,
    };

    let query2 = huus_macros::filter! { ("coll_3")
        "_id": (bson::oid::ObjectId::with_string("243423323458458728644937").unwrap()),
        "data": (data),
        "array": (vec![
            Doc1Data { integer: Some(2), string: "def".to_string(), array: None },
            Doc1Data { integer: Some(3), string: "ghi".to_string(), array: None },
        ]),
        "simple_map": (maplit::btreemap! {
            "choice_1".to_string() => "one".to_string(),
            "choice_2".to_string() => "two".to_string(),
        }),
        "nested_map": (maplit::btreemap! {
            Enum1Data::Choice1 => Doc1Data {
                integer: Some(4),
                string: "jkl".to_string(),
                array: None,
            },
            Enum1Data::Choice2 => Doc1Data {
                integer: Some(5),
                string: "mno".to_string(),
                array: None,
            },
        }),
        "boolean": { "$ne": (true) },
        "date": (epoch),
        "indexed": (hi.clone()),
        "integers": (vec![3.14, 2.72].iter().map(|&e| e as i64).collect()),
        "choice": (Enum1Data::Choice1),
        "union": (Union1Data::Choice1(Doc1Data {
            integer: Some(6),
            string: "pqr".to_string(),
            array: None,
        })),
        "bson": (doc! { "a": 1, "b": hi.clone() }),
    };

    let expectation2 = doc! {
        "_id": bson::oid::ObjectId::with_string("243423323458458728644937").unwrap(),
        "data": {
            "int": 3,
            "str": "hello",
        },
        "array": [
            { "int": 2i32, "str": "def" },
            { "int": 3i32, "str": "ghi" },
        ],
        "simple_map": {
            "choice_1": "one",
            "choice_2": "two",
        },
        "nested_map": {
            "choice_1": { "int": 4i32, "str": "jkl" },
            "choice_2": { "int": 5i32, "str": "mno" },
        },
        "boolean": { "$ne": true },
        "date": epoch,
        "indexed": "hi",
        "integers": [3i64, 2i64],
        "choice": "choice_1",
        "union": {
            "int": 6i32,
            "str": "pqr",
            "_huus_variant": "choice_1"
        },
        "bson": { "a": 1, "b": hi },
    };

    assert_eq!(query1.into_doc(), expectation1);
    assert_eq!(query2.into_doc(), expectation2);
}

/// Check if `huus_macros::update` generates the code as expected in update mode.
#[test]
fn update_formulation() {
    let update1 = huus_macros::update! { ("coll_3")
        "$inc": { "data.int": 1 },
        "$set": {
            data.str: "abc",
            simple_map: (maplit::btreemap! {
                "choice_1".to_string() => "one".to_string(),
                "choice_2".to_string() => "two".to_string(),
            }),
            nested_map: (maplit::btreemap! {
                Enum1Data::Choice1 => Doc1Data {
                    integer: Some(4),
                    string: "jkl".to_string(),
                    array: None,
                },
                Enum1Data::Choice2 => Doc1Data {
                    integer: Some(5),
                    string: "mno".to_string(),
                    array: None,
                },
            }),
        },
        "$unset": {
            "boolean": "",
            "indexed": "",
        },
        "$currentDate": { "date": true },
        "$rename": {
            "bson": "info",
        },
        "$push": {
            "integers": 4,
            "array": (Doc1Data { integer: Some(2), string: "def".to_string(), array: None })
        },
    };

    let expected = doc! {
        "$inc": { "data.int": 1i32 },
        "$set": {
            "data.str": "abc",
            "simple_map": {
                "choice_1": "one",
                "choice_2": "two",
            },
            "nested_map": {
                "choice_1": { "int": 4i32, "str": "jkl" },
                "choice_2": { "int": 5i32, "str": "mno" },
            },
        },
        "$unset": {
            "boolean": "",
            "indexed": "",
        },
        "$currentDate": { "date": true },
        "$rename": {
            "bson": "info",
        },
        "$push": {
            "integers": 4i64,
            "array": {
                "int": 2i32,
                "str": "def",
            },
        },
    };

    assert_eq!(update1.into_doc(), expected);
}

/// Check if `huus_macros::update` generates the code as expected in replacement mode.
#[test]
fn replacement_formulation() {
    let data = Doc1Data { integer: Some(3), string: "hello".to_string(), array: None };
    let hi = "hi".to_string();
    let epoch = chrono::Utc.ymd(1970, 1, 1).and_hms(0, 0, 0);

    let update1 = huus_macros::update! { ("coll_3")
        "_id": "243423323458458728644937",
        "data": {
            "int": 3,
            "str": "hello"
        },
        "boolean": true,
        "date": "1970-01-01T00:00:00Z",
        "indexed": "hi",
    };

    let expectation1 = doc! {
        "_id": bson::oid::ObjectId::with_string("243423323458458728644937").unwrap(),
        "data": {
            "int": 3,
            "str": "hello",
        },
        "boolean": true,
        "date": epoch,
        "indexed": "hi",
    };

    let update2 = huus_macros::update! { ("coll_3")
        "_id": (bson::oid::ObjectId::with_string("243423323458458728644937").unwrap()),
        "data": (data),
        "array": (vec![
            Doc1Data { integer: Some(2), string: "def".to_string(), array: None },
            Doc1Data { integer: Some(3), string: "ghi".to_string(), array: None },
        ]),
        "simple_map": (maplit::btreemap! {
            "choice_1".to_string() => "one".to_string(),
            "choice_2".to_string() => "two".to_string(),
        }),
        "nested_map": (maplit::btreemap! {
            Enum1Data::Choice1 => Doc1Data {
                integer: Some(4),
                string: "jkl".to_string(),
                array: None,
            },
            Enum1Data::Choice2 => Doc1Data {
                integer: Some(5),
                string: "mno".to_string(),
                array: None,
            },
        }),
        "boolean": (true),
        "date": (epoch),
        "indexed": (hi.clone()),
        "integers": (vec![3.14, 2.72].iter().map(|&e| e as i64).collect()),
        "choice": (Enum1Data::Choice1),
        "union": (Union1Data::Choice1(Doc1Data {
            integer: Some(6),
            string: "pqr".to_string(),
            array: None,
        })),
        "bson": (doc! { "a": 1, "b": hi.clone() }),
    };

    let expectation2 = doc! {
        "_id": bson::oid::ObjectId::with_string("243423323458458728644937").unwrap(),
        "data": {
            "int": 3,
            "str": "hello",
        },
        "array": [
            { "int": 2i32, "str": "def" },
            { "int": 3i32, "str": "ghi" },
        ],
        "simple_map": {
            "choice_1": "one",
            "choice_2": "two",
        },
        "nested_map": {
            "choice_1": { "int": 4i32, "str": "jkl" },
            "choice_2": { "int": 5i32, "str": "mno" },
        },
        "boolean": true,
        "date": epoch,
        "indexed": "hi",
        "integers": [3i64, 2i64],
        "choice": "choice_1",
        "union": {
            "int": 6i32,
            "str": "pqr",
            "_huus_variant": "choice_1"
        },
        "bson": { "a": 1, "b": hi },
    };

    assert_eq!(update1.into_doc(), expectation1);
    assert_eq!(update2.into_doc(), expectation2);
}

// -------------------------------------------------------------------------------------------------
// Operators

/// Check if `huus_macros::filter` generates the code properly for all operators.
#[test]
fn filter_operators() {
    let query = huus_macros::filter! { ("coll_3") { data.int: { "$eq": 3 } } };
    let expected = doc! { "data.int": { "$eq": 3i32 } };
    assert_eq!(query.into_doc(), expected);

    let query = huus_macros::filter! { ("coll_3") { data.int: { "$gt": 3 } } };
    let expected = doc! { "data.int": { "$gt": 3i32 } };
    assert_eq!(query.into_doc(), expected);

    let query = huus_macros::filter! { ("coll_3") { data.int: { "$gte": 3 } } };
    let expected = doc! { "data.int": { "$gte": 3i32 } };
    assert_eq!(query.into_doc(), expected);

    let query = huus_macros::filter! { ("coll_3") { data.int: { "$lt": 3 } } };
    let expected = doc! { "data.int": { "$lt": 3i32 } };
    assert_eq!(query.into_doc(), expected);

    let query = huus_macros::filter! { ("coll_3") { data.int: { "$lte": 3 } } };
    let expected = doc! { "data.int": { "$lte": 3i32 } };
    assert_eq!(query.into_doc(), expected);

    let query = huus_macros::filter! { ("coll_3") { data.int: { "$ne": 3 } } };
    let expected = doc! { "data.int": { "$ne": 3i32 } };
    assert_eq!(query.into_doc(), expected);

    let opt = vec![3, 4];
    let query = huus_macros::filter! { ("coll_3") { data.int: { "$in": (opt) } } };
    let expected = doc! { "data.int": { "$in": [3i32, 4i32] } };
    assert_eq!(query.into_doc(), expected);
}

/// Check if `huus_macros::filter` generates the code properly for arrays.
#[test]
fn filter_types() {
    let array = vec![3, 4];

    let query = huus_macros::filter! { ("coll_3") { "integers": { "$in": (array) } } };
    let expected = doc! { "integers": { "$in": [3i64, 4i64] } };
    assert_eq!(query.into_doc(), expected);

    let query = huus_macros::filter! { ("coll_3") { "integers.$": { "$eq": 3 } } };
    let expected = doc! { "integers.$": { "$eq": 3i64 } };
    assert_eq!(query.into_doc(), expected);

    let index = 3;
    let query = huus_macros::filter! { ("coll_3") { integers.(index): { "$eq": (3) } } };
    let expected = doc! { "integers.3": { "$eq": 3i64 } };
    assert_eq!(query.into_doc(), expected);
}

/// Check if `huus_macros::update` generates the code properly for all operators.
#[test]
fn update_operators() {
    let query = huus_macros::update! { ("coll_3") { "$addToSet": { "integers": 3 } } };
    let expected = doc! { "$addToSet": { "integers": 3i64 } };
    assert_eq!(query.into_doc(), expected);

    let query = huus_macros::update! { ("coll_3") { "$currentDate": { "date": true } } };
    let expected = doc! { "$currentDate": { "date": true } };
    assert_eq!(query.into_doc(), expected);

    let query = huus_macros::update! { ("coll_3") { "$inc": { "data.int": 3 } } };
    let expected = doc! { "$inc": { "data.int": 3i32 } };
    assert_eq!(query.into_doc(), expected);

    let query = huus_macros::update! { ("coll_3") { "$max": { "data.int": 3 } } };
    let expected = doc! { "$max": { "data.int": 3i32 } };
    assert_eq!(query.into_doc(), expected);

    let query = huus_macros::update! { ("coll_3") { "$min": { "data.int": 3 } } };
    let expected = doc! { "$min": { "data.int": 3i32 } };
    assert_eq!(query.into_doc(), expected);

    let query = huus_macros::update! { ("coll_3") { "$mul": { "data.int": 3 } } };
    let expected = doc! { "$mul": { "data.int": 3i32 } };
    assert_eq!(query.into_doc(), expected);

    let query = huus_macros::update! { ("coll_3") { "$pop": { "integers": 3 } } };
    let expected = doc! { "$pop": { "integers": 3i64 } };
    assert_eq!(query.into_doc(), expected);

    let query = huus_macros::update! { ("coll_3") { "$pull": { "integers": 3 } } };
    let expected = doc! { "$pull": { "integers": 3i64 } };
    assert_eq!(query.into_doc(), expected);

    let array = vec![3, 4];
    let query = huus_macros::update! { ("coll_3") { "$pull": { "integers": { "$in": (array) } } } };
    let expected = doc! { "$pull": { "integers": { "$in": [3i64, 4i64] } } };
    assert_eq!(query.into_doc(), expected);

    let array = vec![3, 4];
    let query = huus_macros::update! { ("coll_3") { "$pullAll": { "integers": (array) } } };
    let expected = doc! { "$pullAll": { "integers": [3i64, 4i64] } };
    assert_eq!(query.into_doc(), expected);

    let query = huus_macros::update! { ("coll_3") { "$push": { "integers": 3 } } };
    let expected = doc! { "$push": { "integers": 3i64 } };
    assert_eq!(query.into_doc(), expected);

    let query = huus_macros::update! { ("coll_3") { "$set": { "data.int": 3 } } };
    let expected = doc! { "$set": { "data.int": 3i32 } };
    assert_eq!(query.into_doc(), expected);

    let query = huus_macros::update! { ("coll_3") { "$setOnInsert": { "data.int": 3 } } };
    let expected = doc! { "$setOnInsert": { "data.int": 3i32 } };
    assert_eq!(query.into_doc(), expected);
}

/// Check if `huus_macros::update` generates the code properly for arrays.
#[test]
fn update_types() {
    let array = vec![3, 4];
    let query = huus_macros::update! { ("coll_3") { "$set": { "integers": (array) } } };
    let expected = doc! { "$set": { "integers": [3i64, 4i64] } };
    assert_eq!(query.into_doc(), expected);

    let query = huus_macros::update! { ("coll_3") { "$set": { "integers.2": 3 } } };
    let expected = doc! { "$set": { "integers.2": 3i64 } };
    assert_eq!(query.into_doc(), expected);

    let query = huus_macros::update! { ("coll_3") { "$set": { "integers.2": (3) } } };
    let expected = doc! { "$set": { "integers.2": 3i64 } };
    assert_eq!(query.into_doc(), expected);

    let query = huus_macros::update! { ("coll_3") { "$push": { "array.array": "1" } } };
    let expected = doc! { "$push": { "array.array": "1" } };
    assert_eq!(query.into_doc(), expected);

    let string = "1".to_string();
    let query = huus_macros::update! { ("coll_3") { "$push": { "array.array": (string) } } };
    let expected = doc! { "$push": { "array.array": "1" } };
    assert_eq!(query.into_doc(), expected);

    let array = vec!["1".to_string(), "2".to_string()];
    let query = huus_macros::update! { ("coll_3")
        "$push": {
            "array": {
                "array": (array),
                "str": "string",
            }
        }
    };
    let expected = doc! { "$push": { "array": { "array": ["1", "2"], "str": "string" } } };
    assert_eq!(query.into_doc(), expected);
}

// -------------------------------------------------------------------------------------------------
// Creating queries

/// Verify index creation query.
#[test]
fn create_indexes_query() {
    use huus::query::Query;

    let indexed = vec![
        "data.str".to_string(),
        "array.str".to_string(),
        "nested_map.choice_1.str".to_string(),
        "nested_map.choice_2.str".to_string(),
        "indexed".to_string(),
    ];
    let command = huus::commands::CreateIndexesCommand::new("coll_3".to_string(), indexed);
    assert_eq!(Coll3::create_indexes(), command);
}

/// Verify query fetching all entries.
#[test]
fn fetch_all_query() {
    use bson::doc;
    use huus::query::Query;

    let command = huus::commands::FindCommand::new("coll_2".to_string(), doc!(), None);
    assert_eq!(Coll2::fetch_all(), command);
}

/// Verify `find one` query.
#[test]
fn find_one_query() {
    use bson::{bson, doc};
    use huus::query::Query;

    let filter = huus_macros::filter! { ("coll_2")
        "data.int": 1,
        "data.str": "abc",
        "str": "def",
    };
    let command = huus::commands::FindOneCommand::new(
        "coll_2".to_string(),
        doc! { "data.int": 1, "data.str": "abc", "str": "def" },
    );
    assert_eq!(Coll2::find_one(filter), command);
}

/// Verify `find` query.
#[test]
fn find_many_query() {
    use bson::{bson, doc};
    use huus::query::Query;

    let filter = huus_macros::filter! { ("coll_2")
        "data.int": 1,
        "data.str": "abc",
        "str": "def",
    };
    let command = huus::commands::FindCommand::new(
        "coll_2".to_string(),
        doc! { "data.int": 1, "data.str": "abc", "str": "def" },
        None,
    );
    assert_eq!(Coll2::find(filter), command);
}

/// Verify text search query.
#[test]
fn text_search_query() {
    use bson::{bson, doc};
    use huus::query::Query;

    let command = huus::commands::FindCommand::new(
        "coll_2".to_string(),
        doc! { "$text": { "$search": "my_pattern" } },
        None,
    );
    assert_eq!(Coll2::text_search("my_pattern".to_string()), command);
}

/// Verify insert query.
#[test]
fn insert_query() {
    use bson::{bson, doc};
    use huus::query::Query;

    let data = huus_macros::data! { ("coll_2")
        "data": {
            "int": 1,
            "str": "abc",
        },
        "str": "def",
    };

    let command = Coll2::insert(data);
    let actual = command.get_document();
    assert_eq!(*actual.get_document("data").unwrap(), doc! { "int": 1, "str": "abc" });
    assert_eq!(*actual.get_str("str").unwrap(), "def".to_string());
}

/// Verify update query.
#[test]
fn update_query() {
    use bson::{bson, doc};
    use huus::query::Query;

    let filter = huus_macros::filter! { ("coll_2")
        "data.int": 1,
        "data.str": "abc",
        "str": "def",
    };

    let update = huus_macros::update! { ("coll_2")
        "data": {
            "int": 1,
            "str": "abc",
        },
        "str": "def",
    };

    let command = huus::commands::UpdateCommand::new(
        "coll_2".to_string(),
        doc! { "data.int": 1, "data.str": "abc", "str": "def" },
        doc! { "data": { "int": 1, "str": "abc" }, "str": "def" },
        huus::commands::UpdateOptions::UpdateOne,
    );

    assert_eq!(Coll2::update(filter, update), command);
}

