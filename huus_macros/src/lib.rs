// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This crate provides an easy way to define `huus` data structures using macros.

#![feature(proc_macro_diagnostic)]
#![feature(proc_macro_def_site)]
#![recursion_limit = "128"]

extern crate proc_macro;
extern crate proc_macro2;

mod definition_spec;
mod definition_input;
mod definition_output;
mod parser;

#[proc_macro]
pub fn define_huus(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    if let Ok(spec) = definition_input::parse_instruction_stream(stream) {
        definition_output::make_output(spec)
    } else {
        // No need to emit error here - it was already emitted
        proc_macro::TokenStream::new()
    }
}

#[proc_macro]
pub fn define_from(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    if let Ok(spec) = definition_input::parse_file_stream(stream) {
        definition_output::make_output(spec)
    } else {
        // No need to emit error here - it was already emitted
        proc_macro::TokenStream::new()
    }
}

