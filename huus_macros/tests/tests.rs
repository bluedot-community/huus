// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Tests for `huus_macros` crate.

#[macro_use]
extern crate maplit;

use huus::models::prelude::*;

huus_macros::define_huus! {
    pub enum Enum1 {
        Choice1 as "choice_1",
        Choice2 as "choice_2",
    }

    pub struct Doc1 {
        integer as "int": i32?,
        string as "str": String+,
    }

    pub enum Union1 {
        Choice1 as "choice_1": Doc1,
        Choice2 as "choice_2": Doc1,
    }

    pub struct Doc2 in "abc_collection" {
        data: Doc1?,
        string: String?,
    }

    pub struct Doc3 in "abc_collection" {
        object_id as "_id": ObjectId,
        data: Doc1,
        array: Vec Doc1,
        simple_map: BTreeMap String String,
        nested_map: BTreeMap Enum1 Doc1,
        boolean: bool,
        date: Date,
        indexed: String+,
        integers: Vec i64,
        choice: Enum1,
        union: Union1,
        bson: Bson,
    }
}

#[test]
fn test_data_contents() {
    use bson::{bson, doc};
    use huus::conversions::{IntoDoc, HuusFromBson};

    let object_id = huus::types::ObjectId::new().unwrap();
    let date = chrono::Utc::now();
    let data = Doc3Data {
        object_id: object_id.clone(),
        data: Doc1Data { integer: Some(1), string: "abc".to_string() },
        array: vec![
            Doc1Data { integer: Some(2), string: "def".to_string() },
            Doc1Data { integer: Some(3), string: "ghi".to_string() },
        ],
        simple_map: maplit::btreemap! {
            "choice_1".to_string() => "one".to_string(),
            "choice_2".to_string() => "two".to_string(),
        },
        nested_map: maplit::btreemap! {
            Enum1Data::Choice1 => Doc1Data { integer: Some(4), string: "jkl".to_string() },
            Enum1Data::Choice2 => Doc1Data { integer: Some(5), string: "mno".to_string() },
        },
        boolean: true,
        date: date,
        indexed: "indexed".to_string(),
        integers: vec![4, 7],
        choice: Enum1Data::Choice1,
        union: Union1Data::Choice1(Doc1Data { integer: Some(6), string: "pqr".to_string() }),
        bson: doc! { "a": 1, "b": 2 },
    };
    let expected = doc! {
        "_id": object_id,
        "data": { "int": 1i32, "str": "abc" },
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
        "date": date,
        "indexed": "indexed",
        "integers": [4i64, 7i64],
        "choice": "choice_1",
        "union": {
            "int": 6i32,
            "str": "pqr",
            "_huus_variant": "choice_1"
        },
        "bson": { "a": 1, "b": 2 },
    };
    assert_eq!(data.clone().into_doc(), expected);
    assert_eq!(data, Doc3Data::huus_from_bson(bson::Bson::Document(expected)).unwrap());
}

