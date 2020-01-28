// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This crate provides an easy way to define `huus` data structures using macros.

#![warn(missing_docs)]
#![feature(proc_macro_def_site)]
#![feature(proc_macro_diagnostic)]
#![feature(proc_macro_span)]

extern crate proc_macro;

mod parser;

pub mod definition;
pub mod formulation;

pub use definition::interpreter::Interpreter as Definition;
pub use formulation::{interpreter::Interpreter as Formulation, validator::Problem};
