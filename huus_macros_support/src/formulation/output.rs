// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Structures for code generation.

use std::collections::VecDeque;

pub use crate::definition::output::{BuiltInType, Container, Variant};

/// Represents a part of an attribute. Parts are separated by dots (".").
#[derive(Clone, Debug)]
pub enum Part {
    /// Corresponds to an object field name.
    Key(String),

    /// Corresponds to a literal index.
    Index(String),

    /// Corresponds to an index passed in code mode (inside parenthesis "()").
    Code(String),

    /// Corresponds to a dollar ("$") operator.
    Dollar,
}

impl Part {
    /// Constructs a new `Part` basing on the passed string.
    pub fn from_str(string: &str) -> Self {
        if string == "$" {
            Self::Dollar
        } else if let Ok(_) = string.parse::<usize>() {
            Self::Index(string.to_string())
        } else {
            Self::Key(string.to_string())
        }
    }

    /// Constructs a new code `Part` containing the given code.
    pub fn from_code(string: String) -> Self {
        Self::Code(string)
    }

    /// Casts the part to a string.
    pub fn to_str(&self) -> &str {
        match self {
            Self::Key(name) => name.as_str(),
            Self::Index(index) => index.as_str(),
            Self::Code(string) => string.as_str(),
            Self::Dollar => "$",
        }
    }

    /// Returns `true` if the part is a key.
    pub fn is_key(&self) -> bool {
        match self {
            Self::Key(_) => true,
            _ => false,
        }
    }

    /// Returns `true` if the part corresponds to an operator (starts with "$").
    pub fn is_operator(&self) -> bool {
        match self {
            Self::Key(key) => key.starts_with("$"),
            _ => false,
        }
    }
}

/// Represents an attribute.
pub struct Attribute {
    /// A list of attribute parts. Parts in an attribute are separated by dots (".").
    pub parts: VecDeque<Part>,
}

/// Represents type of code data. There is not check for validity of data passed in the code mode.
/// The only check is done by casting the result to the expected type.
pub struct CodeType {
    /// The base type.
    pub variant: Variant,

    /// The container.
    pub container: Container,
}

impl CodeType {
    /// Produces the type to cast the result of code passed in the code mode.
    pub fn to_data(&self) -> String {
        let variant = self.variant.to_data();
        match &self.container {
            Container::Array => format!("Vec<{}>", variant),
            Container::HashMap(key_variant) => {
                let key = key_variant.to_data();
                format!("std::collections::HashMap<{}, {}>", key, variant)
            }
            Container::BTreeMap(key_variant) => {
                let key = key_variant.to_data();
                format!("std::collections::BTreeMap<{}, {}>", key, variant)
            }
            Container::Plain => variant,
        }
    }
}

/// Represent an object field value.
pub enum Value {
    /// Corresponds to a floating point.
    F64(f64),

    /// Corresponds to a string.
    String(String),

    /// Corresponds to an object ID.
    ObjectId(bson::oid::ObjectId),

    /// Corresponds to a boolean value.
    Bool(bool),

    /// Corresponds to a date.
    Date(chrono::DateTime<chrono::Utc>),

    /// Corresponds to a 32-bit integer.
    I32(i32),

    /// Corresponds to a 64-bit integer.
    I64(i64),

    /// Corresponds to an object.
    Object(Object),

    /// Corresponds to the code mode. Code mode it indicated by parentesis "()". There can be any
    /// code provided inside the parentesis.
    Code {
        /// The code.
        code: String,

        /// Expected type.
        cast: CodeType,
    },
}

impl Value {
    /// Constructs a new code value.
    pub fn new_builtin_code(builtin: BuiltInType, container: Container, code: String) -> Self {
        let cast = CodeType { variant: Variant::Field(builtin.clone()), container: container };
        Value::Code { code, cast }
    }
}

/// Represents an object field.
pub struct Field {
    /// The field attribute.
    pub attr: Attribute,

    /// The field value.
    pub value: Value,
}

impl Field {
    /// Constructs a new field.
    pub fn new(attr: Attribute, value: Value) -> Self {
        Self { attr, value }
    }
}

/// Represents an object.
pub struct Object {
    /// List of the object's fields.
    pub fields: Vec<Field>,
}

impl Object {
    /// Constructs a new `Object`.
    pub fn new() -> Self {
        Self { fields: Vec::new() }
    }
}