#[test]
fn test_filter_contents_by_assign() {
    use bson::{bson, doc};
    use huus::filters::BuildFilter;

    let object_id = huus::types::ObjectId::new().unwrap();
    let date = chrono::Utc::now();

    // Assign using explicit cast to `value`.
    let filter1 = Doc3Filter {
        object_id: huus::filters::ObjectIdEntry::Value(object_id.clone()),
        data: huus::filters::ObjectEntry::Dot(Doc1Filter {
            integer: huus::filters::I32Entry::Value(1),
            string: huus::filters::StringEntry::Value("abc".to_string()),
        }),
        array: huus::filters::ArrayEntry::Array(huus::filters::Array::All(vec![
            Doc1Data { integer: 2.into(), string: "def".into() },
            Doc1Data { integer: 3.into(), string: "ghi".into() },
        ])),
        simple_map: huus::filters::BTreeMapEntry::Value(maplit::btreemap! {
            "choice_1".to_string() => "one".to_string(),
            "choice_2".to_string() => "two".to_string(),
        }),
        nested_map: huus::filters::BTreeMapEntry::Value(maplit::btreemap! {
            Enum1Data::Choice1 => Doc1Data { integer: Some(4), string: "jkl".to_string() },
            Enum1Data::Choice2 => Doc1Data { integer: Some(5), string: "mno".to_string() },
        }),
        boolean: huus::filters::BooleanEntry::Value(true),
        date: huus::filters::DateEntry::Value(date),
        indexed: huus::filters::StringEntry::Value("indexed".to_string()),
        integers: huus::filters::ArrayEntry::Array(huus::filters::Array::All(vec![4, 7])),
        choice: huus::filters::EnumEntry::Value(Enum1Data::Choice1),
        union: huus::filters::ObjectEntry::Value(Union1Data::Choice1(Doc1Data {
            integer: Some(6),
            string: "pqr".to_string(),
        })),
        bson: huus::filters::BsonEntry::Value(doc! { "a": 1, "b": 2 }),
    };

    // Assign using implicit cast (`into`) to `value` (where possible).
    let filter2 = Doc3Filter {
        object_id: object_id.clone().into(),
        data: huus::filters::ObjectEntry::Dot(Doc1Filter {
            integer: 1.into(),
            string: "abc".into(),
        }),
        array: vec![
            Doc1Data { integer: 2.into(), string: "def".into() },
            Doc1Data { integer: 3.into(), string: "ghi".into() },
        ]
        .into(),
        simple_map: huus::filters::BTreeMapEntry::Value(maplit::btreemap! {
            "choice_1".to_string() => "one".to_string(),
            "choice_2".to_string() => "two".to_string(),
        }),
        nested_map: huus::filters::BTreeMapEntry::Value(maplit::btreemap! {
            Enum1Data::Choice1 => Doc1Data { integer: Some(4), string: "jkl".to_string() },
            Enum1Data::Choice2 => Doc1Data { integer: Some(5), string: "mno".to_string() },
        }),
        boolean: true.into(),
        date: date.into(),
        indexed: "indexed".into(),
        integers: vec![4.into(), 7.into()].into(),
        choice: huus::filters::EnumEntry::Value(Enum1Data::Choice1),
        union: huus::filters::ObjectEntry::Value(Union1Data::Choice1(Doc1Data {
            integer: Some(6),
            string: "pqr".to_string(),
        })),
        bson: doc! { "a": 1, "b": 2 }.into(),
    };

    let expected = doc! {
        "_id": object_id,
        "data.int": 1i32,
        "data.str": "abc",
        "array": { "$all": [
            { "int": 2i32, "str": "def" },
            { "int": 3i32, "str": "ghi" },
        ]},
        "simple_map": {
            "choice_1": "one",
            "choice_2": "two",
        },
        "nested_map": {
            "choice_1": { "int": 4i32, "str": "jkl" },
            "choice_2": { "int": 5i32, "str": "mno" },
        },
        "boolean": true,
        "date": date,
        "indexed": "indexed",
        "integers": { "$all": [4i64, 7i64] },
        "choice": "choice_1",
        "union": {
            "int": 6,
            "str": "pqr",
            "_huus_variant": "choice_1",
        },
        "bson": { "a": 1, "b": 2 },
    };

    assert_eq!(filter1.build_filter().into_doc(), expected);
    assert_eq!(filter2.build_filter().into_doc(), expected);
}

#[test]
fn test_filter_contents_by_modification() {
    use bson::{bson, doc};
    use huus::filters::ObjectEntry::{Dot, Value};
    use huus::filters::{ArrayFilter, BuildFilter, ElementFilter};

    {
        let filter = Doc3Filter::default();
        let expected = doc! {};
        assert_eq!(filter.build_filter().into_doc(), expected);
    }
    {
        let mut data = Doc1Filter::default();
        data.integer.exists(true);

        let mut filter = Doc3Filter::default();
        filter.data = Dot(data);
        filter.boolean = true.into();
        filter.integers.all(vec![4.into(), 7.into()]);

        let expected = doc! {
            "data.int": { "$exists": true },
            "boolean": true,
            "integers": { "$all": [4i64, 7i64] },
        };

        assert_eq!(filter.build_filter().into_doc(), expected);
    }
    {
        let data = Doc1Data { integer: 3.into(), string: "ghi".into() };
        let mut filter = Doc3Filter::default();
        filter.data = Value(data);
        filter.boolean.exists(false);
        filter.integers.all(vec![4.into(), 7.into()]);

        let expected = doc! {
            "data": { "int": 3, "str": "ghi" },
            "boolean": { "$exists": false },
            "integers": { "$all": [4i64, 7i64] },
        };

        assert_eq!(filter.build_filter().into_doc(), expected);
    }
}

