// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Tests of user-defined structures from `filters` module.

use bson::{bson, doc};

use huus::conversions::HuusIntoBson;
use huus::filters::*;

const KEY: &'static str = "xxx";

#[derive(Clone, Debug)]
struct Data1 {
    int: i32,
    string: String,
}

impl HuusIntoBson for Data1 {
    fn huus_into_bson(self) -> bson::Bson {
        let mut doc = bson::Document::new();
        doc.insert("int".to_string(), self.int);
        doc.insert("string".to_string(), self.string);
        bson::Bson::Document(doc)
    }
}

#[derive(Clone, Debug)]
struct DataFilter1 {
    int: I32Entry,
    string: StringEntry,
}

impl BuildInnerFilter for DataFilter1 {
    fn build_filter(self, field: String) -> Filter {
        let mut filter = Filter::empty();
        filter.incorporate(self.int.build_filter(field.clone() + ".int"));
        filter.incorporate(self.string.build_filter(field.clone() + ".string"));
        filter
    }
}

#[derive(Clone, Debug)]
struct DataFilter2 {
    data: ObjectEntry<DataFilter1, Data1>,
    array: ArrayEntry<i32, i32>,
}

impl BuildFilter for DataFilter2 {
    fn build_filter(self) -> Filter {
        let mut filter = Filter::empty();
        filter.incorporate(self.data.build_filter("data".to_string()));
        filter.incorporate(self.array.build_filter("array".to_string()));
        filter
    }
}

#[test]
fn test_inner_object_entry_filter() {
    let data = Data1 { int: 2, string: "abc".to_string() };
    let filter =
        DataFilter1 { int: I32Entry::Value(3), string: StringEntry::Value("def".to_string()) };
    let expected1 = doc! { KEY: { "int": 2, "string": "abc" } };
    let expected2 = doc! {
        KEY.to_string() + ".int": 3,
        KEY.to_string() + ".string": "def",
    };

    let entry = ObjectEntry::Value::<DataFilter1, Data1>(data.clone());
    assert_eq!(entry.build_filter(KEY.to_string()).into_doc(), expected1);

    let entry = ObjectEntry::Dot::<DataFilter1, Data1>(filter.clone());
    assert_eq!(entry.build_filter(KEY.to_string()).into_doc(), expected2);

    let entry1 = ObjectEntry::Value::<DataFilter1, Data1>(data.clone());
    let entry2 = ObjectEntry::Dot::<DataFilter1, Data1>(filter.clone());
    let entry = ObjectEntry::Logical(Box::new(Logical::Or(vec![entry1, entry2])));
    let expected = doc! { "$or": [expected1, expected2] };
    assert_eq!(entry.build_filter(KEY.to_string()).into_doc(), expected);

    let entry = ObjectEntry::Element::<DataFilter1, Data1>(Element::Exists(true));
    let expected = doc! { KEY: { "$exists": true } };
    assert_eq!(entry.build_filter(KEY.to_string()).into_doc(), expected);
}

#[test]
fn test_outer_object_entry_filter() {
    let filter1 = DataFilter2 {
        data: ObjectEntry::Value(Data1 { int: 2, string: "abc".to_string() }),
        array: ArrayEntry::Array(Array::All(vec![3, 4])),
    };
    let filter2 = DataFilter2 {
        data: ObjectEntry::Dot(DataFilter1 { int: I32Entry::Value(3), string: StringEntry::Empty }),
        array: ArrayEntry::Array(Array::All(vec![5, 6])),
    };
    let expected1 = doc! {
        "data": { "int": 2, "string": "abc" },
        "array": { "$all": [3, 4] },
    };
    let expected2 = doc! {
        "data.int": 3,
        "array": { "$all": [5, 6] },
    };

    assert_eq!(filter1.build_filter().into_doc(), expected1);
    assert_eq!(filter2.build_filter().into_doc(), expected2);
}
