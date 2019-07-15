// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Tests of entry objects from `filters` module.

use bson::{bson, doc};

use huus::filters::*;
use huus::types;

const KEY: &'static str = "xxx";

#[test]
fn test_comparison_filter() {
    use huus::filters::Comparison::{Eq, Gt, Gte, In, Lt, Lte, Ne, Nin};

    let entry = Eq(3.14);
    let expected = doc! { KEY: { "$eq": 3.14 } };
    assert_eq!(entry.build_filter(KEY.to_string()).into_doc(), expected);

    let entry = Eq("abc");
    let expected = doc! { KEY: { "$eq": "abc"  } };
    assert_eq!(entry.build_filter(KEY.to_string()).into_doc(), expected);

    let entry = Gt(3.14);
    let expected = doc! { KEY: { "$gt": 3.14 } };
    assert_eq!(entry.build_filter(KEY.to_string()).into_doc(), expected);

    let entry = Gt("abc");
    let expected = doc! { KEY: { "$gt": "abc"  } };
    assert_eq!(entry.build_filter(KEY.to_string()).into_doc(), expected);

    let entry = Gte(3.14);
    let expected = doc! { KEY: { "$gte": 3.14  } };
    assert_eq!(entry.build_filter(KEY.to_string()).into_doc(), expected);

    let entry = Gte("abc");
    let expected = doc! { KEY : { "$gte": "abc" } };
    assert_eq!(entry.build_filter(KEY.to_string()).into_doc(), expected);

    let entry = In(vec![3.14, 2.718]);
    let expected = doc! { KEY : { "$in": [3.14, 2.718] } };
    assert_eq!(entry.build_filter(KEY.to_string()).into_doc(), expected);

    let entry = In(vec!["abc", "def"]);
    let expected = doc! { KEY : { "$in": ["abc", "def"] } };
    assert_eq!(entry.build_filter(KEY.to_string()).into_doc(), expected);

    let entry = Lt(3.14);
    let expected = doc! { KEY : { "$lt": 3.14 } };
    assert_eq!(entry.build_filter(KEY.to_string()).into_doc(), expected);

    let entry = Lt("abc");
    let expected = doc! { KEY : { "$lt": "abc" } };
    assert_eq!(entry.build_filter(KEY.to_string()).into_doc(), expected);

    let entry = Lte(3.14);
    let expected = doc! { KEY : { "$lte": 3.14 } };
    assert_eq!(entry.build_filter(KEY.to_string()).into_doc(), expected);

    let entry = Lte("abc");
    let expected = doc! { KEY : { "$lte": "abc" } };
    assert_eq!(entry.build_filter(KEY.to_string()).into_doc(), expected);

    let entry = Ne(3.14);
    let expected = doc! { KEY : { "$ne": 3.14 } };
    assert_eq!(entry.build_filter(KEY.to_string()).into_doc(), expected);

    let entry = Ne("abc");
    let expected = doc! { KEY : { "$ne": "abc" } };
    assert_eq!(entry.build_filter(KEY.to_string()).into_doc(), expected);

    let entry = Nin(vec![3.14, 2.718]);
    let expected = doc! { KEY : { "$nin": [3.14, 2.718] } };
    assert_eq!(entry.build_filter(KEY.to_string()).into_doc(), expected);

    let entry = Nin(vec!["abc", "def"]);
    let expected = doc! { KEY : { "$nin": ["abc", "def"] } };
    assert_eq!(entry.build_filter(KEY.to_string()).into_doc(), expected);
}

#[test]
fn test_logical_filter() {
    use huus::filters::Comparison::Eq;
    use huus::filters::Logical::{And, Nor, Not, Or};

    let entry = And(vec![Eq(3.14), Eq(2.718)]);
    let expected = doc! { "$and": [{ KEY: { "$eq": 3.14 } }, { KEY: { "$eq": 2.718 } }] };
    assert_eq!(entry.build_filter(KEY.to_string()).into_doc(), expected);

    let entry = Not(Eq(3.14));
    let expected = doc! { "$not": { KEY: { "$eq": 3.14 } } };
    assert_eq!(entry.build_filter(KEY.to_string()).into_doc(), expected);

    let entry = Nor(vec![Eq(3.14), Eq(2.718)]);
    let expected = doc! { "$nor": [{ KEY: { "$eq": 3.14 } }, { KEY: { "$eq": 2.718 } }] };
    assert_eq!(entry.build_filter(KEY.to_string()).into_doc(), expected);

    let entry = Or(vec![Eq(3.14), Eq(2.718)]);
    let expected = doc! { "$or": [{ KEY: { "$eq": 3.14 } }, { KEY: { "$eq": 2.718 } }] };
    assert_eq!(entry.build_filter(KEY.to_string()).into_doc(), expected);
}

