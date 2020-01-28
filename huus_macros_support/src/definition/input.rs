// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Structures for instructions parsing.

pub use crate::definition::output::{
    BuiltInType, DefinedType, Enum, EnumChoice, Union, UnionChoice,
};

/// Represents the type of container for member.
#[derive(Clone)]
pub enum ContainerTemplate {
    /// Corresponds to `Vec`.
    Array,

    /// Corresponds to `BTreeMap`.
    BTreeMap(String),

    /// Corresponds to `HashMap`.
    HashMap(String),

    /// Corresponds to a type not contained in any container.
    Plain,
}

/// Helps in parsing and reporting errors related to structure member (database object field).
#[derive(Clone)]
pub struct MemberTemplate {
    /// Name to be used in generated code.
    pub rust_name: Option<String>,

    /// Span of the `rust_name`.
    pub rust_name_span: proc_macro::Span,

    /// Name to be used in database queries.
    pub db_name: Option<String>,

    /// Span of the `db_name`.
    pub db_name_span: proc_macro::Span,

    /// Name of the type.
    pub variant: Option<String>,

    /// Container the data is to be stored in.
    pub container: ContainerTemplate,

    /// Span of the `variant`.
    pub variant_span: proc_macro::Span,

    /// Specifies if the member is optional.
    pub is_optional: bool,

    /// Specifies if an index should be created for the given database field.
    pub is_indexed: bool,
}

impl MemberTemplate {
    /// Constructs a new `MemberTemplate`.
    pub fn new() -> Self {
        Self {
            rust_name: None,
            rust_name_span: proc_macro::Span::call_site(),
            db_name: None,
            db_name_span: proc_macro::Span::call_site(),
            variant: None,
            variant_span: proc_macro::Span::call_site(),
            container: ContainerTemplate::Plain,
            is_optional: false,
            is_indexed: false,
        }
    }
}

/// Helps in parsing and reporting errors related to structures (database objects)
#[derive(Clone)]
pub struct StructTemplate {
    /// Name of the structure
    pub struct_name: String,

    /// Span of the `struct_name`.
    pub struct_name_span: proc_macro::Span,

    /// Name of the collection. If specified this is the type of the main document stored in that
    /// collection. For embedded documents the collection name should be `None`.
    pub collection_name: Option<String>,

    /// Span of the `collection_name`.
    pub collection_name_span: proc_macro::Span,

    /// List of all members of this structure (fields in the database object).
    pub members: Vec<MemberTemplate>,
}

/// Helps in parsing and reporting errors related to enums.
#[derive(Clone)]
pub struct EnumTemplate {
    /// Name of the enum.
    pub name: String,

    /// Span of the `name`.
    pub name_span: proc_macro::Span,

    /// List of possible enum variants.
    pub choices: Vec<EnumChoice>,
}

impl EnumTemplate {
    /// Constructs a new `EnumTemplate`.
    pub fn new(name: String, name_span: proc_macro::Span, choices: Vec<EnumChoice>) -> Self {
        Self { name, name_span, choices }
    }
}

impl From<EnumTemplate> for Enum {
    fn from(template: EnumTemplate) -> Self {
        Self { name: DefinedType::new(template.name), choices: template.choices }
    }
}

/// Helps in parsing and reporting errors related to unions.
#[derive(Clone)]
pub struct UnionTemplate {
    /// Name of the union
    pub name: String,

    /// Span of the `name`.
    pub name_span: proc_macro::Span,

    /// List of possible union variants.
    pub choices: Vec<UnionChoice>,
}

impl UnionTemplate {
    /// Constructs a new `UnionTemplate`.
    pub fn new(name: String, name_span: proc_macro::Span, choices: Vec<UnionChoice>) -> Self {
        Self { name, name_span, choices }
    }
}

impl From<UnionTemplate> for Union {
    fn from(template: UnionTemplate) -> Self {
        Self { name: DefinedType::new(template.name), choices: template.choices }
    }
}

/// Holds information about parsed entities (structures, enums and unions).
pub enum EntityTemplate {
    /// Holds information about parsed structure.
    Struct(StructTemplate),

    /// Holds information about parsed enum.
    Enum(EnumTemplate),

    /// Holds information about parsed union.
    Union(UnionTemplate),
}

/// Helps parsing enum- and union-type variants.
pub struct Choices {
    /// List of enum-type variants (without any type).
    pub enum_choices: Vec<EnumChoice>,

    /// List of union-type variants (referencing a structure).
    pub union_choices: Vec<UnionChoice>,
}

impl Choices {
    /// Constructs new `Choices`.
    pub fn new() -> Self {
        Self { enum_choices: Vec::new(), union_choices: Vec::new() }
    }
}
