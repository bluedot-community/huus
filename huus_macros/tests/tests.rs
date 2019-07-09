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
        simple_map: BTreeMap Enum1 String,
        nested_map: BTreeMap Enum1 Doc1,
        boolean: bool,
        date: Date,
        indexed: String+,
        integer: Vec i64,
        choice: Enum1,
        union: Union1,
    }
}

#[test]
fn test_data_contents() {
    use bson::{bson, doc};
    use huus::conversions::IntoDoc;

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
            Enum1Data::Choice1 => "one".to_string(),
            Enum1Data::Choice2 => "two".to_string(),
        },
        nested_map: maplit::btreemap! {
            Enum1Data::Choice1 => Doc1Data { integer: Some(4), string: "jkl".to_string() },
            Enum1Data::Choice2 => Doc1Data { integer: Some(5), string: "mno".to_string() },
        },
        boolean: true,
        date: date,
        indexed: "indexed".to_string(),
        integer: vec![4, 7],
        choice: Enum1Data::Choice1,
        union: Union1Data::Choice1(Doc1Data { integer: Some(6), string: "pqr".to_string() }),
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
        "integer": [4i64, 7i64],
        "choice": "choice_1",
        "union": {
            "int": 6i32,
            "str": "pqr",
            "_huus_variant": "choice_1"
        }
    };
    assert_eq!(data.into_doc(), expected);
}