#[test]
fn test_element_filter() {
    use huus::filters::Element::{Exists, Type};

    let entry = Exists(true);
    let expected = doc! { KEY: { "$exists": true } };
    assert_eq!(entry.build_filter(KEY.to_string()).into_doc(), expected);

    let entry = Type(types::Type::String);
    let expected = doc! { KEY: { "$type": 2 } };
    assert_eq!(entry.build_filter(KEY.to_string()).into_doc(), expected);
}

#[test]
fn test_array_filter() {
    use huus::filters::Array::{All, ElemMatch, Size};

    let entry = All(vec![3.14, 2.718]);
    let expected = doc! { KEY: { "$all": [3.14, 2.718] } };
    assert_eq!(entry.build_filter(KEY.to_string()).into_doc(), expected);

    let entry = ElemMatch(vec![3.14, 2.718]);
    let expected = doc! { KEY: { "$elemMatch": [3.14, 2.718] } };
    assert_eq!(entry.build_filter(KEY.to_string()).into_doc(), expected);

    let entry = Size::<f32>(5);
    let expected = doc! { KEY: { "$size": 5 } };
    assert_eq!(entry.build_filter(KEY.to_string()).into_doc(), expected);
}

#[test]
fn test_double_entry_filter() {
    let entry = F64Entry::Value(3.14);
    let expected = doc! { KEY: bson::Bson::FloatingPoint(3.14) };
    assert_eq!(entry.build_filter(KEY.to_string()).into_doc(), expected);

    let entry = F64Entry::Comparison(Comparison::Eq(3.14));
    let expected = doc! { KEY: { "$eq": bson::Bson::FloatingPoint(3.14) } };
    assert_eq!(entry.build_filter(KEY.to_string()).into_doc(), expected);

    let entry = F64Entry::Element(Element::Exists(true));
    let expected = doc! { KEY: { "$exists": true } };
    assert_eq!(entry.build_filter(KEY.to_string()).into_doc(), expected);
}

#[test]
fn test_string_entry_filter() {
    let entry = StringEntry::Value("abc".to_string());
    let expected = doc! { KEY: bson::Bson::String("abc".to_string()) };
    assert_eq!(entry.build_filter(KEY.to_string()).into_doc(), expected);

    let entry = StringEntry::Comparison(Comparison::Eq("abc".to_string()));
    let expected = doc! { KEY: { "$eq": "abc" } };
    assert_eq!(entry.build_filter(KEY.to_string()).into_doc(), expected);

    let entry = StringEntry::Element(Element::Exists(true));
    let expected = doc! { KEY: { "$exists": true } };
    assert_eq!(entry.build_filter(KEY.to_string()).into_doc(), expected);
}

#[test]
fn test_array_entry_filter() {
    let entry = ArrayEntry::Value::<f64, f64>(3.14);
    let expected = doc! { KEY: 3.14 };
    assert_eq!(entry.build_filter(KEY.to_string()).into_doc(), expected);

    let entry = ArrayEntry::Dot::<f64, f64>(3.14);
    let expected = doc! { KEY: 3.14 };
    assert_eq!(entry.build_filter(KEY.to_string()).into_doc(), expected);

    let entry = ArrayEntry::Array::<f64, f64>(Array::All(vec![3.14, 2.718]));
    let expected = doc! { KEY: { "$all": [3.14, 2.718] } };
    assert_eq!(entry.build_filter(KEY.to_string()).into_doc(), expected);

    let entry = ArrayEntry::Element::<f32, f32>(Element::Exists(true));
    let expected = doc! { KEY: { "$exists": true } };
    assert_eq!(entry.build_filter(KEY.to_string()).into_doc(), expected);
}

#[test]
fn test_objectid_entry_filter() {
    let oid = bson::oid::ObjectId::with_string("11223344556677889900aabb").unwrap();

    let entry = ObjectIdEntry::Value(oid.clone());
    let expected = doc! { KEY: bson::Bson::ObjectId(oid.clone()) };
    assert_eq!(entry.build_filter(KEY.to_string()).into_doc(), expected);

    let entry = ObjectIdEntry::Element(Element::Exists(true));
    let expected = doc! { KEY: { "$exists": true } };
    assert_eq!(entry.build_filter(KEY.to_string()).into_doc(), expected);
}

