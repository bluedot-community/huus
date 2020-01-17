// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Verification for instructions integrity.

use std::collections::{HashMap, HashSet};

use crate::definition::{generator::Generator, input::*, output::*};

// -------------------------------------------------------------------------------------------------

/// Helper structure gathering indexed field including this from children documents.
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

pub struct Validator {
    entities: Vec<EntityTemplate>,
    spec: Spec,
}

impl Validator {
    pub fn new(entities: Vec<EntityTemplate>) -> Self {
        Self { entities, spec: Spec::new() }
    }

    fn find_entity(&self, name: &str) -> Option<&EntityTemplate> {
        for entity in self.entities.iter() {
            match entity {
                EntityTemplate::Struct(struct_template) => {
                    if struct_template.struct_name == *name {
                        return Some(entity);
                    }
                }
                EntityTemplate::Enum(enum_spec) => {
                    if enum_spec.name == *name {
                        return Some(entity);
                    }
                }
                EntityTemplate::Union(union_spec) => {
                    if union_spec.name == *name {
                        return Some(entity);
                    }
                }
            }
        }
        None
    }

    pub fn verify(mut self) -> Result<Generator, ()> {
        self.validate()?;
        self.build()?;
        self.prepare();
        Ok(Generator::new(self.spec))
    }
}

// -------------------------------------------------------------------------------------------------
// Helper validation methods

impl Validator {
    fn validate_member(&self, member: &MemberTemplate) -> Result<(), ()> {
        match &member.container {
            ContainerTemplate::BTreeMap(string) | ContainerTemplate::HashMap(string) => {
                if let Ok(builtin_type) = BuiltInType::from_name(&string) {
                    match builtin_type {
                        BuiltInType::String => Ok(()),
                        _ => {
                            let msg = "Only 'String' can be used as a key".to_string();
                            member.variant_span.error(msg).emit();
                            Err(())
                        }
                    }
                } else {
                    match self.find_entity(&string) {
                        Some(EntityTemplate::Enum(..)) => Ok(()),
                        Some(_) => {
                            let msg = format!("Type '{}' is not an huus enum", string);
                            member.variant_span.error(msg).emit();
                            Err(())
                        }
                        None => {
                            let msg = format!("Type '{}' is neither not (pre)defined", string);
                            member.variant_span.error(msg).emit();
                            Err(())
                        }
                    }
                }
            }
            ContainerTemplate::Array | ContainerTemplate::Plain => Ok(()),
        }
    }

    fn validate(&self) -> Result<(), ()> {
        let mut is_ok = true;
        let mut entity_names = HashSet::new();
        let mut collection_names = HashSet::new();
        fn is_name_unique(name: &str, names: &mut HashSet<String>) -> bool {
            let contains = names.contains(name);
            if !contains {
                names.insert(name.to_string());
            }
            !contains
        }

        // Validate all the entities
        for entity in self.entities.iter() {
            match entity {
                EntityTemplate::Struct(struct_template) => {
                    // Make use the name is not repeated
                    if !is_name_unique(&struct_template.struct_name, &mut entity_names) {
                        struct_template.struct_name_span.error("Structure redefined").emit();
                    }

                    // Make use the collection name is not repeated
                    if let Some(collection_name) = &struct_template.collection_name {
                        if !is_name_unique(collection_name, &mut collection_names) {
                            struct_template
                                .collection_name_span
                                .error("Main document schema already assigned for this collection")
                                .emit();
                        }
                    }

                    // Validate all the members
                    for member in struct_template.members.iter() {
                        is_ok &= self.validate_member(member).is_ok();
                    }
                }
                EntityTemplate::Enum(enum_template) => {
                    // Make use the name is not repeated
                    if !is_name_unique(&enum_template.name, &mut entity_names) {
                        enum_template.name_span.error("Enum redefined").emit();
                    }
                }
                EntityTemplate::Union(union_template) => {
                    // Make use the name is not repeated
                    if !is_name_unique(&union_template.name, &mut entity_names) {
                        union_template.name_span.error("Union redefined").emit();
                    }
                }
            }
        }

        if is_ok {
            Ok(())
        } else {
            Err(())
        }
    }
}

// -------------------------------------------------------------------------------------------------
// Helper build methods

impl Validator {
    fn make_variant(&self, string: String, span: proc_macro::Span) -> Result<Variant, ()> {
        if let Ok(field) = BuiltInType::from_name(&string) {
            Ok(Variant::Field(field))
        } else if let Some(entity) = self.find_entity(&string) {
            match entity {
                EntityTemplate::Struct(..) => Ok(Variant::Struct(DefinedType::new(string))),
                EntityTemplate::Enum(..) => Ok(Variant::Enum(DefinedType::new(string))),
                EntityTemplate::Union(..) => Ok(Variant::Union(DefinedType::new(string))),
            }
        } else {
            span.error(format!("'{}' is neither predefined nor defined in this scope", string))
                .emit();
            Err(())
        }
    }

    fn convert_container(
        &self,
        template: ContainerTemplate,
        span: proc_macro::Span,
    ) -> Result<Container, ()> {
        Ok(match template {
            ContainerTemplate::Array => Container::Array,
            ContainerTemplate::BTreeMap(string) => {
                Container::BTreeMap(self.make_variant(string, span)?)
            }
            ContainerTemplate::HashMap(string) => {
                Container::HashMap(self.make_variant(string, span)?)
            }
            ContainerTemplate::Plain => Container::Plain,
        })
    }

    fn convert_struct(&self, struct_template: StructTemplate) -> Result<Struct, ()> {
        let mut members = Vec::with_capacity(struct_template.members.len());
        for template in struct_template.members {
            let member = Member::new(
                template.rust_name.expect("Member name incomplete"),
                template.db_name.expect("Database entry name incomplete"),
                self.make_variant(
                    template.variant.expect("Member type incomplete"),
                    template.variant_span.clone(),
                )?,
                self.convert_container(template.container, template.variant_span.clone())?,
                template.is_optional,
                template.is_indexed,
            );

            match member {
                Ok(member) => members.push(member),
                Err(SpecError::RustName(msg)) => template.rust_name_span.error(msg).emit(),
                Err(SpecError::DbName(msg)) => template.db_name_span.error(msg).emit(),
                Err(SpecError::Type(msg)) => template.variant_span.error(msg).emit(),
            }
        }

        Ok(Struct {
            struct_name: DefinedType::new(struct_template.struct_name),
            collection_name: struct_template.collection_name,
            members: members,
            indexed_fields: Vec::new(),
        })
    }

    fn build(&mut self) -> Result<(), ()> {
        for entity in self.entities.iter() {
            self.spec.entities.push(match entity {
                EntityTemplate::Struct(struct_template) => {
                    Entity::Struct(self.convert_struct(struct_template.clone())?)
                }
                EntityTemplate::Enum(enum_spec) => Entity::Enum(enum_spec.clone().into()),
                EntityTemplate::Union(union_spec) => Entity::Union(union_spec.clone().into()),
            });
        }
        Ok(())
    }
}

// -------------------------------------------------------------------------------------------------
// Helper build methods

impl Validator {
    fn prepare(&mut self) {
        let mut indexed_fields = IndexedFields::new(&self.spec).prepare();
        for entity in self.spec.entities.iter_mut() {
            match entity {
                Entity::Struct(struct_spec) => {
                    struct_spec.indexed_fields = indexed_fields
                        .remove(&struct_spec.struct_name.name)
                        .expect("Indexed fields not found")
                }
                Entity::Enum(_) | Entity::Union(_) => {
                    // nothing to do
                }
            }
        }
    }
}
