// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This crate provides an easy way to define `huus` data structures using macros.

extern crate proc_macro;

use huus_macros_support::{Definition, Formulation};

#[proc_macro]
pub fn define_huus(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let definition = Definition::new();
    if let Ok(interpreter) = definition.parse_instruction_stream(stream) {
        if let Ok(generator) = interpreter.build().verify() {
            return generator.generate_definition();
        }
    }
    proc_macro::TokenStream::new()
}

#[proc_macro]
pub fn define_huus_from(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let definition = Definition::new();
    if let Ok(interpreter) = definition.parse_file_stream(stream) {
        if let Ok(generator) = interpreter.build().verify() {
            return generator.generate_definition();
        }
    }
    proc_macro::TokenStream::new()
}

#[proc_macro]
pub fn define(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let definition = Definition::new();
    if let Ok(interpreter) = definition.parse_instruction_stream(stream) {
        if let Ok(generator) = interpreter.build().verify() {
            return generator.generate_formulation();
        }
    }
    proc_macro::TokenStream::new()
}

#[proc_macro]
pub fn define_from(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let definition = Definition::new();
    if let Ok(interpreter) = definition.parse_file_stream(stream) {
        if let Ok(generator) = interpreter.build().verify() {
            return generator.generate_formulation();
        }
    }
    proc_macro::TokenStream::new()
}

#[proc_macro]
pub fn data(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let formulation = Formulation::new(false);
    if let Ok(interpreter) = formulation.parse(stream) {
        if let Ok(generator) = interpreter.build().verify_data() {
            return generator.generate_data();
        }
    }
    "bson::Document::new()".parse().expect("Parse into TokenStream")
}

#[proc_macro]
pub fn filter(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let formulation = Formulation::new(false);
    if let Ok(interpreter) = formulation.parse(stream) {
        if let Ok(generator) = interpreter.build().verify_filter() {
            return generator.generate_filter();
        }
    }
    "bson::Document::new()".parse().expect("Parse into TokenStream")
}

#[proc_macro]
pub fn update(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let formulation = Formulation::new(false);
    if let Ok(interpreter) = formulation.parse(stream) {
        if let Ok(generator) = interpreter.build().verify_update() {
            return generator.generate_update();
        }
    }
    "bson::Document::new()".parse().expect("Parse into TokenStream")
}

#[cfg(feature = "testing")]
#[proc_macro]
pub fn data_testing(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let formulation = Formulation::new(true);
    if let Ok(interpreter) = formulation.parse(stream) {
        if let Err(verdict) = interpreter.build().verify_data() {
            return verdict.format().parse().expect("Parse into TokenStream");
        }
    }
    "Vec::<huus_macros_support::Problem>::new()".parse().expect("Parse into TokenStream")
}

#[cfg(feature = "testing")]
#[proc_macro]
pub fn filter_testing(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let formulation = Formulation::new(true);
    if let Ok(interpreter) = formulation.parse(stream) {
        if let Err(verdict) = interpreter.build().verify_filter() {
            return verdict.format().parse().expect("Parse into TokenStream");
        }
    }
    "Vec::<huus_macros_support::Problem>::new()".parse().expect("Parse into TokenStream")
}

#[cfg(feature = "testing")]
#[proc_macro]
pub fn update_testing(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let formulation = Formulation::new(true);
    if let Ok(interpreter) = formulation.parse(stream) {
        if let Err(verdict) = interpreter.build().verify_update() {
            return verdict.format().parse().expect("Parse into TokenStream");
        }
    }
    "Vec::<huus_macros_support::Problem>::new()".parse().expect("Parse into TokenStream")
}

