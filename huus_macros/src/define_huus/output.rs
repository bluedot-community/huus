// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Generation of the code for `define_huus` macro.

use std::collections::HashMap;

use askama::Template;

use super::input::{
    BuiltInType, Container, DefinedType, Entity, Enum, Member, PredefinedType, Spec, Struct, Union,
    Variant,
};

// -------------------------------------------------------------------------------------------------

impl PredefinedType {
    pub fn to_data(&self) -> &'static str {
        match self.variant {
            BuiltInType::F64 => "f64",
            BuiltInType::String => "String",
            BuiltInType::ObjectId => "huus::types::ObjectId",
            BuiltInType::Bool => "bool",
            BuiltInType::Date => "huus::types::Date",
            BuiltInType::I32 => "i32",
            BuiltInType::I64 => "i64",
            BuiltInType::Bson => "bson::Document",
        }
    }

    pub fn to_filter(&self) -> &'static str {
        match self.variant {
            BuiltInType::F64 => "huus::filters::F64Entry",
            BuiltInType::String => "huus::filters::StringEntry",
            BuiltInType::ObjectId => "huus::filters::ObjectIdEntry",
            BuiltInType::Bool => "huus::filters::BooleanEntry",
            BuiltInType::Date => "huus::filters::DateEntry",
            BuiltInType::I32 => "huus::filters::I32Entry",
            BuiltInType::I64 => "huus::filters::I64Entry",
            BuiltInType::Bson => "huus::filters::BsonEntry",
        }
    }

    pub fn to_value(&self) -> &'static str {
        match self.variant {
            BuiltInType::F64 => "f64",
            BuiltInType::String => "String",
            BuiltInType::ObjectId => "huus::types::ObjectId",
            BuiltInType::Bool => "bool",
            BuiltInType::Date => "huus::types::Date",
            BuiltInType::I32 => "i32",
            BuiltInType::I64 => "i64",
            BuiltInType::Bson => "bson::Document",
        }
    }

    pub fn to_update(&self) -> &'static str {
        match self.variant {
            BuiltInType::F64 => "huus::updates::F64Entry",
            BuiltInType::String => "huus::updates::StringEntry",
            BuiltInType::ObjectId => "huus::updates::ObjectIdEntry",
            BuiltInType::Bool => "huus::updates::BooleanEntry",
            BuiltInType::Date => "huus::updates::DateEntry",
            BuiltInType::I32 => "huus::updates::I32Entry",
            BuiltInType::I64 => "huus::updates::I64Entry",
            BuiltInType::Bson => "huus::updates::BsonEntry",
        }
    }

    pub fn from_doc_getter(&self) -> &'static str {
        match self.variant {
            BuiltInType::F64 => "get_f64",
            BuiltInType::String => "get_str",
            BuiltInType::ObjectId => "get_object_id",
            BuiltInType::Bool => "get_bool",
            BuiltInType::Date => "get_utc_datetime",
            BuiltInType::I32 => "get_i32",
            BuiltInType::I64 => "get_i64",
            BuiltInType::Bson => "get_document",
        }
    }

    pub fn to_conversion(&self) -> &'static str {
        let output = match self.variant {
            BuiltInType::F64 => "value",
            BuiltInType::String => "value.to_string()",
            BuiltInType::ObjectId => "value.clone()",
            BuiltInType::Bool => "value",
            BuiltInType::Date => "value.clone()",
            BuiltInType::I32 => "value",
            BuiltInType::I64 => "value",
            BuiltInType::Bson => "value.clone()",
        };
        output.into()
    }
}

impl DefinedType {
    fn to_data(&self) -> String {
        self.name.clone() + "Data"
    }

    fn to_filter(&self) -> String {
        self.name.clone() + "Filter"
    }

    fn to_value(&self) -> String {
        self.name.clone() + "Value"
    }

    fn to_update(&self) -> String {
        self.name.clone() + "Update"
    }
}

impl Variant {
    pub fn to_data(&self) -> String {
        match self {
            Variant::Field(field) => field.to_data().to_string(),
            Variant::Struct(name) => name.to_data(),
            Variant::Enum(name) => name.to_data(),
            Variant::Union(name) => name.to_data(),
        }
    }

    pub fn to_long_filter(&self) -> String {
        let output = match self {
            Variant::Field(field) => field.to_filter().to_string(),
            Variant::Struct(name) => {
                format!("huus::filters::ObjectEntry<{}, {}>", name.to_filter(), name.to_data())
            }
            Variant::Enum(name) => format!("huus::filters::EnumEntry<{}>", name.to_data()),
            Variant::Union(name) => {
                format!("huus::filters::ObjectEntry<{}, {}>", name.to_filter(), name.to_data())
            }
        };
        output.into()
    }

    pub fn to_short_filter(&self) -> String {
        match self {
            Variant::Field(field) => field.to_filter().to_string(),
            Variant::Struct(name) => name.to_filter(),
            Variant::Enum(name) => name.to_data(),
            Variant::Union(name) => name.to_filter(),
        }
    }

