// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Generation of the code for macros generating BSON queries.

use askama::Template;

use crate::definition::output::DefinedType;
use crate::formulation::output::{Cast, Container, Object, Value, Part};

// -------------------------------------------------------------------------------------------------

impl Cast {
    fn to_data(&self) -> String {
        let variant = self.variant.to_data();
        match &self.container {
            Container::Array => format!("Vec<{}>", variant),
            Container::HashMap(key_variant) => {
                let key = key_variant.to_data();
                format!("std::collections::HashMap<{}, {}>", key, variant)
            }
            Container::BTreeMap(key_variant) => {
                let key = key_variant.to_data();
                format!("std::collections::BTreeMap<{}, {}>", key, variant)
            }
            Container::Plain => variant,
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Helper structure for calling rust code from within a template.
#[derive(Clone)]
struct GeneratorCallback;

impl GeneratorCallback {
    /// Constructs a new `GeneratorCallback`.
    pub fn new() -> Self {
        Self
    }

    /// Renders the object template with the give object.
    pub fn object(&self, object: &Object) -> String {
        ObjectTemplate::new(object, self).render().expect("Render object template")
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Template)]
#[template(path = "object.rs", escape = "none")]
struct ObjectTemplate<'a> {
    pub object: &'a Object,
    pub generator: &'a GeneratorCallback,
}

impl<'a> ObjectTemplate<'a> {
    pub fn new(object: &'a Object, generator: &'a GeneratorCallback) -> Self {
        Self { object, generator }
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Template)]
#[template(path = "data.rs", escape = "none")]
struct DataTemplate<'a> {
    pub name: &'a DefinedType,
    pub object: &'a Object,
    pub generator: &'a GeneratorCallback,
}

impl<'a> DataTemplate<'a> {
    pub fn new(name: &'a DefinedType, object: &'a Object, generator: &'a GeneratorCallback) -> Self {
        Self { name, object, generator }
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Template)]
#[template(path = "filter.rs", escape = "none")]
struct FilterTemplate<'a> {
    pub name: &'a DefinedType,
    pub object: &'a Object,
    pub generator: &'a GeneratorCallback,
}

impl<'a> FilterTemplate<'a> {
    pub fn new(name: &'a DefinedType, object: &'a Object, generator: &'a GeneratorCallback) -> Self {
        Self { name, object, generator }
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Template)]
#[template(path = "update.rs", escape = "none")]
struct UpdateTemplate<'a> {
    pub name: &'a DefinedType,
    pub object: &'a Object,
    pub generator: &'a GeneratorCallback,
}

impl<'a> UpdateTemplate<'a> {
    pub fn new(name: &'a DefinedType, object: &'a Object, generator: &'a GeneratorCallback) -> Self {
        Self { name, object, generator }
    }
}

// -------------------------------------------------------------------------------------------------

pub struct Generator {
    name: DefinedType,
    object: Object,
}

impl Generator {
    pub fn new(name: DefinedType, object: Object) -> Self {
        Self { name, object }
    }

    pub fn generate_data(self) -> proc_macro::TokenStream {
        let callback = GeneratorCallback::new();
        DataTemplate::new(&self.name, &self.object, &callback)
            .render()
            .expect("Render value template")
            .parse()
            .expect("Parse into TokenStream")
    }

    pub fn generate_filter(self) -> proc_macro::TokenStream {
        let callback = GeneratorCallback::new();
        FilterTemplate::new(&self.name, &self.object, &callback)
            .render()
            .expect("Render filter template")
            .parse()
            .expect("Parse into TokenStream")
    }

    pub fn generate_update(self) -> proc_macro::TokenStream {
        let callback = GeneratorCallback::new();
        UpdateTemplate::new(&self.name, &self.object, &callback)
            .render()
            .expect("Render update template")
            .parse()
            .expect("Parse into TokenStream")
    }
}
