// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Tests of entry objects from `updates` module.

use bson::{bson, doc};
use huus::types;
use huus::updates::*;
use huus::values::PullValue;

const KEY: &'static str = "xxx";

#[test]
fn test_numerical_update() {
    use huus::updates::Numerical::{Inc, Max, Min, Mul};

    let operation = Inc(3);
    let expected = doc! { "$inc": { KEY: 3 } };
    assert_eq!(operation.build_update(KEY.to_string()).into_doc(), expected);

    let operation = Inc(3.14);
    let expected = doc! { "$inc": { KEY: 3.14 } };
    assert_eq!(operation.build_update(KEY.to_string()).into_doc(), expected);

    let operation = Min(3);
    let expected = doc! { "$min": { KEY: 3 } };
    assert_eq!(operation.build_update(KEY.to_string()).into_doc(), expected);

    let operation = Min(3.14);
    let expected = doc! { "$min": { KEY: 3.14 } };
    assert_eq!(operation.build_update(KEY.to_string()).into_doc(), expected);

    let operation = Max(3);
    let expected = doc! { "$max": { KEY: 3 } };
    assert_eq!(operation.build_update(KEY.to_string()).into_doc(), expected);

    let operation = Max(3.14);
    let expected = doc! { "$max": { KEY: 3.14 } };
    assert_eq!(operation.build_update(KEY.to_string()).into_doc(), expected);

    let operation = Mul(3);
    let expected = doc! { "$mul": { KEY: 3 } };
    assert_eq!(operation.build_update(KEY.to_string()).into_doc(), expected);

    let operation = Mul(3.14);
    let expected = doc! { "$mul": { KEY: 3.14 } };
    assert_eq!(operation.build_update(KEY.to_string()).into_doc(), expected);
}

#[test]
fn test_field_update() {
    use huus::updates::Field::{Rename, Set, SetOnInsert, Unset};

    let operation = Rename::<i32>("new_name".to_string());
    let expected = doc! { "$rename": { KEY: "new_name" } };
    assert_eq!(operation.build_update(KEY.to_string()).into_doc(), expected);

    let operation = Set(3);
    let expected = doc! { "$set": { KEY: 3 } };
    assert_eq!(operation.build_update(KEY.to_string()).into_doc(), expected);

    let operation = SetOnInsert(3);
    let expected = doc! { "$setOnInsert": { KEY: 3 } };
    assert_eq!(operation.build_update(KEY.to_string()).into_doc(), expected);

    let operation = Unset::<i32>;
    let expected = doc! { "$unset": { KEY: true } };
    assert_eq!(operation.build_update(KEY.to_string()).into_doc(), expected);
}

#[test]
fn test_array_update() {
    use huus::updates::Array::{AddToSet, Pop, Pull, PullAll, Push};
    use huus::values::{Each, PushValue};

    let operation = AddToSet(PushValue::Value(3.14));
    let expected = doc! { "$addToSet": { KEY: 3.14 } };
    assert_eq!(operation.build_update(KEY.to_string()).into_doc(), expected);

    let operation = AddToSet(PushValue::Each(Each::new(vec!["abc", "def"])));
    let expected = doc! { "$addToSet": { KEY: { "$each": ["abc", "def"] } } };
    assert_eq!(operation.build_update(KEY.to_string()).into_doc(), expected);

    let operation = Pop::<i32>(PopOption::First);
    let expected = doc! { "$pop": { KEY: -1 } };
    assert_eq!(operation.build_update(KEY.to_string()).into_doc(), expected);

    let operation = Pop::<i32>(PopOption::Last);
    let expected = doc! { "$pop": { KEY: 1 } };
    assert_eq!(operation.build_update(KEY.to_string()).into_doc(), expected);

    let operation = Pull(PullValue::Value(3.14));
    let expected = doc! { "$pull": { KEY: 3.14 } };
    assert_eq!(operation.build_update(KEY.to_string()).into_doc(), expected);

    let operation = Pull(PullValue::In(vec!["abc", "def"]));
    let expected = doc! { "$pull": { KEY: { "$in": ["abc", "def"] } } };
    assert_eq!(operation.build_update(KEY.to_string()).into_doc(), expected);

    let operation = Push(PushValue::Value(3.14));
    let expected = doc! { "$push": { KEY: 3.14 } };
    assert_eq!(operation.build_update(KEY.to_string()).into_doc(), expected);

    let operation = Push(PushValue::Each(Each::new(vec!["abc", "def"])));
    let expected = doc! { "$push": { KEY: { "$each": ["abc", "def"] } } };
    assert_eq!(operation.build_update(KEY.to_string()).into_doc(), expected);

    let operation = PullAll(3.14);
    let expected = doc! { "$pullAll": { KEY: 3.14 } };
    assert_eq!(operation.build_update(KEY.to_string()).into_doc(), expected);

    let operation = PullAll("abc");
    let expected = doc! { "$pullAll": { KEY: "abc" } };
    assert_eq!(operation.build_update(KEY.to_string()).into_doc(), expected);
}