    pub fn to_value(&self) -> String {
        match self {
            Variant::Field(field) => field.to_value().to_string(),
            Variant::Struct(name) => name.to_value(),
            Variant::Enum(name) => name.to_value(),
            Variant::Union(name) => name.to_value(),
        }
    }

    pub fn to_update(&self) -> String {
        match self {
            Variant::Field(field) => field.to_update().to_string(),
            Variant::Struct(name) => {
                format!("huus::updates::ObjectEntry<{}, {}>", name.to_update(), name.to_value())
            }
            Variant::Enum(name) => format!("huus::updates::EnumEntry<{}>", name.to_value()),
            Variant::Union(name) => {
                format!("huus::updates::ObjectEntry<{}, {}>", name.to_update(), name.to_value())
            }
        }
    }

    pub fn from_doc_getter(&self) -> &'static str {
        match self {
            Variant::Field(field) => field.from_doc_getter(),
            Variant::Struct(_) => "get_document",
            Variant::Enum(_) => "get_str",
            Variant::Union(_) => "get_document",
        }
    }

    pub fn to_conversion(&self) -> String {
        match self {
            Variant::Field(field) => field.to_conversion().to_string(),
            Variant::Struct(name) => format!("{}::from_doc(value.clone())?", name.to_data()),
            Variant::Enum(name) => format!("{}::from_str(&value)?", name.to_data()),
            Variant::Union(name) => format!("{}::from_doc(value.clone())?", name.to_data()),
        }
    }
}

impl Member {
    fn to_data(&self) -> String {
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

    fn to_filter(&self) -> String {
        match &self.container {
            Container::Array => {
                let key = self.variant.to_short_filter();
                let value = self.variant.to_data();
                format!("huus::filters::ArrayEntry<{}, {}>", key, value)
            }
            Container::BTreeMap(key_variant) => {
                let key = key_variant.to_data();
                let value = self.variant.to_data();
                format!("huus::filters::BTreeMapEntry<{}, {}>", key, value)
            }
            Container::HashMap(key_variant) => {
                let key = key_variant.to_data();
                let value = self.variant.to_data();
                format!("huus::filters::HashMapEntry<{}, {}>", key, value)
            }
            Container::Plain => self.variant.to_long_filter(),
        }
    }

    fn to_value(&self) -> String {
        // TODO: Add separate entries for maps.
        match &self.container {
            Container::Array => format!("huus::values::ArrayEntry<{}>", self.variant.to_value()),
            Container::HashMap(key_variant) => {
                let key = key_variant.to_value();
                let value = self.variant.to_data();
                format!("huus::values::Entry<std::collections::HashMap<{}, {}>>", key, value)
            }
            Container::BTreeMap(key_variant) => {
                let key = key_variant.to_value();
                let value = self.variant.to_data();
                format!("huus::values::Entry<std::collections::BTreeMap<{}, {}>>", key, value)
            }
            Container::Plain => format!("huus::values::Entry<{}>", self.variant.to_value()),
        }
    }

    fn to_update(&self) -> String {
        match &self.container {
            Container::Array => {
                let variant = self.variant.to_value();
                format!("huus::updates::ArrayEntry<{}>", variant)
            }
            Container::BTreeMap(key_variant) => {
                let key = key_variant.to_data();
                let value = self.variant.to_data();
                format!("huus::updates::BTreeMapEntry<{}, {}>", key, value)
            }
            Container::HashMap(key_variant) => {
                let key = key_variant.to_data();
                let value = self.variant.to_data();
                format!("huus::updates::HashMapEntry<{}, {}>", key, value)
            }
            Container::Plain => self.variant.to_update(),
        }
    }

    pub fn from_doc_getter(&self) -> &'static str {
        match self.container {
            Container::Array => "get_array",
            Container::HashMap(_) => "get_document",
            Container::BTreeMap(_) => "get_document",
            Container::Plain => self.variant.from_doc_getter(),
        }
    }

    pub fn to_conversion(&self) -> String {
        match self.container {
            Container::Array => "value.clone().huus_into_struct()?".to_string(),
            Container::HashMap(_) => "value.clone().huus_into_struct()?".to_string(),
            Container::BTreeMap(_) => "value.clone().huus_into_struct()?".to_string(),
            Container::Plain => self.variant.to_conversion(),
        }
    }

    pub fn to_default(&self) -> Option<&str> {
        match self.container {
            Container::Array => Some("Vec::new()"),
            Container::HashMap(_) => Some("std::collections::HashMap::new()"),
            Container::BTreeMap(_) => Some("std::collections::BTreeMap::new()"),
            Container::Plain => None,
        }

    }
}

// -------------------------------------------------------------------------------------------------

struct IndexedFields<'a> {
    spec: &'a Spec,
    fields: HashMap<String, Vec<String>>,
}

impl<'a> IndexedFields<'a> {
    fn new(spec: &'a Spec) -> Self {
        Self { spec: spec, fields: HashMap::new() }
    }

