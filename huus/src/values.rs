// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Contains functionalities for value versions of `huus` structures.

use std::collections::{BTreeMap, HashMap};

use crate::conversions::{HuusEnum, HuusIntoBson};
use crate::types;

// -------------------------------------------------------------------------------------------------

pub trait BuildValue {
    fn build_value(self) -> Value;
}

// -------------------------------------------------------------------------------------------------

impl BuildValue for types::Double {
    fn build_value(self) -> Value {
        Value::new(bson::Bson::FloatingPoint(self))
    }
}

impl<'a> BuildValue for &'a str {
    fn build_value(self) -> Value {
        Value::new(bson::Bson::String(self.to_string()))
    }
}

impl BuildValue for String {
    fn build_value(self) -> Value {
        Value::new(bson::Bson::String(self))
    }
}

impl BuildValue for types::ObjectId {
    fn build_value(self) -> Value {
        Value::new(bson::Bson::ObjectId(self))
    }
}

impl BuildValue for bool {
    fn build_value(self) -> Value {
        Value::new(bson::Bson::Boolean(self))
    }
}

impl BuildValue for types::Date {
    fn build_value(self) -> Value {
        Value::new(bson::Bson::UtcDatetime(self))
    }
}

impl BuildValue for i32 {
    fn build_value(self) -> Value {
        Value::new(bson::Bson::I32(self))
    }
}

impl BuildValue for types::TimeStamp {
    fn build_value(self) -> Value {
        Value::new(self.huus_into_bson())
    }
}

impl BuildValue for i64 {
    fn build_value(self) -> Value {
        Value::new(bson::Bson::I64(self))
    }
}

impl<E, B> BuildValue for BTreeMap<E, B>
where
    E: HuusEnum,
    B: HuusIntoBson,
{
    fn build_value(self) -> Value {
        Value::new(self.huus_into_bson())
    }
}

impl<E, B> BuildValue for HashMap<E, B>
where
    E: HuusEnum,
    B: HuusIntoBson,
{
    fn build_value(self) -> Value {
        Value::new(self.huus_into_bson())
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Each<T>
where
    T: HuusIntoBson,
{
    pub each: Vec<T>,
    pub position: Option<usize>,
    pub slice: Option<usize>,
}

// -------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub enum Array<T>
where
    T: HuusIntoBson,
{
    Array(Vec<T>),
    Each(Each<T>),
}

impl<T> BuildValue for Array<T>
where
    T: HuusIntoBson,
{
    fn build_value(self) -> Value {
        match self {
            Array::Array(value) => {
                let mut array = Vec::new();
                for element in value {
                    array.push(element.huus_into_bson());
                }
                Value::new(bson::Bson::Array(array))
            }
            Array::Each(each) => {
                let mut array = bson::Array::new();
                for element in each.each {
                    array.push(element.huus_into_bson());
                }
                let mut result = bson::Document::new();
                result.insert("$each", array);
                if let Some(position) = each.position {
                    result.insert("$position", position as i64);
                }
                if let Some(slice) = each.slice {
                    result.insert("$slice", slice as i64);
                }
                Value::new(bson::Bson::Document(result))
            }
        }
    }
}

impl<T> std::convert::From<Vec<T>> for Array<T>
where
    T: HuusIntoBson,
{
    fn from(value: Vec<T>) -> Array<T> {
        Array::Array(value)
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct ObjectValue {
    doc: bson::Document,
}

impl ObjectValue {
    pub fn empty() -> Self {
        Self { doc: bson::Document::new() }
    }

    pub fn insert(&mut self, key: String, value: bson::Bson) {
        self.doc.insert_bson(key, value);
    }
}

impl BuildValue for ObjectValue {
    fn build_value(self) -> Value {
        Value::new(bson::Bson::Document(self.doc))
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Value {
    value: bson::Bson,
}

impl Value {
    pub fn empty() -> Self {
        Self { value: bson::Bson::Document(bson::Document::new()) }
    }

    pub fn new(value: bson::Bson) -> Self {
        Self { value }
    }

    pub fn into_bson(self) -> bson::Bson {
        self.value
    }
}
