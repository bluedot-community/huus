// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Structures for instructions parsing.

use std::collections::VecDeque;

pub use crate::formulation::output::{Attribute, Part};

/// Helps in parsing and reporting errors related to collection names.
#[derive(Clone)]
pub struct SpannedCollection {
    /// Name of the collection.
    pub name: String,

    /// Span of the `name`.
    pub span: proc_macro::Span,
}

impl SpannedCollection {
    /// Constructs a new `SpannedCollection`.
    pub fn new() -> Self {
        Self { name: String::new(), span: proc_macro::Span::call_site() }
    }
}

/// Helps in parsing and reporting errors related to attribute parts.
#[derive(Clone, Debug)]
pub struct SpannedPart {
    /// Information about the attribute part.
    pub part: Part,

    /// Span of the `part`.
    pub span: proc_macro::Span,
}

impl SpannedPart {
    /// Constructs a new `SpannedPart` from a string and a span.
    pub fn from_str(string: &str, span: proc_macro::Span) -> Self {
        Self { part: Part::from_str(string), span: span }
    }

    /// Constructs a new code `SpannedPart` from a string and a span.
    pub fn from_code(string: String, span: proc_macro::Span) -> Self {
        Self { part: Part::from_code(string), span: span }
    }

    /// Returns a string corresponding to the given attribute part.
    pub fn to_str(&self) -> &str {
        self.part.to_str()
    }

    /// Returns `true` if the part represents a key (as opposed to an index).
    pub fn is_key(&self) -> bool {
        self.part.is_key()
    }

    /// Returns `true` if the part represents an operator.
    pub fn is_operator(&self) -> bool {
        self.part.is_operator()
    }
}

/// Helps in parsing and reporting errors related to attributes.
#[derive(Clone, Debug)]
pub struct SpannedAttribute {
    /// List of attribute parts.
    pub parts: VecDeque<SpannedPart>,

    /// Span of the attribute.
    pub span: proc_macro::Span,
}

impl SpannedAttribute {
    /// Constructs a new empty `SpannedAttribute`.
    pub fn new() -> Self {
        Self { parts: VecDeque::new(), span: proc_macro::Span::call_site() }
    }

    /// Constructs a new `SpannedAttribute` from a string and a span.
    pub fn from_str(string: &str, span: proc_macro::Span) -> Self {
        let parts =
            string.split(".").map(|s| SpannedPart::from_str(s.trim(), span.clone())).collect();
        Self { parts, span }
    }

    /// Casts to `Attribute`.
    pub fn into_attribute(mut self) -> Attribute {
        Attribute { parts: self.parts.drain(..).map(|p| p.part).collect() }
    }

    /// Casts to a string.
    pub fn to_composed(&self) -> String {
        self.parts.iter().map(|p| p.to_str()).collect::<Vec<&str>>().join(".")
    }

    /// Returns the number of parts.
    pub fn len(&self) -> usize {
        self.parts.len()
    }

    /// Adds a new part.
    pub fn push(&mut self, part: SpannedPart) {
        if self.parts.len() == 0 {
            self.span = part.span.clone();
        } else {
            self.span = self.span.join(part.span.clone()).expect("Join spans");
        }
        self.parts.push_back(part)
    }

    /// Pop the next part.
    pub fn pop(&mut self) -> Option<SpannedPart> {
        self.parts.pop_front()
    }

    /// Returns a reference to the next part.
    pub fn next(&self) -> Option<&SpannedPart> {
        self.parts.front()
    }

    /// Returns `true` if the attribute corresponds to an operator.
    pub fn is_operator(&self) -> bool {
        if let Some(first) = self.next() {
            first.is_operator() && (self.len() == 1)
        } else {
            false
        }
    }
}

/// Helps in parsing and reporting errors related to values.
#[derive(Clone, Debug)]
pub enum ValueTemplate {
    /// Corresponds to any quoted value - strings, dates.
    Quoted(String),

    /// Corresponds to any unquoted value - numbers, booleans.
    Unquoted(String),

    /// Corresponds to objects (bound by curly braces "{}")
    Object(ObjectTemplate),

    /// Corresponds to code mode (bound by parenthesis "()")
    Code(String),
}

impl ValueTemplate {
    /// Return `true` if the value corresponds to an empty string.
    pub fn is_empty_string(&self) -> bool {
        match self {
            Self::Quoted(string) => string.is_empty(),
            _ => false,
        }
    }
}

/// Helps in parsing and reporting errors related to values.
#[derive(Clone, Debug)]
pub struct SpannedValue {
    /// Value template.
    pub value: ValueTemplate,

    /// Span of the `value`.
    pub span: proc_macro::Span,
}

impl SpannedValue {
    /// Constructs a new `SpannedValue`.
    pub fn new(value: ValueTemplate, span: proc_macro::Span) -> Self {
        Self { value, span }
    }
}

/// Helps in parsing and reporting errors related to fields.
#[derive(Clone, Debug)]
pub struct FieldTemplate {
    /// Describes the field attribute.
    pub attr: SpannedAttribute,

    /// Describes the field value.
    pub value: SpannedValue,
}

impl FieldTemplate {
    /// Constructs a new `FieldTemplate`.
    pub fn new(attr: SpannedAttribute, value: SpannedValue) -> Self {
        Self { attr, value }
    }
}

/// Helps in parsing and reporting errors related to objects.
#[derive(Clone, Debug)]
pub struct ObjectTemplate {
    /// List of the object's fields.
    pub fields: Vec<FieldTemplate>,

    /// Span of the object.
    pub span: proc_macro::Span,
}

impl ObjectTemplate {
    /// Constructs a new `ObjectTemplate`.
    pub fn new(span: proc_macro::Span) -> Self {
        Self { fields: Vec::new(), span }
    }
}