    fn prepare(mut self) -> HashMap<String, Vec<String>> {
        for entity in self.spec.entities.iter() {
            self.prepare_entity(entity);
        }
        self.fields
    }

    fn prepare_entity(&mut self, entity: &Entity) {
        match entity {
            Entity::Struct(struct_spec) => {
                if self.fields.get(&struct_spec.struct_name.name).is_none() {
                    self.prepare_struct(struct_spec);
                }
            }
            Entity::Enum(enum_spec) => {
                if self.fields.get(&enum_spec.name.name).is_none() {
                    self.prepare_enum(enum_spec);
                }
            }
            Entity::Union(union_spec) => {
                if self.fields.get(&union_spec.name.name).is_none() {
                    self.prepare_union(union_spec);
                }
            }
        }
    }

    fn prepare_struct(&mut self, struct_spec: &Struct) {
        let mut indexed_fields = Vec::new();
        for member in struct_spec.members.iter() {
            match &member.variant {
                Variant::Field(_) => {
                    if member.is_indexed {
                        indexed_fields.push(member.db_name.clone());
                    }
                }
                Variant::Struct(variant) | Variant::Union(variant) => {
                    let entity = self
                        .spec
                        .find_entity(&variant.name)
                        .expect(&format!("Failed to find '{}'", variant.name));
                    self.prepare_entity(entity);
                    let struct_indexed_fields = self
                        .fields
                        .get(&variant.name)
                        .expect(&format!("Failed to find indexed fields for '{}'", variant.name));

                    let keys = match &member.container {
                        Container::Array | Container::Plain => Vec::new(),
                        Container::BTreeMap(variant) | Container::HashMap(variant) => match variant
                        {
                            Variant::Field(_) => Vec::new(),
                            Variant::Struct(key_type)
                            | Variant::Enum(key_type)
                            | Variant::Union(key_type) => {
                                let entity = self.spec.find_entity(&key_type.name);
                                match entity {
                                    Some(Entity::Enum(enum_spec)) => enum_spec.to_db_names(),
                                    _ => Vec::new(),
                                }
                            }
                        },
                    };

                    let base = member.db_name.clone() + ".";
                    if keys.len() > 0 {
                        for key in keys {
                            for field in struct_indexed_fields.iter() {
                                indexed_fields.push(base.clone() + &key + "." + field);
                            }
                        }
                    } else {
                        for field in struct_indexed_fields.iter() {
                            indexed_fields.push(base.clone() + field);
                        }
                    }
                }
                Variant::Enum(_) => {}
            };
        }

        self.fields.insert(struct_spec.struct_name.name.clone(), indexed_fields);
    }

    fn prepare_enum(&mut self, enum_spec: &Enum) {
        self.fields.insert(enum_spec.name.name.clone(), Vec::new());
    }

    fn prepare_union(&mut self, union_spec: &Union) {
        self.fields.insert(union_spec.name.name.clone(), Vec::new());
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Template)]
#[template(path = "struct.rs", escape = "none")]
struct StructTemplate {
    pub spec: Struct,
    pub indexed_fields: Vec<String>,
}

impl StructTemplate {
    pub fn new(spec: Struct, indexed_fields: Vec<String>) -> Self {
        Self { spec, indexed_fields }
    }
}

fn make_struct_output(spec: Struct, indexed_fields: Vec<String>) -> String {
    StructTemplate::new(spec, indexed_fields).render().expect("Render struct template")
}

// -------------------------------------------------------------------------------------------------

#[derive(Template)]
#[template(path = "enum.rs", escape = "none")]
struct EnumTemplate {
    pub spec: Enum,
}

impl EnumTemplate {
    pub fn new(spec: Enum) -> Self {
        Self { spec }
    }
}

fn make_enum_output(spec: Enum) -> String {
    EnumTemplate::new(spec).render().expect("Render enum template")
}

// -------------------------------------------------------------------------------------------------

#[derive(Template)]
#[template(path = "union.rs", escape = "none")]
struct UnionTemplate {
    pub spec: Union,
}

impl UnionTemplate {
    pub fn new(spec: Union) -> Self {
        Self { spec }
    }
}

fn make_union_output(spec: Union) -> String {
    UnionTemplate::new(spec).render().expect("Render union template")
}

// -------------------------------------------------------------------------------------------------

pub fn make_output(spec: Spec) -> proc_macro::TokenStream {
    let mut entities = Vec::new();
    let indexed_fields_map = IndexedFields::new(&spec).prepare();
    for entity in spec.entities {
        entities.push(match entity {
            Entity::Struct(struct_spec) => {
                let msg =
                    format!("Failed to find indexed fields for '{}'", struct_spec.struct_name.name);
                let indexed_fields =
                    indexed_fields_map.get(&struct_spec.struct_name.name).expect(&msg);
                make_struct_output(struct_spec, indexed_fields.clone())
            }
            Entity::Enum(enum_spec) => make_enum_output(enum_spec),
            Entity::Union(union_spec) => make_union_output(union_spec),
        });
    }
    entities.join("\n\n").parse().expect("Parse into TokenStream")
}
