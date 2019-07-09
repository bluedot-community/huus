// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This crate provides an easy way to define `huus` data structures using macros.

#![feature(proc_macro_diagnostic)]
#![feature(proc_macro_def_site)]
#![recursion_limit = "128"]

extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;

mod define_huus;
mod parser;

#[proc_macro]
pub fn define_huus(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    if let Ok(spec) = define_huus::input::parse(stream) {
        define_huus::output::make_output(spec)
    } else {
        // No need to emit error here - it was already emitted
        quote::quote!().into()
    }
}