#[test]
fn test_value_contents_by_assign() {
    use bson::{bson, doc};
    use huus::values::BuildValue;

    let object_id = huus::types::ObjectId::new().unwrap();
    let date = chrono::Utc::now();
    let value = Doc3Value {
        object_id: object_id.clone().into(),
        data: Doc1Value { integer: 1.into(), string: "abc".to_string().into() }.into(),
        array: vec![Doc1Value { integer: 2.into(), string: "def".to_string().into() }].into(),
        simple_map: maplit::btreemap! {
            "choice_1".to_string() => "one".to_string(),
            "choice_2".to_string() => "two".to_string(),
        }
        .into(),
        nested_map: maplit::btreemap! {
            Enum1Value::Choice1 => Doc1Data {
                integer: 4.into(),
                string: "jkl".to_string(),
            },
            Enum1Value::Choice2 => Doc1Data {
                integer: 5.into(),
                string: "mno".to_string(),
            },
        }
        .into(),
        boolean: true.into(),
        date: date.into(),
        indexed: vec!["indexed".to_string()].into(),
        integers: vec![4, 5].into(),
        choice: Enum1Value::Choice1.into(),
        union: Union1Value::Choice1(Doc1Value {
            integer: 6.into(),
            string: "pqr".to_string().into(),
        })
        .into(),
        bson: doc! { "a": 1, "b": 2 }.into(),
    };
    let expected = bson!({
        "_id": object_id,
        "data": {
            "int": 1i32,
            "str": "abc",
        },
        "array": [{
            "int": 2i32,
            "str": "def",
        }],
        "simple_map": {
            "choice_1": "one",
            "choice_2": "two",
        },
        "nested_map": {
            "choice_1": { "int": 4i32, "str": "jkl" },
            "choice_2": { "int": 5i32, "str": "mno" },
        },
        "boolean": true,
        "date": date,
        "indexed": { "$in": ["indexed"] },
        "integers": [4i64, 5i64],
        "choice": "choice_1",
        "union": {
            "int": 6i32,
            "str": "pqr",
            "_huus_variant": "choice_1",
        },
        "bson": { "a": 1, "b": 2 },
    });
    assert_eq!(value.build_value().into_bson(), expected);
}

#[test]
fn test_value_contents_by_modification() {
    use bson::{bson, doc};
    use huus::values::BuildValue;

    let object_id = huus::types::ObjectId::new().unwrap();
    let date = chrono::Utc::now();

    let mut value = Doc3Value::default();
    value.object_id = object_id.clone().into();
    value.boolean = true.into();
    value.date = date.into();
    value.integers = vec![4, 5].into();
    value.choice = Enum1Value::Choice1.into();

    let expected = bson!({
        "_id": object_id,
        "boolean": true,
        "date": date,
        "integers": [4i64, 5i64],
        "choice": "choice_1",
    });
    assert_eq!(value.build_value().into_bson(), expected);
}