#[test]
fn test_filter_contents_by_assign() {
    use bson::{bson, doc};
    use huus::filters::BuildFilter;

    let object_id = huus::types::ObjectId::new().unwrap();
    let date = chrono::Utc::now();
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
            Enum1Data::Choice1 => "one".to_string(),
            Enum1Data::Choice2 => "two".to_string(),
        }),
        nested_map: huus::filters::BTreeMapEntry::Value(maplit::btreemap! {
            Enum1Data::Choice1 => Doc1Data { integer: Some(4), string: "jkl".to_string() },
            Enum1Data::Choice2 => Doc1Data { integer: Some(5), string: "mno".to_string() },
        }),
        boolean: huus::filters::BooleanEntry::Value(true),
        date: huus::filters::DateEntry::Value(date),
        indexed: huus::filters::StringEntry::Value("indexed".to_string()),
        integer: huus::filters::ArrayEntry::Array(huus::filters::Array::All(vec![4, 7])),
        choice: huus::filters::EnumEntry::Value(Enum1Data::Choice1),
        union: huus::filters::ObjectEntry::Value(Union1Data::Choice1(Doc1Data {
            integer: Some(6),
            string: "pqr".to_string(),
        })),
    };

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
            Enum1Data::Choice1 => "one".to_string(),
            Enum1Data::Choice2 => "two".to_string(),
        }),
        nested_map: huus::filters::BTreeMapEntry::Value(maplit::btreemap! {
            Enum1Data::Choice1 => Doc1Data { integer: Some(4), string: "jkl".to_string() },
            Enum1Data::Choice2 => Doc1Data { integer: Some(5), string: "mno".to_string() },
        }),
        boolean: true.into(),
        date: date.into(),
        indexed: "indexed".into(),
        integer: vec![4.into(), 7.into()].into(),
        choice: huus::filters::EnumEntry::Value(Enum1Data::Choice1),
        union: huus::filters::ObjectEntry::Value(Union1Data::Choice1(Doc1Data {
            integer: Some(6),
            string: "pqr".to_string(),
        })),
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
        "integer": { "$all": [4i64, 7i64] },
        "choice": "choice_1",
        "union": {
            "int": 6,
            "str": "pqr",
            "_huus_variant": "choice_1",
        }
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
        filter.integer.all(vec![4.into(), 7.into()]);

        let expected = doc! {
            "data.int": { "$exists": true },
            "boolean": true,
            "integer": { "$all": [4i64, 7i64] },
        };

        assert_eq!(filter.build_filter().into_doc(), expected);
    }
    {
        let data = Doc1Data { integer: 3.into(), string: "ghi".into() };
        let mut filter = Doc3Filter::default();
        filter.data = Value(data);
        filter.boolean.exists(false);
        filter.integer.all(vec![4.into(), 7.into()]);

        let expected = doc! {
            "data": { "int": 3, "str": "ghi" },
            "boolean": { "$exists": false },
            "integer": { "$all": [4i64, 7i64] },
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
        object_id: Some(object_id.clone()),
        data: Some(Doc1Value { integer: Some(1), string: Some("abc".to_string()) }),
        array: Some(huus::values::Array::Array(vec![Doc1Data {
            integer: Some(2),
            string: "def".to_string(),
        }])),
        simple_map: Some(maplit::btreemap! {
            Enum1Value::Choice1 => "one".to_string(),
            Enum1Value::Choice2 => "two".to_string(),
        }),
        nested_map: Some(maplit::btreemap! {
            Enum1Value::Choice1 => Doc1Data {
                integer: Some(4),
                string: "jkl".to_string(),
            },
            Enum1Value::Choice2 => Doc1Data {
                integer: Some(5),
                string: "mno".to_string(),
            },
        }),
        boolean: Some(true),
        date: Some(date),
        indexed: Some("indexed".to_string()),
        integer: Some(huus::values::Array::Array(vec![4, 5])),
        choice: Some(Enum1Value::Choice1),
        union: Some(Union1Value::Choice1(Doc1Value {
            integer: Some(6),
            string: Some("pqr".to_string()),
        })),
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
        "indexed": "indexed",
        "integer": [4i64, 5i64],
        "choice": "choice_1",
        "union": {
            "int": 6i32,
            "str": "pqr",
            "_huus_variant": "choice_1",
        }
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
    value.object_id = Some(object_id.clone());
    value.boolean = Some(true);
    value.date = Some(date);
    value.integer = Some(huus::values::Array::Array(vec![4, 5]));
    value.choice = Some(Enum1Value::Choice1);

    let expected = bson!({
        "_id": object_id,
        "boolean": true,
        "date": date,
        "integer": [4i64, 5i64],
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
    let update1 = Doc3Update {
        object_id: huus::updates::ObjectIdEntry::Value(object_id.clone()),
        data: huus::updates::ObjectEntry::Dot(Doc1Update {
            integer: huus::updates::I32Entry::Value(1),
            string: huus::updates::StringEntry::Value("abc".to_string()),
        }),
        array: huus::updates::ArrayEntry::Array(
            huus::updates::Array::Push(Doc1Value {
                integer: Some(2),
                string: Some("def".to_string()),
            }),
            huus::updates::Operator::None,
        ),
        simple_map: huus::updates::BTreeMapEntry::Value(maplit::btreemap! {
            Enum1Data::Choice1 => "one".to_string(),
            Enum1Data::Choice2 => "two".to_string(),
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
        integer: huus::updates::ArrayEntry::Array(
            huus::updates::Array::Push(4),
            huus::updates::Operator::First,
        ),
        choice: huus::updates::EnumEntry::Value(Enum1Value::Choice1),
        union: huus::updates::ObjectEntry::Dot(Union1Update::Choice1(Doc1Update {
            integer: huus::updates::I32Entry::Value(6),
            string: huus::updates::StringEntry::Value("pqr".to_string()),
        })),
    };

    let update2 = Doc3Update {
        object_id: object_id.clone().into(),
        data: huus::updates::ObjectEntry::Dot(Doc1Update {
            integer: 1.into(),
            string: "abc".into(),
        }),
        array: huus::updates::ArrayEntry::Array(
            huus::updates::Array::Push(Doc1Value {
                integer: Some(2),
                string: Some("def".to_string()),
            }),
            huus::updates::Operator::None,
        ),
        simple_map: huus::updates::BTreeMapEntry::Value(maplit::btreemap! {
            Enum1Data::Choice1 => "one".to_string(),
            Enum1Data::Choice2 => "two".to_string(),
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
        integer: huus::updates::ArrayEntry::Array(
            huus::updates::Array::Push(4),
            huus::updates::Operator::First,
        ),
        choice: huus::updates::EnumEntry::Value(Enum1Value::Choice1),
        union: huus::updates::ObjectEntry::Dot(Union1Update::Choice1(Doc1Update {
            integer: huus::updates::I32Entry::Value(6),
            string: huus::updates::StringEntry::Value("pqr".to_string()),
        })),
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
        "$push": {
            "integer.$": 4i64,
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
    update.integer.push(4, Operator::First);

    let expected = doc! {
        "data.str": "abc",
        "$max": { "data.int": 40i32 },
        "$set": { "boolean": true },
        "$push": { "integer.$": 4i64 },
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
        let command = huus::commands::FindCommand::new(
            "abc_collection".to_string(),
            doc! { "data.int": 1, "data.str": "abc", "string": "def" },
            Some(1),
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
        let command = huus::commands::FindCommand::new(
            "abc_collection".to_string(),
            doc! { "data": { "int": 1, "str": "abc" }, "string": "def" },
            Some(1),
        );
        assert_eq!(filter.find_one(), command);
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

    let command = huus::commands::InsertCommand::new(
        "abc_collection".to_string(),
        doc! { "data": { "int": 1, "str": "abc" }, "string": "def" },
    );

    assert_eq!(data.insert(), command);
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
