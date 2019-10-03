// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Contains functionalities for value versions of `huus` structures.

use std::collections::{BTreeMap, HashMap};

use crate::conversions::{HuusIntoBson, HuusKey};
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

impl BuildValue for &str {
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

impl<K, B> BuildValue for BTreeMap<K, B>
where
    K: HuusKey,
    B: HuusIntoBson,
{
    fn build_value(self) -> Value {
        Value::new(self.huus_into_bson())
    }
}

impl<K, B> BuildValue for HashMap<K, B>
where
    K: HuusKey,
    B: HuusIntoBson,
{
    fn build_value(self) -> Value {
        Value::new(self.huus_into_bson())
    }
}

impl BuildValue for bson::Document {
    fn build_value(self) -> Value {
        Value::new(bson::Bson::Document(self))
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

fn vec_into_array<V>(elements: Vec<V>) -> bson::Bson
where
    V: BuildValue,
{
    let mut values = bson::Array::new();
    for element in elements {
        values.push(element.build_value().into_bson());
    }
    bson::Bson::Array(values)
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

// -------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub enum Entry<V>
where
    V: BuildValue,
{
    Value(V),
    In(Vec<V>),
    Empty,
}

impl<V> Entry<V>
where
    V: BuildValue,
{
    pub fn build_value(self) -> Option<Value> {
        match self {
            Entry::Value(value) => Some(value.build_value()),
            Entry::In(values) => {
                let mut result = bson::Document::new();
                result.insert("$in", vec_into_array(values));
                Some(Value::new(bson::Bson::Document(result)))
            }
            Entry::Empty => None,
        }
    }
}

impl<V> std::convert::From<V> for Entry<V>
where
    V: BuildValue,
{
    fn from(value: V) -> Entry<V> {
        Entry::Value(value)
    }
}

impl<V> std::convert::From<Option<V>> for Entry<V>
where
    V: BuildValue,
{
    fn from(value: Option<V>) -> Entry<V> {
        if let Some(value) = value {
            Entry::Value(value)
        } else {
            Entry::Empty
        }
    }
}

impl<V> std::convert::From<Vec<V>> for Entry<V>
where
    V: BuildValue,
{
    fn from(values: Vec<V>) -> Entry<V> {
        Entry::In(values)
    }
}

impl<V> Default for Entry<V>
where
    V: BuildValue,
{
    fn default() -> Self {
        Entry::Empty
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub enum ArrayEntry<V>
where
    V: BuildValue,
{
    Values(Vec<V>),
    Empty,
}

impl<V> ArrayEntry<V>
where
    V: BuildValue,
{
    pub fn build_value(self) -> Option<Value> {
        match self {
            ArrayEntry::Values(values) => Some(Value::new(vec_into_array(values))),
            ArrayEntry::Empty => None,
        }
    }
}

impl<V> std::convert::From<Vec<V>> for ArrayEntry<V>
where
    V: BuildValue,
{
    fn from(values: Vec<V>) -> ArrayEntry<V> {
        ArrayEntry::Values(values)
    }
}

impl<V> Default for ArrayEntry<V>
where
    V: BuildValue,
{
    fn default() -> Self {
        ArrayEntry::Empty
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Each<V>
where
    V: BuildValue,
{
    pub each: Vec<V>,
    pub position: Option<usize>,
    pub slice: Option<usize>,
}

impl<V> Each<V>
where
    V: BuildValue,
{
    pub fn new(each: Vec<V>) -> Self {
        Self { each: each, position: None, slice: None }
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub enum PushValue<V>
where
    V: BuildValue,
{
    Value(V),
    Each(Each<V>),
}

impl<V> BuildValue for PushValue<V>
where
    V: BuildValue,
{
    fn build_value(self) -> Value {
        match self {
            PushValue::Value(value) => value.build_value(),
            PushValue::Each(each) => {
                let mut result = bson::Document::new();
                result.insert("$each", vec_into_array(each.each));
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

impl<V> std::convert::From<V> for PushValue<V>
where
    V: BuildValue,
{
    fn from(values: V) -> PushValue<V> {
        PushValue::Value(values)
    }
}

impl<V> std::convert::From<Vec<V>> for PushValue<V>
where
    V: BuildValue,
{
    fn from(values: Vec<V>) -> PushValue<V> {
        PushValue::Each(Each::new(values))
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub enum PullValue<V>
where
    V: BuildValue,
{
    Value(V),
    In(Vec<V>),
}

impl<V> BuildValue for PullValue<V>
where
    V: BuildValue,
{
    fn build_value(self) -> Value {
        match self {
            PullValue::Value(value) => value.build_value(),
            PullValue::In(values) => {
                let mut result = bson::Document::new();
                result.insert("$in", vec_into_array(values));
                Value::new(bson::Bson::Document(result))
            }
        }
    }
}

impl<V> std::convert::From<V> for PullValue<V>
where
    V: BuildValue,
{
    fn from(values: V) -> PullValue<V> {
        PullValue::Value(values)
    }
}

impl<V> std::convert::From<Vec<V>> for PullValue<V>
where
    V: BuildValue,
{
    fn from(values: Vec<V>) -> PullValue<V> {
        PullValue::In(values)
    }
}