#[test]
fn test_update_contents_by_assign() {
    use bson::{bson, doc};
    use huus::updates::BuildUpdate;

    let object_id = huus::types::ObjectId::new().unwrap();
    let date = chrono::Utc::now();

    // Assign using explicit cast to `value`.
    let update1 = Doc3Update {
        object_id: huus::updates::ObjectIdEntry::Value(object_id.clone()),
        data: huus::updates::ObjectEntry::Dot(Doc1Update {
            integer: huus::updates::I32Entry::Value(1),
            string: huus::updates::StringEntry::Value("abc".to_string()),
        }),
        array: huus::updates::ArrayEntry::Array(
            huus::updates::Array::Push(huus::values::PushValue::Value(Doc1Value {
                integer: 2.into(),
                string: "def".to_string().into(),
            })),
            huus::updates::Operator::None,
        ),
        simple_map: huus::updates::BTreeMapEntry::Value(maplit::btreemap! {
            "choice_1".to_string() => "one".to_string(),
            "choice_2".to_string() => "two".to_string(),
        }),
        nested_map: huus::updates::BTreeMapEntry::Value(maplit::btreemap! {
            Enum1Data::Choice1 => Doc1Data {
                integer: Some(4),
                string: "jkl".to_string(),
            },
            Enum1Data::Choice2 => Doc1Data {
                integer: Some(5),
                string: "mno".to_string(),
            },
        }),
        boolean: huus::updates::BooleanEntry::Value(true),
        date: huus::updates::DateEntry::Value(date),
        indexed: huus::updates::StringEntry::Value("indexed".to_string()),
        integers: huus::updates::ArrayEntry::Array(
            huus::updates::Array::Push(huus::values::PushValue::Value(4)),
            huus::updates::Operator::First,
        ),
        choice: huus::updates::EnumEntry::Value(Enum1Value::Choice1),
        union: huus::updates::ObjectEntry::Dot(Union1Update::Choice1(Doc1Update {
            integer: huus::updates::I32Entry::Value(6),
            string: huus::updates::StringEntry::Value("pqr".to_string()),
        })),
        bson: huus::updates::BsonEntry::Value(doc! { "a": 1, "b": 2 }),
    };

    // Assign using implicit cast (`into`) to `value` (where possible).
    let update2 = Doc3Update {
        object_id: object_id.clone().into(),
        data: huus::updates::ObjectEntry::Dot(Doc1Update {
            integer: 1.into(),
            string: "abc".into(),
        }),
        array: huus::updates::ArrayEntry::Array(
            huus::updates::Array::Push(
                Doc1Value { integer: 2.into(), string: "def".to_string().into() }.into(),
            ),
            huus::updates::Operator::None,
        ),
        simple_map: huus::updates::BTreeMapEntry::Value(maplit::btreemap! {
            "choice_1".to_string() => "one".to_string(),
            "choice_2".to_string() => "two".to_string(),
        }),
        nested_map: huus::updates::BTreeMapEntry::Value(maplit::btreemap! {
            Enum1Data::Choice1 => Doc1Data {
                integer: Some(4),
                string: "jkl".to_string(),
            },
            Enum1Data::Choice2 => Doc1Data {
                integer: Some(5),
                string: "mno".to_string(),
            },
        }),
        boolean: true.into(),
        date: date.into(),
        indexed: "indexed".into(),
        integers: huus::updates::ArrayEntry::Array(
            huus::updates::Array::Push(4.into()),
            huus::updates::Operator::First,
        ),
        choice: huus::updates::EnumEntry::Value(Enum1Value::Choice1),
        union: huus::updates::ObjectEntry::Dot(Union1Update::Choice1(Doc1Update {
            integer: huus::updates::I32Entry::Value(6),
            string: huus::updates::StringEntry::Value("pqr".to_string()),
        })),
        bson: doc! { "a": 1, "b": 2 }.into(),
    };

    let expected = doc! {
        "data.int": 1i32,
        "data.str": "abc",
        "simple_map": {
            "choice_1": "one",
            "choice_2": "two",
        },
        "nested_map": {
            "choice_1": { "int": 4i32, "str": "jkl" },
            "choice_2": { "int": 5i32, "str": "mno" },
        },
        "boolean": true,
        "date": date,
        "indexed": "indexed",
        "choice": "choice_1",
        "union.int": 6i32,
        "union.str": "pqr",
        "union._huus_variant": "choice_1",
        "bson": { "a": 1, "b": 2 },
        "$push": {
            "integers.$": 4i64,
            "array": {
                "int": 2i32,
                "str": "def",
            },
        },
    };
    assert_eq!(update1.build_update().into_doc(), expected);
    assert_eq!(update2.build_update().into_doc(), expected);
}

#[test]
fn test_update_contents_by_modification() {
    use bson::{bson, doc};
    use huus::updates::Operator;
    use huus::updates::{ArrayUpdate, BuildUpdate, FieldUpdate, NumericalUpdate, ObjectUpdate};

    let update = Doc3Update::default();
    let expected = doc! {};
    assert_eq!(update.build_update().into_doc(), expected);

    let mut update1 = Doc1Update::default();
    update1.integer.max(40);
    update1.string = "abc".into();

    let mut update = Doc3Update::default();
    update.data.dot(update1);
    update.boolean.set(true);
    update.integers.push(4.into(), Operator::First);

    let expected = doc! {
        "data.str": "abc",
        "$max": { "data.int": 40i32 },
        "$set": { "boolean": true },
        "$push": { "integers.$": 4i64 },
    };

    assert_eq!(update.build_update().into_doc(), expected);
}

