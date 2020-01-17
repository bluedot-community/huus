// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Structures for instructions parsing.

use std::collections::VecDeque;

pub use crate::formulation::output::{Attribute, Part};

#[derive(Clone)]
pub struct SpannedCollection {
    pub name: String,
    pub span: proc_macro::Span,
}

impl SpannedCollection {
    pub fn new() -> Self {
        Self { name: String::new(), span: proc_macro::Span::call_site() }
    }
}

#[derive(Clone, Debug)]
pub struct SpannedPart {
    pub part: Part,
    pub span: proc_macro::Span,
}

impl SpannedPart {
    pub fn from_str(string: &str, span: proc_macro::Span) -> Self {
        Self { part: Part::from_str(string), span: span }
    }

    pub fn from_code(string: String, span: proc_macro::Span) -> Self {
        Self { part: Part::from_code(string), span: span }
    }

    pub fn to_str(&self) -> &str {
        self.part.to_str()
    }

    pub fn is_key(&self) -> bool {
        self.part.is_key()
    }

    pub fn is_update_key(&self) -> bool {
        self.part.is_update_key()
    }
}

#[derive(Clone, Debug)]
pub struct SpannedAttribute {
    pub parts: VecDeque<SpannedPart>,
    pub span: proc_macro::Span,
}

impl SpannedAttribute {
    pub fn new() -> Self {
        Self {
            parts: VecDeque::new(),
            span: proc_macro::Span::call_site(),
        }
    }

    pub fn from_str(string: &str, span: proc_macro::Span) -> Self {
        let parts =
            string.split(".").map(|s| SpannedPart::from_str(s.trim(), span.clone())).collect();
        Self { parts, span }
    }

    pub fn into_attribute(mut self) -> Attribute {
        Attribute { parts: self.parts.drain(..).map(|p| p.part).collect() }
    }

    pub fn to_composed(&self) -> String {
        self.parts.iter().map(|p| p.to_str()).collect::<Vec<&str>>().join(".")
    }

    pub fn len(&self) -> usize {
        self.parts.len()
    }

    pub fn push(&mut self, part: SpannedPart) {
        if self.parts.len() == 0 {
            self.span = part.span.clone();
        } else {
            self.span = self.span.join(part.span.clone()).expect("Join spans");
        }
        self.parts.push_back(part)
    }

    pub fn pop(&mut self) -> Option<SpannedPart> {
        self.parts.pop_front()
    }

    pub fn next(&self) -> Option<&SpannedPart> {
        self.parts.front()
    }

    pub fn is_updating(&self) -> bool {
        if let Some(first) = self.next() {
            first.is_update_key() && (self.len() == 1)
        } else {
            false
        }
    }
}

#[derive(Clone, Debug)]
pub enum ValueTemplate {
    Quoted(String),
    Unquoted(String),
    Object(ObjectTemplate),
    Code(String),
}

impl ValueTemplate {
    pub fn is_empty_string(&self) -> bool {
        match self {
            Self::Quoted(string) => string.is_empty(),
            _ => false
        }
    }
}

#[derive(Clone, Debug)]
pub struct SpannedValue {
    pub value: ValueTemplate,
    pub span: proc_macro::Span,
}

impl SpannedValue {
    pub fn new(value: ValueTemplate, span: proc_macro::Span) -> Self {
        Self { value, span }
    }
}

#[derive(Clone, Debug)]
pub struct FieldTemplate {
    pub attr: SpannedAttribute,
    pub value: SpannedValue,
}

impl FieldTemplate {
    pub fn new(attr: SpannedAttribute, value: SpannedValue) -> Self {
        Self { attr, value }
    }
}

#[derive(Clone, Debug)]
pub struct ObjectTemplate {
    pub fields: Vec<FieldTemplate>,
    pub span: proc_macro::Span,
}

impl ObjectTemplate {
    pub fn new(span: proc_macro::Span) -> Self {
        Self { fields: Vec::new(), span }
    }
}