#[test]
fn test_boolean_entry_filter() {
    let entry = BooleanEntry::Value(true);
    let expected = doc! { KEY: bson::Bson::Boolean(true) };
    assert_eq!(entry.build_filter(KEY.to_string()).into_doc(), expected);

    let entry = BooleanEntry::Element(Element::Exists(true));
    let expected = doc! { KEY: { "$exists": true } };
    assert_eq!(entry.build_filter(KEY.to_string()).into_doc(), expected);
}

#[test]
fn test_date_entry_filter() {
    let datetime = chrono::NaiveDateTime::from_timestamp(10, 0);
    let datetime = chrono::DateTime::<chrono::Utc>::from_utc(datetime, chrono::Utc);

    let entry = DateEntry::Value(datetime);
    let expected = doc! { KEY: bson::Bson::UtcDatetime(datetime) };
    assert_eq!(entry.build_filter(KEY.to_string()).into_doc(), expected);

    let entry = DateEntry::Comparison(Comparison::Eq(datetime));
    let expected = doc! { KEY: { "$eq": datetime } };
    assert_eq!(entry.build_filter(KEY.to_string()).into_doc(), expected);

    let entry = DateEntry::Element(Element::Exists(true));
    let expected = doc! { KEY: { "$exists": true } };
    assert_eq!(entry.build_filter(KEY.to_string()).into_doc(), expected);
}

#[test]
fn test_null_entry_filter() {
    let entry = NullEntry::Element(Element::Exists(true));
    let expected = doc! { KEY: { "$exists": true } };
    assert_eq!(entry.build_filter(KEY.to_string()).into_doc(), expected);
}

#[test]
fn test_i32_entry_filter() {
    let entry = I32Entry::Value(3);
    let expected = doc! { KEY: bson::Bson::I32(3) };
    assert_eq!(entry.build_filter(KEY.to_string()).into_doc(), expected);

    let entry = I32Entry::Comparison(Comparison::Eq(3));
    let expected = doc! { KEY: { "$eq": bson::Bson::I32(3) } };
    assert_eq!(entry.build_filter(KEY.to_string()).into_doc(), expected);

    let entry = I32Entry::Element(Element::Exists(true));
    let expected = doc! { KEY: { "$exists": true } };
    assert_eq!(entry.build_filter(KEY.to_string()).into_doc(), expected);
}

#[test]
fn test_timestamp_entry_filter() {
    let entry = TimeStampEntry::Value(types::TimeStamp(3));
    let expected = doc! { KEY: bson::Bson::TimeStamp(3) };
    assert_eq!(entry.build_filter(KEY.to_string()).into_doc(), expected);

    let entry = TimeStampEntry::Comparison(Comparison::Eq(types::TimeStamp(3)));
    let expected = doc! { KEY: { "$eq": bson::Bson::TimeStamp(3) } };
    assert_eq!(entry.build_filter(KEY.to_string()).into_doc(), expected);

    let entry = TimeStampEntry::Element(Element::Exists(true));
    let expected = doc! { KEY: { "$exists": true } };
    assert_eq!(entry.build_filter(KEY.to_string()).into_doc(), expected);
}

#[test]
fn test_i64_entry_filter() {
    let entry = I64Entry::Value(3);
    let expected = doc! { KEY: bson::Bson::I64(3) };
    assert_eq!(entry.build_filter(KEY.to_string()).into_doc(), expected);

    let entry = I64Entry::Comparison(Comparison::Eq(3));
    let expected = doc! { KEY: { "$eq": bson::Bson::I64(3) } };
    assert_eq!(entry.build_filter(KEY.to_string()).into_doc(), expected);

    let entry = I64Entry::Element(Element::Exists(true));
    let expected = doc! { KEY: { "$exists": true } };
    assert_eq!(entry.build_filter(KEY.to_string()).into_doc(), expected);
}

#[test]
fn test_bson_entry_filter() {
    let entry = BsonEntry::Value(doc! { "a": 1, "b": 2 });
    let expected = doc! { KEY: { "a": 1, "b": 2 } };
    assert_eq!(entry.build_filter(KEY.to_string()).into_doc(), expected);

    let entry = BsonEntry::Element(Element::Exists(true));
    let expected = doc! { KEY: { "$exists": true } };
    assert_eq!(entry.build_filter(KEY.to_string()).into_doc(), expected);
}
