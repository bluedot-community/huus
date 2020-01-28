// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Parsing the token stream for macros generating BSON queries.

use crate::{
    definition::data::SCHEMA,
    formulation::{input::*, validator::Validator},
    parser::{ExpectedTokenTree, Parser},
};

const SPAN: &str = "Span should be present";

// -------------------------------------------------------------------------------------------------

/// Parses the macro input. Returns parsed structure ready for verification.
pub struct Interpreter {
    collection: SpannedCollection,
    object: ObjectTemplate,
    testing: bool,
}

impl Interpreter {
    /// Constructs a new `Interpreter`.
    pub fn new(testing: bool) -> Self {
        Self {
            collection: SpannedCollection::new(),
            object: ObjectTemplate::new(proc_macro::Span::call_site()),
            testing: testing,
        }
    }

    /// Parses the macro input containing the query.
    pub fn parse(mut self, stream: proc_macro::TokenStream) -> Result<Self, ()> {
        let mut parser = Parser::new(stream);
        self.collection = self.parse_prelude(parser.expect_group()?)?;
        self.object = if parser.is_group() {
            let group = parser.expect_group()?;
            let next_parser = Parser::new(group.stream());
            self.parse_object(next_parser, group.span().clone())?
        } else {
            self.parse_object(parser, proc_macro::Span::call_site())?
        };
        Ok(self)
    }

    /// Returns the validator for the parsed data.
    pub fn build(self) -> Validator<'static> {
        Validator::new(self.collection, self.object, &*SCHEMA, self.testing)
    }
}

// -------------------------------------------------------------------------------------------------
// Helper parse methods

impl Interpreter {
    /// Parses the name of collection the data will refer to.
    fn parse_prelude(&self, group: proc_macro::Group) -> Result<SpannedCollection, ()> {
        let mut parser = Parser::new(group.stream());
        let collection =
            SpannedCollection { name: parser.expect_string()?, span: parser.span().expect(SPAN) };
        parser.expect_eof()?;
        Ok(collection)
    }

    /// Parse the code from code mode.
    fn parse_code(&self, group: proc_macro::Group) -> Result<String, ()> {
        Ok(group.stream().to_string())
    }

    /// Parses an object.
    fn parse_object(
        &self,
        mut parser: Parser,
        span: proc_macro::Span,
    ) -> Result<ObjectTemplate, ()> {
        let mut object = ObjectTemplate::new(span);

        while !parser.is_end() {
            // TODO: Allow also attributes provided without parentesis (idents separated by a
            // single dot).

            let attribute = self.parse_attribute(&mut parser)?;
            let value = self.parse_value(&mut parser)?;

            let value = SpannedValue::new(value, parser.span().expect(SPAN));
            let field = FieldTemplate::new(attribute, value);
            object.fields.push(field);

            if !parser.is_end() {
                let _ = parser.expect_punctuation(Some(','))?;
            }
        }
        Ok(object)
    }

    /// Parses an attribute.
    fn parse_attribute(&self, parser: &mut Parser) -> Result<SpannedAttribute, ()> {
        if parser.is_literal() {
            let attr =
                SpannedAttribute::from_str(&parser.expect_string()?, parser.span().expect(SPAN));
            let _ = parser.expect_punctuation(Some(':'))?;
            Ok(attr)
        } else {
            let mut attr = SpannedAttribute::new();
            loop {
                let part = match parser.expect() {
                    ExpectedTokenTree::Ident(ident) => {
                        SpannedPart::from_str(&ident.to_string(), parser.span().expect(SPAN))
                    }
                    ExpectedTokenTree::Group(group) => SpannedPart::from_code(
                        group.stream().to_string(),
                        parser.span().expect(SPAN),
                    ),
                    _ => {
                        parser
                            .span()
                            .expect(SPAN)
                            .error("Expected an identifier or parenthesis '()'")
                            .emit();
                        return Err(());
                    }
                };
                attr.push(part);

                match parser.expect_punctuation(None)? {
                    ':' => break,
                    '.' => {} // continue
                    _ => {
                        parser
                            .span()
                            .expect(SPAN)
                            .error("Expected a colon (':') or a dot ('.')")
                            .emit();
                        return Err(());
                    }
                }
            }
            Ok(attr)
        }
    }

    /// Parse a value.
    fn parse_value(&self, parser: &mut Parser) -> Result<ValueTemplate, ()> {
        match parser.expect() {
            ExpectedTokenTree::String(string) => Ok(ValueTemplate::Quoted(string)),
            ExpectedTokenTree::Value(string) => Ok(ValueTemplate::Unquoted(string)),
            ExpectedTokenTree::Ident(ident) => Ok(ValueTemplate::Unquoted(ident.to_string())),
            ExpectedTokenTree::Group(group) => match group.delimiter() {
                proc_macro::Delimiter::Parenthesis => {
                    Ok(ValueTemplate::Code(self.parse_code(group)?))
                }
                proc_macro::Delimiter::Brace => {
                    let next_parser = Parser::new(group.stream());
                    Ok(ValueTemplate::Object(self.parse_object(next_parser, group.span().clone())?))
                }
                _ => {
                    parser.span().expect(SPAN).error("Expected '()' or '{}' block").emit();
                    Err(())
                }
            },
            _ => {
                parser
                    .span()
                    .expect(SPAN)
                    .error("Expected a literal value or '()' or '{}' block")
                    .emit();
                Err(())
            }
        }
    }
}
