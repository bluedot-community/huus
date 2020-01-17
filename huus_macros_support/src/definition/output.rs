// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Structures for code generation.

pub enum SpecError {
    RustName(String),
    DbName(String),
    Type(String),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BuiltInType {
    F64,
    String,
    ObjectId,
    Bool,
    Date,
    I32,
    I64,
    Bson,
}

impl BuiltInType {
    fn allows_indexing(&self) -> bool {
        match self {
            BuiltInType::String | BuiltInType::ObjectId => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DefinedType {
    pub name: String,
}

impl DefinedType {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn to_data(&self) -> String {
        self.name.clone() + "Data"
    }

    pub fn to_insert(&self) -> String {
        self.name.clone() + "Insert"
    }

    pub fn to_filter(&self) -> String {
        self.name.clone() + "Filter"
    }

    pub fn to_value(&self) -> String {
        self.name.clone() + "Value"
    }

    pub fn to_update(&self) -> String {
        self.name.clone() + "Update"
    }
}

impl PartialEq<str> for DefinedType {
    fn eq(&self, name: &str) -> bool {
        self.name == name
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Variant {
    Field(BuiltInType),
    Struct(DefinedType),
    Enum(DefinedType),
    Union(DefinedType),
}

impl Variant {
    fn allows_indexing(&self) -> bool {
        match self {
            Variant::Field(field) => field.allows_indexing(),
            _ => false,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Container {
    Array,
    BTreeMap(Variant),
    HashMap(Variant),
    Plain,
}

impl Container {
    pub fn is_plain(&self) -> bool {
        *self == Self::Plain
    }

    pub fn is_array(&self) -> bool {
        *self == Self::Array
    }
}

#[derive(Clone, Debug)]
pub struct Member {
    pub rust_name: String,
    pub db_name: String,
    pub variant: Variant,
    pub container: Container,
    pub is_optional: bool,
    pub is_indexed: bool,
}

impl Member {
    pub fn new(
        rust_name: String,
        db_name: String,
        variant: Variant,
        container: Container,
        is_optional: bool,
        is_indexed: bool,
    ) -> Result<Self, SpecError> {
        // Check if the name is allowed
        const FORBIDDEN_PREFIX: &'static str = "_huus";
        if rust_name.starts_with(FORBIDDEN_PREFIX) {
            let msg = format!("Member name cannot start with '{}'", FORBIDDEN_PREFIX);
            return Err(SpecError::RustName(msg));
        }

        // Check if the database name contains only allowed characters
        for character in db_name.chars() {
            if !character.is_alphanumeric() && character != '_' {
                let msg = format!(
                    "Field name can contain only alphanumerics and underscore, but '{}' was used",
                    character
                );
                return Err(SpecError::DbName(msg));
            }
        }

        // Check if indexing is requested only for types supporting indexing
        if is_indexed && !variant.allows_indexing() {
            let msg = "Indexing not supported for this type".to_string();
            return Err(SpecError::Type(msg));
        }

        Ok(Member { rust_name, db_name, variant, container, is_optional, is_indexed })
    }
}

#[derive(Clone, Debug)]
pub struct EnumChoice {
    pub rust_name: String,
    pub db_name: String,
}

impl EnumChoice {
    pub fn new(rust_name: String, db_name: String) -> Self {
        Self { rust_name, db_name }
    }
}

#[derive(Clone, Debug)]
pub struct UnionChoice {
    pub rust_name: String,
    pub db_name: String,
    pub variant: DefinedType,
}

impl UnionChoice {
    pub fn new(rust_name: String, db_name: String, variant: DefinedType) -> Self {
        Self { rust_name, db_name, variant }
    }
}

#[derive(Clone, Debug)]
pub struct Struct {
    pub struct_name: DefinedType,
    pub collection_name: Option<String>,
    pub members: Vec<Member>,
    pub indexed_fields: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct Enum {
    pub name: DefinedType,
    pub choices: Vec<EnumChoice>,
}

impl Enum {
    pub fn to_db_names(&self) -> Vec<String> {
        let mut result = Vec::with_capacity(self.choices.len());
        for choice in self.choices.iter() {
            result.push(choice.db_name.clone());
        }
        result
    }
}

#[derive(Clone, Debug)]
pub struct Union {
    pub name: DefinedType,
    pub choices: Vec<UnionChoice>,
}

#[derive(Clone, Debug)]
pub enum Entity {
    Struct(Struct),
    Enum(Enum),
    Union(Union),
}

pub struct Spec {
    pub entities: Vec<Entity>,
}

impl Spec {
    pub fn new() -> Self {
        Self { entities: Vec::new() }
    }

    pub fn find_entity(&self, name: &str) -> Option<&Entity> {
        for entity in self.entities.iter() {
            match entity {
                Entity::Struct(struct_spec) => {
                    if struct_spec.struct_name == *name {
                        return Some(entity);
                    }
                }
                Entity::Enum(enum_spec) => {
                    if enum_spec.name == *name {
                        return Some(entity);
                    }
                }
                Entity::Union(union_spec) => {
                    if union_spec.name == *name {
                        return Some(entity);
                    }
                }
            }
        }
        None
    }

    pub fn find_entity_for_collection(&self, name: &str) -> Option<&Entity> {
        for entity in self.entities.iter() {
            match entity {
                Entity::Struct(struct_spec) => {
                    if let Some(collection_name) = &struct_spec.collection_name {
                        if collection_name == name {
                            return Some(entity);
                        }
                    }
                }
                _ => {}
            }
        }
        None
    }
}