#[test]
fn test_f64_entry_update() {
    let entry = F64Entry::Value(3.14);
    let expected = doc! { KEY: 3.14 };
    assert_eq!(entry.build_update(KEY.to_string()).into_doc(), expected);

    let entry = F64Entry::Numerical(Numerical::Max(3.14));
    let expected = doc! { "$max": { KEY: 3.14 } };
    assert_eq!(entry.build_update(KEY.to_string()).into_doc(), expected);

    let entry = F64Entry::Field(Field::Set(3.14));
    let expected = doc! { "$set": { KEY: 3.14 } };
    assert_eq!(entry.build_update(KEY.to_string()).into_doc(), expected);
}

#[test]
fn test_string_entry_update() {
    let entry = StringEntry::Value("abc".to_string());
    let expected = doc! { KEY: "abc" };
    assert_eq!(entry.build_update(KEY.to_string()).into_doc(), expected);

    let entry = StringEntry::Field(Field::Set("abc".to_string()));
    let expected = doc! { "$set": { KEY: "abc" } };
    assert_eq!(entry.build_update(KEY.to_string()).into_doc(), expected);
}

#[test]
fn test_array_entry_update() {
    let entry = ArrayEntry::Array::<I32Entry, i32>(Array::Pop(PopOption::First), Operator::None);
    let expected = doc! { "$pop": { KEY: -1 } };
    assert_eq!(entry.build_update(KEY.to_string()).into_doc(), expected);

    let entry = ArrayEntry::Array::<I32Entry, i32>(Array::Pop(PopOption::Last), Operator::First);
    let expected = doc! { "$pop": { KEY.to_string() + ".$": 1 } };
    assert_eq!(entry.build_update(KEY.to_string()).into_doc(), expected);

    let entry = ArrayEntry::Element::<F64Entry, f64>(Element::Set(3.14), Operator::None);
    let expected = doc! { "$set": { KEY: 3.14 } };
    assert_eq!(entry.build_update(KEY.to_string()).into_doc(), expected);

    let entry = ArrayEntry::Element::<F64Entry, f64>(Element::Set(3.14), Operator::First);
    let expected = doc! { "$set": { KEY.to_string() + ".$": 3.14 } };
    assert_eq!(entry.build_update(KEY.to_string()).into_doc(), expected);

    let entry = ArrayEntry::Indexed::<F64Entry, f64>(3, F64Entry::Value(3.14));
    let expected = doc! { KEY.to_string() + ".3": 3.14 };
    assert_eq!(entry.build_update(KEY.to_string()).into_doc(), expected);
}

#[test]
fn test_objectid_entry_update() {
    let oid = bson::oid::ObjectId::with_string("11223344556677889900aabb").unwrap();

    let entry = ObjectIdEntry::Value(oid.clone());
    let expected = doc! { KEY: oid.clone() };
    assert_eq!(entry.build_update(KEY.to_string()).into_doc(), expected);

    let entry = ObjectIdEntry::Field(Field::Set(oid.clone()));
    let expected = doc! { "$set": { KEY: oid.clone() } };
    assert_eq!(entry.build_update(KEY.to_string()).into_doc(), expected);
}

