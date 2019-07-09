// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Types used in BSON.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Type {
    Double = 1,
    String = 2,
    Object = 3,
    Array = 4,
    BinaryData = 5,
    #[deprecated]
    Undefined = 6,
    ObjectId = 7,
    Boolean = 8,
    Date = 9,
    Null = 10,
    Regex = 11,
    #[deprecated]
    DBPointer = 12,
    JavaScript = 13,
    #[deprecated]
    Symbol = 14,
    JavaScriptWithScope = 15,
    I32 = 16,
    Timestamp = 17,
    I64 = 18,
    Decimal128 = 19,
}

pub type Double = f64;
pub type ObjectId = bson::oid::ObjectId;
pub type Date = chrono::DateTime<chrono::Utc>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TimeStamp(pub i64);
