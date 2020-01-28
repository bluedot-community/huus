// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Parsing the token stream for macros defining the data types.

use std::{path::PathBuf, str::FromStr};

use crate::{
    definition::{input::*, validator::Validator},
    parser::{ExpectedTokenTree, Parser},
};

const SPAN: &str = "Span should be present";

// -------------------------------------------------------------------------------------------------

/// Parses the macro input. Returns parsed structure ready for verification.
pub struct Interpreter {
    entities: Vec<EntityTemplate>,
}

impl Interpreter {
    /// Constructs a new `Interpreter`.
    pub fn new() -> Self {
        Self { entities: Vec::new() }
    }

    /// Parses the schema definition.
    pub fn parse_instruction_stream(mut self, stream: proc_macro::TokenStream) -> Result<Self, ()> {
        let start_len = self.entities.len();

        let mut parser = Parser::new(stream);
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
                "struct" => self.entities.push(self.parse_struct(&mut parser)?),
                "enum" => self.entities.push(self.parse_enum_or_union(&mut parser)?),
                _ => {
                    ident.span().error("Expected 'struct' or 'enum'").emit();
                    return Err(());
                }
            }
        }

        if self.entities.len() > start_len {
            Ok(self)
        } else {
            proc_macro::Span::def_site().error("The specification seems to be empty").emit();
            Err(())
        }
    }

    /// Reads in and parses the schema file.
    pub fn parse_file(self, path: PathBuf) -> Result<Self, ()> {
        let contents =
            std::fs::read_to_string(path.clone()).expect(&format!("Read file: {:?}", path));

        let stream = proc_macro::TokenStream::from_str(&contents).expect("Create token stream");
        self.parse_instruction_stream(stream)
    }

    /// Parses out a file name, reads it in and parses as a schema definition.
    pub fn parse_file_stream(self, stream: proc_macro::TokenStream) -> Result<Self, ()> {
        let mut parser = Parser::new(stream);
        let name = parser.expect_string()?;
        parser.expect_eof()?;

        let mut path = PathBuf::new();
        path.push(std::env::var("CARGO_MANIFEST_DIR").expect("Read CARGO_MANIFEST_DIR variable"));
        path.push("huus");
        path.push(name);
        path.set_extension("huus.rs");

        self.parse_file(path)
    }

    /// Returns the validator for the parsed data.
    pub fn build(self) -> Validator {
        Validator::new(self.entities)
    }
}

// -------------------------------------------------------------------------------------------------
// Helper parse methods

impl Interpreter {
    /// Parses a single structure.
    fn parse_struct(&self, parser: &mut Parser) -> Result<EntityTemplate, ()> {
        let name_ident = parser.expect_ident(None)?;
        let (collection_name, collection_name_span) = if parser.is_ident() {
            let _ = parser.expect_ident(Some("in"))?;
            (Some(parser.expect_string()?), parser.span().expect(SPAN))
        } else {
            (None, proc_macro::Span::call_site())
        };
        let members = self.parse_members(parser.expect_group()?)?;

        let struct_name = name_ident.to_string();
        let struct_name_span = name_ident.span().into();
        Ok(EntityTemplate::Struct(StructTemplate {
            struct_name: struct_name,
            struct_name_span: struct_name_span,
            collection_name: collection_name,
            collection_name_span: collection_name_span,
            members: members,
        }))
    }

    /// Parses a list of members.
    fn parse_members(&self, group: proc_macro::Group) -> Result<Vec<MemberTemplate>, ()> {
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

    /// Parses an enum or an union. The difference between enum and union is that a union variants
    /// reference structures, while enum variants are to be interpreted as constant strings.
    fn parse_enum_or_union(&self, parser: &mut Parser) -> Result<EntityTemplate, ()> {
        let name_ident = parser.expect_ident(None)?;
        let name = name_ident.to_string();
        let name_span = parser.span().expect(SPAN).clone();
        let choices = self.parse_choices(parser.expect_group()?)?;

        if (choices.enum_choices.len() != 0) && (choices.union_choices.len() == 0) {
            let template = EnumTemplate::new(name, name_span, choices.enum_choices);
            return Ok(EntityTemplate::Enum(template));
        } else if (choices.enum_choices.len() == 0) && (choices.union_choices.len() != 0) {
            let template = UnionTemplate::new(name, name_span, choices.union_choices);
            return Ok(EntityTemplate::Union(template));
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

    /// Parse enum's or union's variants.
    fn parse_choices(&self, group: proc_macro::Group) -> Result<Choices, ()> {
        let mut result = Choices::new();
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
            if next.is_punct(':') {
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
}
