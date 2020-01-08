// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Tests of user-defined structures from `updates` module.

use bson::{bson, doc};

use huus::updates::{BuildInnerUpdate, BuildUpdate, Operator};
use huus::{updates, values};

const KEY: &'static str = "xxx";

// -------------------------------------------------------------------------------------------------

#[derive(Clone)]
struct DataValue1 {
    int: i32,
    string: String,
}

impl values::BuildValue for DataValue1 {
    fn build_value(self) -> values::Value {
        let mut value = values::ObjectValue::empty();
        value.insert("int".to_string(), self.int.build_value().into_bson());
        value.insert("string".to_string(), self.string.build_value().into_bson());
        value.build_value()
    }
}

#[derive(Clone)]
struct DataUpdate1 {
    int: updates::I32Entry,
    string: updates::StringEntry,
}

impl BuildInnerUpdate for DataUpdate1 {
    fn build_update(self, field: String) -> updates::Update {
        let mut update = updates::Update::empty();
        update.incorporate(self.int.build_update(field.clone() + ".int"));
        update.incorporate(self.string.build_update(field.clone() + ".string"));
        update
    }
}

#[derive(Clone)]
struct DataUpdate2 {
    data: updates::ObjectEntry<DataUpdate1, DataValue1>,
    array: updates::ArrayEntry<DataUpdate1, DataValue1>,
}

impl BuildUpdate for DataUpdate2 {
    fn build_update(self) -> updates::Update {
        let mut update = updates::Update::empty();
        update.incorporate(self.data.build_update("data".to_string()));
        update.incorporate(self.array.build_update("array".to_string()));
        update
    }
}

// -------------------------------------------------------------------------------------------------

#[test]
fn test_simple_object_entry_update() {
    let data = DataValue1 { int: 2, string: "abc".to_string() };

    let update = DataUpdate1 {
        int: updates::I32Entry::Value(2),
        string: updates::StringEntry::Value("abc".to_string()),
    };

    let entry = updates::ObjectEntry::Value::<DataUpdate1, _>(data.clone());
    let expected = doc! { KEY: { "int": 2, "string": "abc" } };
    assert_eq!(entry.build_update(KEY.to_string()).into_doc(), expected);

    let entry = updates::ObjectEntry::Dot::<_, DataValue1>(update.clone());
    let expected = doc! { "xxx.int": 2, "xxx.string": "abc" };
    assert_eq!(entry.build_update(KEY.to_string()).into_doc(), expected);

    let entry = updates::ObjectEntry::Field::<DataUpdate1, _>(updates::Field::Set(data.clone()));
    let expected = doc! { "$set": { "xxx": { "int": 2, "string": "abc" } } };
    assert_eq!(entry.build_update(KEY.to_string()).into_doc(), expected);
}

#[test]
fn test_object_entry_update_nested_with_dot() {
    let object = DataUpdate2 {
        data: updates::ObjectEntry::Dot(DataUpdate1 {
            int: updates::I32Entry::Value(2),
            string: updates::StringEntry::Value("abc".to_string()),
        }),
        array: updates::ArrayEntry::Array(
            updates::Array::Pop(updates::PopOption::First),
            Operator::None,
        ),
    };
    let expected = doc! {
        "data.int": 2,
        "data.string": "abc",
        "$pop": { "array": -1 },
    };

    assert_eq!(object.build_update().into_doc(), expected);
}

#[test]
fn test_object_entry_update_nested_with_value() {
    let object = DataUpdate2 {
        data: updates::ObjectEntry::Value(DataValue1 { int: 2, string: "abc".to_string() }),
        array: updates::ArrayEntry::Array(
            updates::Array::Pull(values::PullValue::Value(DataValue1 {
                int: 3,
                string: "def".to_string(),
            })),
            Operator::None,
        ),
    };
    let expected = doc! {
        "data": { "int": 2, "string": "abc" },
        "$pull": { "array": { "int": 3, "string": "def" } },
    };

    assert_eq!(object.build_update().into_doc(), expected);
}

#[test]
fn test_object_entry_update_nested_indexed() {
    let object1 = DataUpdate2 {
        data: updates::ObjectEntry::Empty,
        array: updates::ArrayEntry::Indexed(1, DataUpdate1 {
            int: updates::I32Entry::Value(2),
            string: updates::StringEntry::Value("abc".to_string()),
        }),
    };

    let object2 = DataUpdate2 {
        data: updates::ObjectEntry::Empty,
        array: updates::ArrayEntry::Selected(DataUpdate1 {
            int: updates::I32Entry::Field(updates::Field::Set(2)),
            string: updates::StringEntry::Value("abc".to_string()),
        }),
    };

    let expected1 = doc! {
        "array.1.int": 2,
        "array.1.string": "abc",
    };

    let expected2 = doc! {
        "array.$.string": "abc",
        "$set": { "array.$.int": 2 }
    };

    assert_eq!(object1.build_update().into_doc(), expected1);
    assert_eq!(object2.build_update().into_doc(), expected2);
}
