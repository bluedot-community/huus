// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Generation of the code for macros defining the data types.

use askama::Template;

use crate::definition::output::{Entity, Enum, Schema, Struct, Union};

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

/// Template used for structure definition code generation.
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

/// Template used for enum definition code generation.
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

/// Template used for union definition code generation.
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

/// Template used for structure formulation code generation.
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

/// Template used for enum formulation code generation.
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

/// Template used for union formulation code generation.
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

/// Query definition/formulation code generator.
pub struct Generator {
    schema: Schema,
}

impl Generator {
    /// Constructs a new `Generator`.
    pub fn new(schema: Schema) -> Self {
        Self { schema }
    }

    /// Returns the schema to be used for code generation.
    pub fn into_schema(self) -> Schema {
        self.schema
    }

    /// Generates the definition code basing on the schema.
    ///
    /// By `definition` we mean structures with members corresponding to database fields generating
    /// query BSONs.
    pub fn generate_definition(self) -> proc_macro::TokenStream {
        let generator = GeneratorCallback::new();
        let mut entities = Vec::new();
        for entity in self.schema.entities {
            entities.push(match entity {
                Entity::Struct(struct_spec) => {
                    make_struct_definition_output(struct_spec, &generator)
                }
                Entity::Enum(enum_spec) => make_enum_definition_output(enum_spec),
                Entity::Union(union_spec) => make_union_definition_output(union_spec),
            });
        }
        entities.join("\n\n").parse().expect("Parse into TokenStream")
    }

    /// Generates the formulation code basing on the schema.
    ///
    /// By `formulation` we mean structures build by `data`, `filter` and `update` macros from
    /// `huus_macro`.
    pub fn generate_formulation(self) -> proc_macro::TokenStream {
        let generator = GeneratorCallback::new();
        let mut entities = Vec::new();
        for entity in self.schema.entities {
            entities.push(match entity {
                Entity::Struct(struct_spec) => {
                    make_struct_formulation_output(struct_spec, &generator)
                }
                Entity::Enum(enum_spec) => make_enum_formulation_output(enum_spec),
                Entity::Union(union_spec) => make_union_formulation_output(union_spec),
            });
        }
        entities.join("\n\n").parse().expect("Parse into TokenStream")
    }
}
