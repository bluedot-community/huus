// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Structures for instructions parsing.

pub use crate::definition::output::{
    BuiltInType, DefinedType, Enum, EnumChoice, Union, UnionChoice,
};

#[derive(Clone)]
pub enum ContainerTemplate {
    Array,
    BTreeMap(String),
    HashMap(String),
    Plain,
}

#[derive(Clone)]
pub struct MemberTemplate {
    pub rust_name: Option<String>,
    pub rust_name_span: proc_macro::Span,
    pub db_name: Option<String>,
    pub db_name_span: proc_macro::Span,
    pub variant: Option<String>,
    pub variant_span: proc_macro::Span,
    pub is_optional: bool,
    pub is_indexed: bool,
    pub container: ContainerTemplate,
}

impl MemberTemplate {
    pub fn new() -> Self {
        Self {
            rust_name: None,
            rust_name_span: proc_macro::Span::call_site(),
            db_name: None,
            db_name_span: proc_macro::Span::call_site(),
            variant: None,
            variant_span: proc_macro::Span::call_site(),
            is_optional: false,
            is_indexed: false,
            container: ContainerTemplate::Plain,
        }
    }
}

#[derive(Clone)]
pub struct StructTemplate {
    pub struct_name: String,
    pub struct_name_span: proc_macro::Span,
    pub collection_name: Option<String>,
    pub collection_name_span: proc_macro::Span,
    pub members: Vec<MemberTemplate>,
}

#[derive(Clone)]
pub struct EnumTemplate {
    pub name: String,
    pub name_span: proc_macro::Span,
    pub choices: Vec<EnumChoice>,
}

impl EnumTemplate {
    pub fn new(name: String, name_span: proc_macro::Span, choices: Vec<EnumChoice>) -> Self {
        Self { name, name_span, choices }
    }
}

impl From<EnumTemplate> for Enum {
    fn from(template: EnumTemplate) -> Self {
        Self { name: DefinedType::new(template.name), choices: template.choices }
    }
}

#[derive(Clone)]
pub struct UnionTemplate {
    pub name: String,
    pub name_span: proc_macro::Span,
    pub choices: Vec<UnionChoice>,
}

impl UnionTemplate {
    pub fn new(name: String, name_span: proc_macro::Span, choices: Vec<UnionChoice>) -> Self {
        Self { name, name_span, choices }
    }
}

impl From<UnionTemplate> for Union {
    fn from(template: UnionTemplate) -> Self {
        Self { name: DefinedType::new(template.name), choices: template.choices }
    }
}

pub enum EntityTemplate {
    Struct(StructTemplate),
    Enum(EnumTemplate),
    Union(UnionTemplate),
}

pub struct Choices {
    pub enum_choices: Vec<EnumChoice>,
    pub union_choices: Vec<UnionChoice>,
}

impl Choices {
    pub fn new() -> Self {
        Self { enum_choices: Vec::new(), union_choices: Vec::new() }
    }
}

impl BuiltInType {
    pub fn from_name(name: &str) -> Result<Self, ()> {
        match name {
            "f64" => Ok(BuiltInType::F64),
            "String" => Ok(BuiltInType::String),
            "ObjectId" => Ok(BuiltInType::ObjectId),
            "bool" => Ok(BuiltInType::Bool),
            "Date" => Ok(BuiltInType::Date),
            "i32" => Ok(BuiltInType::I32),
            "i64" => Ok(BuiltInType::I64),
            "Bson" => Ok(BuiltInType::Bson),
            _ => return Err(()),
        }
    }
}