#[test]
fn test_create_indexes_query() {
    use huus::query::Query;

    let indexed = vec![
        "data.str".to_string(),
        "array.str".to_string(),
        "nested_map.choice_1.str".to_string(),
        "nested_map.choice_2.str".to_string(),
        "indexed".to_string(),
    ];
    let command = huus::commands::CreateIndexesCommand::new("abc_collection".to_string(), indexed);
    assert_eq!(Doc3Data::create_indexes(), command);
}

#[test]
fn test_fetch_all_query() {
    use bson::doc;
    use huus::query::Query;

    let command = huus::commands::FindCommand::new("abc_collection".to_string(), doc!(), None);
    assert_eq!(Doc2Filter::fetch_all(), command);
}

#[test]
fn test_find_one_query() {
    use bson::{bson, doc};
    use huus::query::Query;

    {
        let filter = Doc2Filter {
            data: huus::filters::ObjectEntry::Dot(Doc1Filter {
                integer: 1.into(),
                string: "abc".into(),
            }),
            string: "def".into(),
        };
        let command = huus::commands::FindOneCommand::new(
            "abc_collection".to_string(),
            doc! { "data.int": 1, "data.str": "abc", "string": "def" },
        );
        assert_eq!(filter.find_one(), command);
    }

    {
        let filter = Doc2Filter {
            data: huus::filters::ObjectEntry::Value(Doc1Data {
                integer: 1.into(),
                string: "abc".into(),
            }),
            string: "def".into(),
        };
        let command = huus::commands::FindOneCommand::new(
            "abc_collection".to_string(),
            doc! { "data": { "int": 1, "str": "abc" }, "string": "def" },
        );
        assert_eq!(filter.find_one(), command);
    }
}

#[test]
fn test_find_many_query() {
    use bson::{bson, doc};
    use huus::query::Query;

    {
        let filter = Doc2Filter {
            data: huus::filters::ObjectEntry::Dot(Doc1Filter {
                integer: 1.into(),
                string: "abc".into(),
            }),
            string: "def".into(),
        };
        let command = huus::commands::FindCommand::new(
            "abc_collection".to_string(),
            doc! { "data.int": 1, "data.str": "abc", "string": "def" },
            None,
        );
        assert_eq!(filter.find(), command);
    }

    {
        let filter = Doc2Filter {
            data: huus::filters::ObjectEntry::Value(Doc1Data {
                integer: 1.into(),
                string: "abc".into(),
            }),
            string: "def".into(),
        };
        let command = huus::commands::FindCommand::new(
            "abc_collection".to_string(),
            doc! { "data": { "int": 1, "str": "abc" }, "string": "def" },
            None,
        );
        assert_eq!(filter.find(), command);
    }
}

#[test]
fn test_text_search_query() {
    use bson::{bson, doc};
    use huus::query::Query;

    let command = huus::commands::FindCommand::new(
        "abc_collection".to_string(),
        doc! { "$text": { "$search": "my_pattern" } },
        None,
    );
    assert_eq!(Doc2Filter::text_search("my_pattern".to_string()), command);
}

#[test]
fn test_insert_query() {
    use bson::{bson, doc};
    use huus::query::Query;

    let data = Doc2Data {
        data: Some(Doc1Data { integer: Some(1), string: "abc".to_string() }),
        string: Some("def".to_string()),
    };

    let command = data.insert();
    let actual = command.get_document();
    assert_eq!(*actual.get_document("data").unwrap(), doc! { "int": 1, "str": "abc" });
    assert_eq!(*actual.get_str("string").unwrap(), "def".to_string());
}

#[test]
fn test_update_query() {
    use bson::{bson, doc};
    use huus::query::Query;

    let filter = Doc2Filter {
        data: huus::filters::ObjectEntry::Dot(Doc1Filter {
            integer: 1.into(),
            string: "abc".into(),
        }),
        string: "def".into(),
    };

    let update = Doc2Update {
        data: huus::updates::ObjectEntry::Dot(Doc1Update {
            integer: 1.into(),
            string: "abc".into(),
        }),
        string: "def".into(),
    };

    let command = huus::commands::UpdateCommand::new(
        "abc_collection".to_string(),
        doc! { "data.int": 1, "data.str": "abc", "string": "def" },
        doc! { "data.int": 1, "data.str": "abc", "string": "def" },
        huus::commands::UpdateOptions::UpdateOne,
    );

    assert_eq!(filter.update(update), command);
}
