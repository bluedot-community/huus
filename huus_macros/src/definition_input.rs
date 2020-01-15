// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Parsing the token stream of `define_huus` macro.

use std::str::FromStr;

use crate::{
    parser::{ExpectedTokenTree, Parser},
    definition_spec::{
        BuiltInType, Container, DefinedType, Entity, Enum, Member, Spec, Struct,
        Union, Variant, SpecError, UnionChoice, EnumChoice,
    },
};

const SPAN: &str = "Span should be present";

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

#[derive(Clone)]
pub enum ContainerTemplate {
    Array,
    BTreeMap(String),
    HashMap(String),
    Plain,
}

#[derive(Clone)]
struct MemberTemplate {
    rust_name: Option<String>,
    rust_name_span: proc_macro::Span,
    db_name: Option<String>,
    db_name_span: proc_macro::Span,
    variant: Option<String>,
    variant_span: proc_macro::Span,
    is_optional: bool,
    is_indexed: bool,
    container: ContainerTemplate,
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
struct StructTemplate {
    pub struct_name: String,
    pub struct_name_span: proc_macro2::Span,
    pub collection_name: Option<String>,
    pub members: Vec<MemberTemplate>,
}

enum EntityTemplate {
    Struct(StructTemplate),
    Enum(Enum),
    Union(Union),
}

// -------------------------------------------------------------------------------------------------

struct SpecBuilder {
    pub entities: Vec<EntityTemplate>,
}

impl SpecBuilder {
    fn new() -> Self {
        Self { entities: Vec::new() }
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

    fn make_container(
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

    fn build_struct(&self, struct_template: StructTemplate) -> Result<Struct, ()> {
        let mut members = Vec::with_capacity(struct_template.members.len());
        for template in struct_template.members {
            let member = Member::new(
                template.rust_name.expect("Member name incomplete"),
                template.db_name.expect("Database entry name incomplete"),
                self.make_variant(
                    template.variant.expect("Member type incomplete"),
                    template.variant_span.clone(),
                )?,
                self.make_container(template.container, template.variant_span.clone())?,
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
        })
    }

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
        // Check if map key is an enumeration type
        for entity in self.entities.iter() {
            match entity {
                EntityTemplate::Struct(struct_template) => {
                    for member in struct_template.members.iter() {
                        self.validate_member(member)?;
                    }
                }
                EntityTemplate::Enum(_) | EntityTemplate::Union(_) => {}
            }
        }

        Ok(())
    }

    fn build(self) -> Result<Spec, ()> {
        self.validate()?;
        let mut result = Spec::new();
        for entity in self.entities.iter() {
            result.entities.push(match entity {
                EntityTemplate::Struct(struct_template) => {
                    Entity::Struct(self.build_struct(struct_template.clone())?)
                }
                EntityTemplate::Enum(enum_spec) => {
                    Entity::Enum(enum_spec.clone())
                }
                EntityTemplate::Union(union_spec) => {
                    Entity::Union(union_spec.clone())
                }
            });
        }
        Ok(result)
    }
}

struct ParseChoicesResult {
    enum_choices: Vec<EnumChoice>,
    union_choices: Vec<UnionChoice>,
}

impl ParseChoicesResult {
    fn new() -> Self {
        Self { enum_choices: Vec::new(), union_choices: Vec::new() }
    }
}

fn parse_struct(parser: &mut Parser) -> Result<EntityTemplate, ()> {
    let name_ident = parser.expect_ident(None)?;
    let collection_name = if parser.is_ident() {
        let _ = parser.expect_ident(Some("in"))?;
        Some(parser.expect_string()?)
    } else {
        None
    };
    let members = parse_members(parser.expect_group()?)?;

    let struct_name = name_ident.to_string();
    let struct_name_span = name_ident.span().into();
    Ok(EntityTemplate::Struct(StructTemplate {
        struct_name: struct_name,
        collection_name: collection_name,
        members: members,
        struct_name_span: struct_name_span,
    }))
}

fn parse_members(group: proc_macro::Group) -> Result<Vec<MemberTemplate>, ()> {
    const ARRAY: &str = "Vec";
    const BTREEMAP: &str = "BTreeMap";
    const HASHMAP: &str = "HashMap";

    let mut result = Vec::new();
    let mut parser = Parser::new(group.stream());
    loop {
        // Parse name
        let mut member = MemberTemplate::new();
        let ident = parser.expect_ident(None)?;
        member.rust_name = Some(ident.to_string());
        member.rust_name_span = ident.span().into();
        if parser.is_ident() {
            let _ = parser.expect_ident(Some("as"))?;
            member.db_name = Some(parser.expect_string()?);
            member.db_name_span = parser.span().expect(SPAN).into();
        } else {
            member.db_name = Some(ident.to_string());
            member.db_name_span = ident.span().into();
        };
        let _ = parser.expect_punctuation(Some(':'))?;

        // Parse type
        let ident = parser.expect_ident(None)?;
        member.variant_span = ident.span().into();
        let ident_name = ident.to_string();
        match ident_name.as_ref() {
            ARRAY => {
                let ident = parser.expect_ident(None)?;
                member.container = ContainerTemplate::Array;
                member.variant = Some(ident.to_string());
            }
            BTREEMAP => {
                let ident = parser.expect_ident(None)?;
                member.container = ContainerTemplate::BTreeMap(ident.to_string());
                let ident = parser.expect_ident(None)?;
                member.variant = Some(ident.to_string());
            }
            HASHMAP => {
                let ident = parser.expect_ident(None)?;
                member.container = ContainerTemplate::HashMap(ident.to_string());
                let ident = parser.expect_ident(None)?;
                member.variant = Some(ident.to_string());
            }
            _ => {
                member.variant = Some(ident_name);
            }
        }

        // Parse modifiers
        let punctuation = parser.expect_punctuation(None)?;
        if punctuation == '?' {
            member.is_optional = true;
            let _ = parser.expect_punctuation(Some(','))?;
        } else if punctuation == '+' {
            member.is_indexed = true;
            let _ = parser.expect_punctuation(Some(','))?;
        }

        // Finalize
        result.push(member);
        if parser.is_end() {
            break;
        }
    }
    return Ok(result);
}

fn parse_enum_or_union(parser: &mut Parser) -> Result<EntityTemplate, ()> {
    let name_ident = parser.expect_ident(None)?;
    let choices = parse_choices(parser.expect_group()?)?;

    let name = DefinedType::new(name_ident.to_string());
    if (choices.enum_choices.len() != 0) && (choices.union_choices.len() == 0) {
        return Ok(EntityTemplate::Enum(Enum::new(name, choices.enum_choices)));
    } else if (choices.enum_choices.len() == 0) && (choices.union_choices.len() != 0) {
        return Ok(EntityTemplate::Union(Union::new(name, choices.union_choices)));
    } else if (choices.enum_choices.len() == 0) && (choices.union_choices.len() == 0) {
        parser.span().expect(SPAN).error("The enum cannot be empty").emit();
        return Err(());
    } else {
        parser
            .span()
            .expect(SPAN)
            .error("The enum must have all entries with type or all entries without type")
            .emit();
        return Err(());
    }
}

fn parse_choices(group: proc_macro::Group) -> Result<ParseChoicesResult, ()> {
    let mut result = ParseChoicesResult::new();
    let mut parser = Parser::new(group.stream());
    loop {
        let rust_name = match parser.expect() {
            ExpectedTokenTree::Ident(name) => name,
            ExpectedTokenTree::EndOfStream => break,
            _ => {
                parser.span().expect(SPAN).error("Expected identifier").emit();
                return Err(());
            }
        };
        let _ = parser.expect_ident(Some("as"));
        let db_name = parser.expect_string()?;

        let mut next = parser.expect();
        if next == ExpectedTokenTree::Punct(':') {
            let variant = DefinedType::new(parser.expect_ident(None)?.to_string());
            let choice = UnionChoice::new(rust_name, db_name, variant);
            result.union_choices.push(choice);
            next = parser.expect();
        } else {
            let choice = EnumChoice::new(rust_name, db_name);
            result.enum_choices.push(choice);
        };

        match next {
            ExpectedTokenTree::Punct(',') => {}
            ExpectedTokenTree::EndOfStream => break,
            _ => {
                parser.span().expect(SPAN).error("Expected ',' or end of stream").emit();
                return Err(());
            }
        }
    }
    Ok(result)
}

pub fn parse_file_stream(stream: proc_macro::TokenStream) -> Result<Spec, ()> {
    let mut parser = Parser::new(stream);
    let name = parser.expect_string()?;
    parser.expect_eof()?;
    parse_file(name)
}

pub fn parse_file(name: String) -> Result<Spec, ()> {
    let mut path = std::path::PathBuf::new();
    path.push(std::env::var("CARGO_MANIFEST_DIR").expect("Read CARGO_MANIFEST_DIR variable"));
    path.push("huus");
    path.push(name);
    path.set_extension("huus.rs");

    let contents = std::fs::read_to_string(path.clone())
        .expect(&format!("Read file: {:?}", path));

    let stream = proc_macro::TokenStream::from_str(&contents).expect("Create token stream");
    parse_instruction_stream(stream)
}

pub fn parse_instruction_stream(stream: proc_macro::TokenStream) -> Result<Spec, ()> {
    let mut parser = Parser::new(stream);
    let mut spec = SpecBuilder::new();
    loop {
        match parser.expect() {
            ExpectedTokenTree::Ident(name) => {
                if name != "pub" {
                    parser.span().expect(SPAN).error("Expected ident 'pub'").emit();
                    return Err(());
                }
            }
            ExpectedTokenTree::EndOfStream => break,
            _ => {
                parser.span().expect(SPAN).error("Expected an ident or end of stream").emit();
                return Err(());
            }
        };

        let ident = parser.expect_ident(None)?;
        match ident.to_string().as_ref() {
            "struct" => spec.entities.push(parse_struct(&mut parser)?),
            "enum" => spec.entities.push(parse_enum_or_union(&mut parser)?),
            _ => {
                ident.span().error("Expected 'struct' or 'enum'").emit();
                return Err(());
            }
        }
    }
    if spec.entities.len() > 0 {
        Ok(spec.build()?)
    } else {
        proc_macro::Span::def_site().error("The macro seems to be empty").emit();
        Err(())
    }
}

