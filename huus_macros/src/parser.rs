// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Parser for macro token tree.

mod utils {
    pub fn strip_quotes(literal: &proc_macro::Literal) -> Result<String, ()> {
        let mut string = literal.to_string();
        if (string.pop() == Some('"'))
            && !string.is_empty()
            && (string.remove(0) == '"')
            && !string.is_empty()
        {
            Ok(string)
        } else {
            literal.span().error("Expected non-empty literal string").emit();
            Err(())
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ExpectedTokenTree {
    Group,
    Ident(String),
    Punct(char),
    String(String),
    EndOfStream,
    Error,
}

pub struct Parser {
    iter: proc_macro::token_stream::IntoIter,
    prev: Option<proc_macro::TokenTree>,
    current: Option<proc_macro::TokenTree>,
    next: Option<proc_macro::TokenTree>,
}

impl Parser {
    pub fn new(stream: proc_macro::TokenStream) -> Self {
        let mut iter = stream.into_iter();
        Self { prev: None, current: None, next: iter.next(), iter: iter }
    }

    fn start(&mut self) {
        self.current = self.next.take();
        self.next = self.iter.next();
    }

    fn finish(&mut self) {
        self.prev = self.current.take();
        self.current = None;
    }

    pub fn is_end(&self) -> bool {
        self.next.is_none()
    }

    pub fn is_ident(&self) -> bool {
        match &self.next {
            Some(proc_macro::TokenTree::Ident(_)) => true,
            Some(proc_macro::TokenTree::Group(_)) => false,
            Some(proc_macro::TokenTree::Punct(_)) => false,
            Some(proc_macro::TokenTree::Literal(_)) => false,
            None => false,
        }
    }

    pub fn span(&self) -> Option<proc_macro::Span> {
        match &self.prev {
            Some(proc_macro::TokenTree::Group(item)) => Some(item.span()),
            Some(proc_macro::TokenTree::Ident(item)) => Some(item.span()),
            Some(proc_macro::TokenTree::Punct(item)) => Some(item.span()),
            Some(proc_macro::TokenTree::Literal(item)) => Some(item.span()),
            None => None,
        }
    }

    pub fn expect_group(&mut self) -> Result<proc_macro::Group, ()> {
        self.start();
        let result = match &self.current {
            Some(proc_macro::TokenTree::Group(item)) => Ok(item.clone()),
            Some(proc_macro::TokenTree::Ident(item)) => {
                item.span().error("Expected a group, found an ident").emit();
                Err(())
            }
            Some(proc_macro::TokenTree::Punct(item)) => {
                item.span().error("Expected a group, found a punctuation").emit();
                Err(())
            }
            Some(proc_macro::TokenTree::Literal(item)) => {
                item.span().error("Expected a group, found a literal").emit();
                Err(())
            }
            None => {
                panic!("End of stream");
            }
        };
        self.finish();
        result
    }

    pub fn expect_ident(&mut self, expected: Option<&str>) -> Result<proc_macro::Ident, ()> {
        self.start();
        let result = match &self.current {
            Some(proc_macro::TokenTree::Ident(item)) => {
                let found = item.to_string();
                if let Some(expected) = expected {
                    if expected == found {
                        Ok(item.clone())
                    } else {
                        let msg = format!("Expected ident '{}', found '{}'", expected, found);
                        item.span().error(msg).emit();
                        Err(())
                    }
                } else {
                    Ok(item.clone())
                }
            }
            Some(proc_macro::TokenTree::Group(item)) => {
                item.span().error("Expected an ident, found a group").emit();
                Err(())
            }
            Some(proc_macro::TokenTree::Punct(item)) => {
                item.span().error("Expected an ident, found a punctuation").emit();
                Err(())
            }
            Some(proc_macro::TokenTree::Literal(item)) => {
                item.span().error("Expected an ident, found a literal").emit();
                Err(())
            }
            None => {
                panic!("End of stream");
            }
        };
        self.finish();
        result
    }

    pub fn expect_punctuation(&mut self, expected: Option<char>) -> Result<char, ()> {
        self.start();
        let result = match &self.current {
            Some(proc_macro::TokenTree::Punct(item)) => {
                let found = item.as_char();
                if let Some(expected) = expected {
                    if expected == found {
                        Ok(found)
                    } else {
                        let msg = format!("Expected punctuation '{}', found '{}'", expected, found);
                        item.span().error(msg).emit();
                        Err(())
                    }
                } else {
                    Ok(found)
                }
            }
            Some(proc_macro::TokenTree::Group(item)) => {
                item.span().error("Expected a punctuation, found a group").emit();
                Err(())
            }
            Some(proc_macro::TokenTree::Ident(item)) => {
                item.span().error("Expected a punctuation, found an ident").emit();
                Err(())
            }
            Some(proc_macro::TokenTree::Literal(item)) => {
                item.span().error("Expected a punctuation, found a literal").emit();
                Err(())
            }
            None => {
                panic!("End of stream");
            }
        };
        self.finish();
        result
    }

    pub fn expect_string(&mut self) -> Result<String, ()> {
        self.start();
        let result = match &self.current {
            Some(proc_macro::TokenTree::Literal(item)) => Ok(utils::strip_quotes(item)?),
            Some(proc_macro::TokenTree::Group(item)) => {
                item.span().error("Expected an literal, found a group").emit();
                Err(())
            }
            Some(proc_macro::TokenTree::Ident(item)) => {
                item.span().error("Expected a literal, found an ident").emit();
                Err(())
            }
            Some(proc_macro::TokenTree::Punct(item)) => {
                item.span().error("Expected an literal, found a punctuation").emit();
                Err(())
            }
            None => {
                panic!("End of stream");
            }
        };
        self.finish();
        result
    }

    pub fn expect_eof(&mut self) -> Result<(), ()> {
        self.start();
        let result = match &self.current {
            Some(proc_macro::TokenTree::Group(item)) => {
                item.span().error("Expected end of the macto, found a group").emit();
                Err(())
            }
            Some(proc_macro::TokenTree::Ident(item)) => {
                item.span().error("Expected end of the macro, found an ident").emit();
                Err(())
            }
            Some(proc_macro::TokenTree::Punct(item)) => {
                item.span().error("Expected end of the macro, found a punctuation").emit();
                Err(())
            }
            Some(proc_macro::TokenTree::Literal(item)) => {
                item.span().error("Expected end of the macto, found a literal").emit();
                Err(())
            }
            None => {
                Ok(())
            }
        };
        self.finish();
        result
    }

    pub fn expect(&mut self) -> ExpectedTokenTree {
        self.start();
        let result = match &self.current {
            Some(proc_macro::TokenTree::Group(..)) => ExpectedTokenTree::Group,
            Some(proc_macro::TokenTree::Ident(item)) => ExpectedTokenTree::Ident(item.to_string()),
            Some(proc_macro::TokenTree::Punct(item)) => ExpectedTokenTree::Punct(item.as_char()),
            Some(proc_macro::TokenTree::Literal(item)) => {
                if let Ok(string) = utils::strip_quotes(item) {
                    ExpectedTokenTree::String(string)
                } else {
                    ExpectedTokenTree::Error
                }
            }
            None => ExpectedTokenTree::EndOfStream,
        };
        self.finish();
        result
    }
}
