// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Provides traits and implementations for conversions from Rust structures to BSON and vice versa.

use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;

use crate::errors::ConversionError;
use crate::types;

// -------------------------------------------------------------------------------------------------

pub trait FromDoc: Sized {
    fn from_doc(document: bson::Document) -> Result<Self, ConversionError>;
}

pub trait IntoDoc: Sized {
    fn into_doc(self) -> bson::Document;
}

// -------------------------------------------------------------------------------------------------

pub trait HuusFromBson: Sized {
    fn huus_from_bson(bson: bson::Bson) -> Result<Self, ConversionError>;
}

pub trait HuusFromBsonArray: Sized {
    fn huus_from_bson_array(array: bson::Array) -> Result<Self, ConversionError>;
}

pub trait HuusIntoStruct<T> {
    fn huus_into_struct(self) -> Result<T, ConversionError>;
}

pub trait HuusIntoBson {
    fn huus_into_bson(self) -> bson::Bson;
}

// -------------------------------------------------------------------------------------------------

pub trait HuusKey: Clone + PartialEq + Eq + PartialOrd + Ord + Hash {
    fn from_str(string: &str) -> Result<Self, ConversionError>;
    fn to_str(&self) -> &str;
}

// -------------------------------------------------------------------------------------------------

impl<K, T> FromDoc for BTreeMap<K, T>
where
    K: HuusKey,
    T: HuusFromBson,
{
    fn from_doc(document: bson::Document) -> Result<Self, ConversionError> {
        let mut result = BTreeMap::new();
        for (key, value) in document {
            match K::from_str(&key) {
                Ok(ok) => result.insert(ok, T::huus_from_bson(value)?),
                Err(err) => return Err(err),
            };
        }
        Ok(result)
    }
}

impl<K, T> FromDoc for HashMap<K, T>
where
    K: HuusKey,
    T: HuusFromBson,
{
    fn from_doc(document: bson::Document) -> Result<Self, ConversionError> {
        let mut result = HashMap::new();
        for (key, value) in document {
            match K::from_str(&key) {
                Ok(ok) => result.insert(ok, T::huus_from_bson(value)?),
                Err(err) => return Err(err),
            };
        }
        Ok(result)
    }
}

// -------------------------------------------------------------------------------------------------

impl IntoDoc for bson::Document {
    fn into_doc(self) -> bson::Document {
        self
    }
}

// -------------------------------------------------------------------------------------------------

impl HuusFromBson for String {
    fn huus_from_bson(bson: bson::Bson) -> Result<Self, ConversionError> {
        match bson {
            bson::Bson::String(value) => Ok(value),
            _ => Err(ConversionError::wrong_type_for_unknown_key()),
        }
    }
}

impl HuusFromBson for i32 {
    fn huus_from_bson(bson: bson::Bson) -> Result<Self, ConversionError> {
        match bson {
            bson::Bson::I32(value) => Ok(value),
            _ => Err(ConversionError::wrong_type_for_unknown_key()),
        }
    }
}

impl HuusFromBson for i64 {
    fn huus_from_bson(bson: bson::Bson) -> Result<Self, ConversionError> {
        match bson {
            bson::Bson::I64(value) => Ok(value),
            _ => Err(ConversionError::wrong_type_for_unknown_key()),
        }
    }
}

