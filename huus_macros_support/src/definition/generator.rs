// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Generation of the code for macros defining the data types.

use askama::Template;

use crate::definition::output::{
    BuiltInType, Container, Entity, Enum, Member, Spec, Struct, Union, Variant,
};

// -------------------------------------------------------------------------------------------------

impl BuiltInType {
    pub fn to_data(&self) -> &'static str {
        match self {
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
        match self {
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
        match self {
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
        match self {
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
        match self {
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
        let output = match self {
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

    pub fn to_short_update(&self) -> String {
        match self {
            Variant::Field(field) => field.to_update().to_string(),
            Variant::Struct(name) => name.to_update(),
            Variant::Enum(name) => name.to_update(),
            Variant::Union(name) => name.to_update(),
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
                let update = self.variant.to_short_update();
                let value = self.variant.to_value();
                format!("huus::updates::ArrayEntry<{}, {}>", update, value)
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

/// Helper structure for calling rust code from within a template.
#[derive(Clone)]
struct GeneratorCallback;

impl GeneratorCallback {
    /// Constructs a new `GeneratorCallback`.
    pub fn new() -> Self {
        Self
    }

    pub fn make_coll_name(&self, string: &String) -> String {
        fn capitalize(string: &str) -> String {
            if string.len() > 0 {
                string.chars().next().unwrap().to_uppercase().to_string() + &string[1..]
            } else {
                String::new()
            }
        }

        string.split('_').map(capitalize).collect::<Vec<String>>().join("")
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Template)]
#[template(path = "struct_definition.rs", escape = "none")]
struct StructDefinitionTemplate<'a> {
    pub spec: Struct,
    pub generator: &'a GeneratorCallback,
}

impl<'a> StructDefinitionTemplate<'a> {
    pub fn new(spec: Struct, generator: &'a GeneratorCallback) -> Self {
        Self { spec, generator }
    }
}

fn make_struct_definition_output(spec: Struct, generator: &GeneratorCallback) -> String {
    StructDefinitionTemplate::new(spec, generator).render().expect("Render struct template")
}

// -------------------------------------------------------------------------------------------------

#[derive(Template)]
#[template(path = "enum_definition.rs", escape = "none")]
struct EnumDefinitionTemplate {
    pub spec: Enum,
}

impl EnumDefinitionTemplate {
    pub fn new(spec: Enum) -> Self {
        Self { spec }
    }
}

fn make_enum_definition_output(spec: Enum) -> String {
    EnumDefinitionTemplate::new(spec).render().expect("Render enum template")
}

// -------------------------------------------------------------------------------------------------

#[derive(Template)]
#[template(path = "union_definition.rs", escape = "none")]
struct UnionDefinitionTemplate {
    pub spec: Union,
}

impl UnionDefinitionTemplate {
    pub fn new(spec: Union) -> Self {
        Self { spec }
    }
}

fn make_union_definition_output(spec: Union) -> String {
    UnionDefinitionTemplate::new(spec).render().expect("Render union template")
}

// -------------------------------------------------------------------------------------------------

#[derive(Template)]
#[template(path = "struct_formulation.rs", escape = "none")]
struct StructFormulationTemplate<'a> {
    pub spec: Struct,
    pub generator: &'a GeneratorCallback,
}

impl<'a> StructFormulationTemplate<'a> {
    pub fn new(spec: Struct, generator: &'a GeneratorCallback) -> Self {
        Self { spec, generator }
    }
}

fn make_struct_formulation_output(spec: Struct, generator: &GeneratorCallback) -> String {
    StructFormulationTemplate::new(spec, generator).render().expect("Render struct template")
}

// -------------------------------------------------------------------------------------------------

#[derive(Template)]
#[template(path = "enum_formulation.rs", escape = "none")]
struct EnumFormulationTemplate {
    pub spec: Enum,
}

impl EnumFormulationTemplate {
    pub fn new(spec: Enum) -> Self {
        Self { spec }
    }
}

fn make_enum_formulation_output(spec: Enum) -> String {
    EnumFormulationTemplate::new(spec).render().expect("Render enum template")
}

// -------------------------------------------------------------------------------------------------

#[derive(Template)]
#[template(path = "union_formulation.rs", escape = "none")]
struct UnionFormulationTemplate {
    pub spec: Union,
}

impl UnionFormulationTemplate {
    pub fn new(spec: Union) -> Self {
        Self { spec }
    }
}

fn make_union_formulation_output(spec: Union) -> String {
    UnionFormulationTemplate::new(spec).render().expect("Render union template")
}

// -------------------------------------------------------------------------------------------------

pub struct Generator {
    spec: Spec,
}

impl Generator {
    pub fn new(spec: Spec) -> Self {
        Self { spec }
    }

    pub fn into_spec(self) -> Spec {
        self.spec
    }

    pub fn generate_definition(self) -> proc_macro::TokenStream {
        let generator = GeneratorCallback::new();
        let mut entities = Vec::new();
        for entity in self.spec.entities {
            entities.push(match entity {
                Entity::Struct(struct_spec) => make_struct_definition_output(struct_spec, &generator),
                Entity::Enum(enum_spec) => make_enum_definition_output(enum_spec),
                Entity::Union(union_spec) => make_union_definition_output(union_spec),
            });
        }
        entities.join("\n\n").parse().expect("Parse into TokenStream")
    }

    pub fn generate_formulation(self) -> proc_macro::TokenStream {
        let generator = GeneratorCallback::new();
        let mut entities = Vec::new();
        for entity in self.spec.entities {
            entities.push(match entity {
                Entity::Struct(struct_spec) => make_struct_formulation_output(struct_spec, &generator),
                Entity::Enum(enum_spec) => make_enum_formulation_output(enum_spec),
                Entity::Union(union_spec) => make_union_formulation_output(union_spec),
            });
        }
        entities.join("\n\n").parse().expect("Parse into TokenStream")
    }
}