#[test]
fn test_boolean_entry_update() {
    let entry = BooleanEntry::Value(true);
    let expected = doc! { KEY: true };
    assert_eq!(entry.build_update(KEY.to_string()).into_doc(), expected);

    let entry = BooleanEntry::Field(Field::Set(true));
    let expected = doc! { "$set": { KEY: true } };
    assert_eq!(entry.build_update(KEY.to_string()).into_doc(), expected);
}

#[test]
fn test_date_entry_update() {
    let datetime = chrono::NaiveDateTime::from_timestamp(10, 0);
    let datetime = chrono::DateTime::<chrono::Utc>::from_utc(datetime, chrono::Utc);

    let entry = DateEntry::Value(datetime);
    let expected = doc! { KEY: datetime };
    assert_eq!(entry.build_update(KEY.to_string()).into_doc(), expected);

    let entry = DateEntry::CurrentDate;
    let expected = doc! { "$currentDate": { KEY: "date" } };
    assert_eq!(entry.build_update(KEY.to_string()).into_doc(), expected);

    let entry = DateEntry::Field(Field::Set(datetime));
    let expected = doc! { "$set": { KEY: datetime } };
    assert_eq!(entry.build_update(KEY.to_string()).into_doc(), expected);
}

#[test]
fn test_i32_entry_update() {
    let entry = I32Entry::Value(3);
    let expected = doc! { KEY: 3i32 };
    assert_eq!(entry.build_update(KEY.to_string()).into_doc(), expected);

    let entry = I32Entry::Numerical(Numerical::Max(3));
    let expected = doc! { "$max": { KEY: 3i32 } };
    assert_eq!(entry.build_update(KEY.to_string()).into_doc(), expected);

    let entry = I32Entry::Field(Field::Set(3));
    let expected = doc! { "$set": { KEY: 3i32 } };
    assert_eq!(entry.build_update(KEY.to_string()).into_doc(), expected);
}

#[test]
fn test_timestamp_entry_update() {
    let entry = TimeStampEntry::Value(types::TimeStamp(3));
    let expected = doc! { KEY: bson::Bson::TimeStamp(3) };
    assert_eq!(entry.build_update(KEY.to_string()).into_doc(), expected);

    let entry = TimeStampEntry::CurrentDate;
    let expected = doc! { "$currentDate": { KEY: "timestamp" } };
    assert_eq!(entry.build_update(KEY.to_string()).into_doc(), expected);

    let entry = TimeStampEntry::Field(Field::Set(types::TimeStamp(3)));
    let expected = doc! { "$set": { KEY: bson::Bson::TimeStamp(3) } };
    assert_eq!(entry.build_update(KEY.to_string()).into_doc(), expected);
}

#[test]
fn test_i64_entry_update() {
    let entry = I64Entry::Value(3);
    let expected = doc! { KEY: 3i64 };
    assert_eq!(entry.build_update(KEY.to_string()).into_doc(), expected);

    let entry = I64Entry::Numerical(Numerical::Max(3));
    let expected = doc! { "$max": { KEY: 3i64 } };
    assert_eq!(entry.build_update(KEY.to_string()).into_doc(), expected);

    let entry = I64Entry::Field(Field::Set(3));
    let expected = doc! { "$set": { KEY: 3i64 } };
    assert_eq!(entry.build_update(KEY.to_string()).into_doc(), expected);
}

#[test]
fn test_bson_entry_update() {
    let entry = BsonEntry::Value(doc! { "a": 1, "b": 2 });
    let expected = doc! { KEY: { "a": 1, "b": 2 } };
    assert_eq!(entry.build_update(KEY.to_string()).into_doc(), expected);

    let entry = BsonEntry::Field(Field::Set(doc! { "a": 1, "b": 2 }));
    let expected = doc! { "$set": { KEY: { "a": 1, "b": 2 } } };
    assert_eq!(entry.build_update(KEY.to_string()).into_doc(), expected);
}