impl<T> HuusFromBson for T
where
    T: FromDoc,
{
    fn huus_from_bson(bson: bson::Bson) -> Result<Self, ConversionError> {
        match bson {
            bson::Bson::Document(doc) => Ok(T::from_doc(doc)?),
            _ => Err(ConversionError::wrong_type_for_unknown_key()),
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl<T> HuusFromBsonArray for Vec<T>
where
    T: HuusFromBson,
{
    fn huus_from_bson_array(array: bson::Array) -> Result<Self, ConversionError> {
        let mut result = Vec::with_capacity(array.len());
        for element in array {
            result.push(element.huus_into_struct()?);
        }
        Ok(result)
    }
}

// -------------------------------------------------------------------------------------------------

impl<T> HuusIntoStruct<T> for bson::Bson
where
    T: HuusFromBson,
{
    fn huus_into_struct(self) -> Result<T, ConversionError> {
        T::huus_from_bson(self)
    }
}

impl<T> HuusIntoStruct<T> for bson::Document
where
    T: FromDoc,
{
    fn huus_into_struct(self) -> Result<T, ConversionError> {
        T::from_doc(self)
    }
}

impl<T> HuusIntoStruct<T> for bson::Array
where
    T: HuusFromBsonArray,
{
    fn huus_into_struct(self) -> Result<T, ConversionError> {
        T::huus_from_bson_array(self)
    }
}

// -------------------------------------------------------------------------------------------------

impl HuusIntoBson for f32 {
    fn huus_into_bson(self) -> bson::Bson {
        bson::Bson::FloatingPoint(self as f64)
    }
}

impl HuusIntoBson for f64 {
    fn huus_into_bson(self) -> bson::Bson {
        bson::Bson::FloatingPoint(self)
    }
}

impl HuusIntoBson for String {
    fn huus_into_bson(self) -> bson::Bson {
        bson::Bson::String(self)
    }
}

impl HuusIntoBson for &str {
    fn huus_into_bson(self) -> bson::Bson {
        bson::Bson::String(self.to_string())
    }
}

impl HuusIntoBson for types::ObjectId {
    fn huus_into_bson(self) -> bson::Bson {
        bson::Bson::ObjectId(self)
    }
}

impl HuusIntoBson for bool {
    fn huus_into_bson(self) -> bson::Bson {
        bson::Bson::Boolean(self)
    }
}

impl HuusIntoBson for types::Date {
    fn huus_into_bson(self) -> bson::Bson {
        bson::Bson::UtcDatetime(self)
    }
}

impl HuusIntoBson for i32 {
    fn huus_into_bson(self) -> bson::Bson {
        bson::Bson::I32(self)
    }
}

impl HuusIntoBson for types::TimeStamp {
    fn huus_into_bson(self) -> bson::Bson {
        bson::Bson::TimeStamp(self.0)
    }
}

impl HuusIntoBson for i64 {
    fn huus_into_bson(self) -> bson::Bson {
        bson::Bson::I64(self)
    }
}

impl<T> HuusIntoBson for T
where
    T: IntoDoc,
{
    fn huus_into_bson(self) -> bson::Bson {
        bson::Bson::Document(self.into_doc())
    }
}

impl<T> HuusIntoBson for Vec<T>
where
    T: HuusIntoBson,
{
    fn huus_into_bson(self) -> bson::Bson {
        let mut result = bson::Array::with_capacity(self.len());
        for element in self {
            result.push(element.huus_into_bson());
        }
        bson::Bson::Array(result)
    }
}

impl<K, T> HuusIntoBson for BTreeMap<K, T>
where
    K: HuusKey,
    T: HuusIntoBson,
{
    fn huus_into_bson(self) -> bson::Bson {
        let mut result = bson::Document::new();
        for (key, value) in self {
            result.insert(key.to_str().to_string(), value.huus_into_bson());
        }
        bson::Bson::Document(result)
    }
}

impl<K, T> HuusIntoBson for HashMap<K, T>
where
    K: HuusKey,
    T: HuusIntoBson,
{
    fn huus_into_bson(self) -> bson::Bson {
        let mut result = bson::Document::new();
        for (key, value) in self {
            result.insert(key.to_str().to_string(), value.huus_into_bson());
        }
        bson::Bson::Document(result)
    }
}

// -------------------------------------------------------------------------------------------------

impl HuusKey for String {
    fn from_str(string: &str) -> Result<Self, ConversionError> {
        Ok(String::from(string))
    }

    fn to_str(&self) -> &str {
        self.as_ref()
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::HuusIntoStruct;

    #[test]
    fn test_array_into_vec() {
        let mut array = bson::Array::new();
        array.push(bson::Bson::I32(1));
        array.push(bson::Bson::I32(4));
        array.push(bson::Bson::I32(7));
        array.push(bson::Bson::I32(8));
        let expected: Vec<i32> = vec![1, 4, 7, 8];
        let result: Vec<i32> = array.huus_into_struct().unwrap();
        assert_eq!(result, expected);
    }
}
