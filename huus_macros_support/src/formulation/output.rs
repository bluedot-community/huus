// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Structures for code generation.

use std::collections::VecDeque;

pub use crate::definition::output::{Container, Variant, BuiltInType};

#[derive(Clone, Debug)]
pub enum Part {
    Key(String),
    Index(String),
    Code(String),
    Dollar,
}

impl Part {
    pub fn from_str(string: &str) -> Self {
        if string == "$" {
            Self::Dollar
        } else if let Ok(_) = string.parse::<usize>() {
            Self::Index(string.to_string())
        } else {
            Self::Key(string.to_string())
        }
    }

    pub fn from_code(string: String) -> Self {
        Self::Code(string)
    }

    pub fn to_str(&self) -> &str {
        match self {
            Self::Key(name) => name.as_str(),
            Self::Index(index) => index.as_str(),
            Self::Code(string) => string.as_str(),
            Self::Dollar => "$",
        }
    }

    pub fn is_key(&self) -> bool {
        match self {
            Self::Key(_) => true,
            _ => false,
        }
    }

    pub fn is_update_key(&self) -> bool {
        match self {
            Self::Key(key) => key.starts_with("$"),
            _ => false,
        }
    }
}

pub struct Attribute {
    pub parts: VecDeque<Part>,
}

pub struct Cast {
    pub variant: Variant,
    pub container: Container,
}

pub enum Value {
    F64(f64),
    String(String),
    ObjectId(bson::oid::ObjectId),
    Bool(bool),
    Date(chrono::DateTime<chrono::Utc>),
    I32(i32),
    I64(i64),
    Object(Object),
    Code { code: String, cast: Cast },
}

impl Value {
    pub fn new_builtin_code(builtin: BuiltInType, container: Container, code: String) -> Self {
        let cast = Cast {
            variant: Variant::Field(builtin.clone()),
            container: container,
        };
        Value::Code { code, cast }
    }
}

pub struct Field {
    pub attr: Attribute,
    pub value: Value,
}

impl Field {
    pub fn new(attr: Attribute, value: Value) -> Self {
        Self { attr, value }
    }
}

pub struct Object {
    pub fields: Vec<Field>,
}

impl Object {
    pub fn new() -> Self {
        Self { fields: Vec::new() }
    }
}
