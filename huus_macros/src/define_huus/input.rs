// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Parsing the token stream of `define_huus` macro.

use crate::parser::{ExpectedTokenTree, Parser};

const SPAN: &str = "Span should be present";

#[derive(Clone, PartialEq)]
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
            _ => Err(()),
        }
    }

    fn allows_indexing(&self) -> bool {
        match self {
            BuiltInType::String => true,
            _ => false,
        }
    }
}

#[derive(Clone)]
pub struct PredefinedType {
    pub variant: BuiltInType,
    pub span: proc_macro2::Span,
}

impl PredefinedType {
    pub fn from_name(name: &str, span: proc_macro2::Span) -> Result<Self, ()> {
        match BuiltInType::from_name(name) {
            Ok(variant) => Ok(PredefinedType { variant, span }),
            Err(err) => Err(err),
        }
    }

    fn allows_indexing(&self) -> bool {
        self.variant.allows_indexing()
    }
}

impl PartialEq<PredefinedType> for PredefinedType {
    fn eq(&self, other: &PredefinedType) -> bool {
        self.variant == other.variant
    }
}

#[derive(Clone)]
pub struct DefinedType {
    pub name: String,
    pub span: proc_macro2::Span,
}

impl DefinedType {
    fn new(name: String, span: proc_macro2::Span) -> Self {
        Self { name, span }
    }
}

impl PartialEq<DefinedType> for DefinedType {
    fn eq(&self, other: &DefinedType) -> bool {
        self.name == other.name
    }
}

impl PartialEq<str> for DefinedType {
    fn eq(&self, name: &str) -> bool {
        self.name == name
    }
}

#[derive(Clone, PartialEq)]
pub enum Variant {
    Field(PredefinedType),
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

#[derive(Clone, PartialEq)]
pub enum Container {
    Array,
    BTreeMap(Variant),
    HashMap(Variant),
    Plain,
}

#[derive(Clone, PartialEq)]
pub enum ContainerTemplate {
    Array,
    BTreeMap(String),
    HashMap(String),
    Plain,
}

pub struct Member {
    pub rust_name: String,
    pub rust_name_span: proc_macro2::Span,
    pub db_name: String,
    pub db_name_span: proc_macro2::Span,
    pub variant: Variant,
    pub variant_span: proc_macro2::Span,
    pub is_optional: bool,
    pub is_indexed: bool,
    pub container: Container,
}

impl Member {
    fn validate(&self) -> Result<(), ()> {
        // Check if the name is allowed
        const FORBIDDEN_PREFIX: &'static str = "_huus";
        if self.rust_name.starts_with(FORBIDDEN_PREFIX) {
            self.rust_name_span
                .unwrap()
                .error(format!("Member name cannot start with '{}'", FORBIDDEN_PREFIX))
                .emit();
            return Err(());
        }

        // Check if the database name contains only allowed characters
        for character in self.db_name.chars() {
            if !character.is_alphanumeric() && character != '_' {
                let msg = format!(
                    "Field name can contain only alphanumerics and underscore, but '{}' was used",
                    character
                );
                self.db_name_span.unwrap().error(msg).emit();
                return Err(());
            }
        }

        // Check if indexing is requested only for types supporting indexing
        if self.is_indexed && !self.variant.allows_indexing() {
            self.variant_span.unwrap().error("Indexing not supported for this type").emit();
        }

        Ok(())
    }
}

#[derive(Clone)]
struct MemberTemplate {
    rust_name: Option<String>,
    rust_name_span: Option<proc_macro2::Span>,
    db_name: Option<String>,
    db_name_span: Option<proc_macro2::Span>,
    variant: Option<String>,
    variant_span: Option<proc_macro2::Span>,
    is_optional: bool,
    is_indexed: bool,
    container: ContainerTemplate,
}

impl MemberTemplate {
    pub fn new() -> Self {
        Self {
            rust_name: None,
            rust_name_span: None,
            db_name: None,
            db_name_span: None,
            variant: None,
            variant_span: None,
            is_optional: false,
            is_indexed: false,
            container: ContainerTemplate::Plain,
        }
    }
}

#[derive(Clone)]
pub struct EnumChoice {
    pub rust_name: String,
    pub rust_name_span: proc_macro2::Span,
    pub db_name: String,
    pub db_name_span: proc_macro2::Span,
}

impl EnumChoice {
    fn new(
        rust_name: String,
        rust_name_span: proc_macro2::Span,
        db_name: String,
        db_name_span: proc_macro2::Span,
    ) -> Self {
        Self { rust_name, rust_name_span, db_name: db_name, db_name_span }
    }
}

#[derive(Clone)]
pub struct UnionChoice {
    pub rust_name: String,
    pub rust_name_span: proc_macro2::Span,
    pub db_name: String,
    pub variant: DefinedType,
}

impl UnionChoice {
    fn new(
        rust_name: String,
        rust_name_span: proc_macro2::Span,
        db_name: String,
        variant: DefinedType,
    ) -> Self {
        Self { rust_name, rust_name_span, db_name, variant }
    }
}

pub struct Struct {
    pub struct_name: DefinedType,
    pub struct_name_span: proc_macro2::Span,
    pub collection_name: Option<String>,
    pub members: Vec<Member>,
}

#[derive(Clone)]
struct StructTemplate {
    pub struct_name: String,
    pub struct_name_span: proc_macro2::Span,
    pub collection_name: Option<String>,
    pub members: Vec<MemberTemplate>,
}

#[derive(Clone)]
pub struct Enum {
    pub name: DefinedType,
    pub choices: Vec<EnumChoice>,
}

impl Enum {
    fn new(name: DefinedType, choices: Vec<EnumChoice>) -> Self {
        Self { name, choices }
    }

