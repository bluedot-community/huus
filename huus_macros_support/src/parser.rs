// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Parser for macro token tree.

#[derive(Debug)]
pub enum ExpectedTokenTree {
    Group(proc_macro::Group),
    Ident(String),
    Punct(char),
    String(String),
    Value(String),
    EndOfStream,
}

impl ExpectedTokenTree {
    pub fn from_token_tree(token_tree: &Option<proc_macro::TokenTree>) -> Self {
        match token_tree {
            Some(proc_macro::TokenTree::Group(group)) => ExpectedTokenTree::Group(group.clone()),
            Some(proc_macro::TokenTree::Ident(item)) => ExpectedTokenTree::Ident(item.to_string()),
            Some(proc_macro::TokenTree::Punct(item)) => ExpectedTokenTree::Punct(item.as_char()),
            Some(proc_macro::TokenTree::Literal(item)) => ExpectedTokenTree::from_literal(item),
            None => ExpectedTokenTree::EndOfStream,
        }
    }

    pub fn from_literal(literal: &proc_macro::Literal) -> Self {
        let mut string = literal.to_string();
        if string.starts_with("\"") && string.ends_with("\"") {
            string.pop();
            string.remove(0);
            Self::String(string)
        } else {
            Self::Value(string)
        }
    }

    pub fn is_punct(&self, c1: char) -> bool {
        match self {
            Self::Punct(c2) => c1 == *c2,
            _ => false,
        }
    }
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

    pub fn is_group(&self) -> bool {
        match &self.next {
            Some(proc_macro::TokenTree::Ident(_)) => false,
            Some(proc_macro::TokenTree::Group(_)) => true,
            Some(proc_macro::TokenTree::Punct(_)) => false,
            Some(proc_macro::TokenTree::Literal(_)) => false,
            None => false,
        }
    }

    pub fn is_literal(&self) -> bool {
        match &self.next {
            Some(proc_macro::TokenTree::Ident(_)) => false,
            Some(proc_macro::TokenTree::Group(_)) => false,
            Some(proc_macro::TokenTree::Punct(_)) => false,
            Some(proc_macro::TokenTree::Literal(_)) => true,
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
                panic!("Expected a group, but the stream ended");
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
                panic!("Expected an ident, but the stream ended");
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
                panic!("Expected a punctuation, but the stream ended");
            }
        };
        self.finish();
        result
    }

    pub fn expect_string(&mut self) -> Result<String, ()> {
        self.start();
        let result = match &self.current {
            Some(proc_macro::TokenTree::Literal(item)) => {
                match ExpectedTokenTree::from_literal(item) {
                    ExpectedTokenTree::String(string) => Ok(string),
                    _ => {
                        item.span().error("Expected a literal string").emit();
                        Err(())
                    }
                }
            }
            Some(proc_macro::TokenTree::Group(item)) => {
                item.span().error("Expected a literal, found a group").emit();
                Err(())
            }
            Some(proc_macro::TokenTree::Ident(item)) => {
                item.span().error("Expected a literal, found an ident").emit();
                Err(())
            }
            Some(proc_macro::TokenTree::Punct(item)) => {
                item.span().error("Expected a literal, found a punctuation").emit();
                Err(())
            }
            None => {
                panic!("Expected a literal, but the stream ended");
            }
        };
        self.finish();
        result
    }

    pub fn expect_eof(&mut self) -> Result<(), ()> {
        self.start();
        let result = match &self.current {
            Some(proc_macro::TokenTree::Group(item)) => {
                item.span().error("Expected end of the macro, found a group").emit();
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
                item.span().error("Expected end of the macro, found a literal").emit();
                Err(())
            }
            None => Ok(()),
        };
        self.finish();
        result
    }

    pub fn expect(&mut self) -> ExpectedTokenTree {
        self.start();
        let result = ExpectedTokenTree::from_token_tree(&self.current);
        self.finish();
        result
    }
}