    pub fn to_db_names(&self) -> Vec<String> {
        let mut result = Vec::with_capacity(self.choices.len());
        for choice in self.choices.iter() {
            result.push(choice.db_name.clone());
        }
        result
    }
}

#[derive(Clone)]
pub struct Union {
    pub name: DefinedType,
    pub choices: Vec<UnionChoice>,
}

impl Union {
    fn new(name: DefinedType, choices: Vec<UnionChoice>) -> Self {
        Self { name, choices }
    }
}

pub enum Entity {
    Struct(Struct),
    Enum(Enum),
    Union(Union),
}

enum EntityTemplate {
    Struct(StructTemplate),
    Enum(Enum),
    Union(Union),
}

pub struct Spec {
    pub entities: Vec<Entity>,
}

impl Spec {
    fn new() -> Self {
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
}

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

    fn make_variant(&self, string: String, span: proc_macro2::Span) -> Result<Variant, ()> {
        if let Ok(field) = PredefinedType::from_name(&string, span.clone()) {
            Ok(Variant::Field(field))
        } else if let Some(entity) = self.find_entity(&string) {
            match entity {
                EntityTemplate::Struct(..) => Ok(Variant::Struct(DefinedType::new(string, span))),
                EntityTemplate::Enum(..) => Ok(Variant::Enum(DefinedType::new(string, span))),
                EntityTemplate::Union(..) => Ok(Variant::Union(DefinedType::new(string, span))),
            }
        } else {
            span.unwrap()
                .error(format!("'{}' is neither predefined nor defined in this scope", string))
                .emit();
            Err(())
        }
    }

    fn make_container(
        &self,
        template: ContainerTemplate,
        span: proc_macro2::Span,
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
            let variant_span = template.variant_span.expect("Member type span incomplete");
            let member = Member {
                rust_name: template.rust_name.expect("Member name incomplete"),
                rust_name_span: template.rust_name_span.expect("Member name span incomplete"),
                db_name: template.db_name.expect("Database entry name incomplete"),
                db_name_span: template.db_name_span.expect("Database entry name span incomplete"),
                variant: self.make_variant(
                    template.variant.expect("Member type incomplete"),
                    variant_span.clone(),
                )?,
                variant_span: variant_span,
                is_optional: template.is_optional,
                is_indexed: template.is_indexed,
                container: self.make_container(template.container, variant_span.clone())?,
            };
            member.validate()?;
            members.push(member);
        }

        Ok(Struct {
            struct_name: DefinedType::new(
                struct_template.struct_name,
                struct_template.struct_name_span,
            ),
            struct_name_span: struct_template.struct_name_span,
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
                            member
                                .variant_span
                                .expect("Incomplete member type")
                                .unwrap()
                                .error("Only 'String' can be used as a key".to_string())
                                .emit();
                            Err(())
                        }
                    }
                } else {
                    match self.find_entity(&string) {
                        Some(EntityTemplate::Enum(..)) => Ok(()),
                        Some(_) => {
                            let msg = format!("Type '{}' is not an huus enum", string);
                            member
                                .variant_span
                                .expect("Incomplete member type")
                                .unwrap()
                                .error(msg)
                                .emit();
                            Err(())
                        }
                        None => {
                            let msg = format!("Type '{}' is neither not (pre)defined", string);
                            member
                                .variant_span
                                .expect("Incomplete member type")
                                .unwrap()
                                .error(msg)
                                .emit();
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
                EntityTemplate::Enum(enum_spec) => Entity::Enum(enum_spec.clone()),
                EntityTemplate::Union(union_spec) => {
                    // TODO: Validate if the union type is valid.
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

pub fn parse(stream: proc_macro::TokenStream) -> Result<Spec, ()> {
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
        member.rust_name_span = Some(ident.span().into());
        if parser.is_ident() {
            let _ = parser.expect_ident(Some("as"))?;
            member.db_name = Some(parser.expect_string()?);
            member.db_name_span = Some(parser.span().expect(SPAN).into());
        } else {
            member.db_name = Some(ident.to_string());
            member.db_name_span = Some(ident.span().into());
        };
        let _ = parser.expect_punctuation(Some(':'))?;

        // Parse type
        let ident = parser.expect_ident(None)?;
        member.variant_span = Some(ident.span().into());
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

    let name = DefinedType::new(name_ident.to_string(), name_ident.span().into());
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
        let rust_name_span = parser.span().expect(SPAN).into();
        let _ = parser.expect_ident(Some("as"));
        let db_name = parser.expect_string()?;
        let db_name_span = parser.span().expect(SPAN).into();

        let mut next = parser.expect();
        if next == ExpectedTokenTree::Punct(':') {
            let variant = DefinedType::new(
                parser.expect_ident(None)?.to_string(),
                parser.span().expect(SPAN).into(),
            );
            next = parser.expect();
            let choice = UnionChoice::new(rust_name, rust_name_span, db_name, variant);
            result.union_choices.push(choice);
        } else {
            let choice = EnumChoice::new(rust_name, rust_name_span, db_name, db_name_span);
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
