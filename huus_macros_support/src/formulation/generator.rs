// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Generation of the code for macros generating BSON queries.

use askama::Template;

use crate::definition::output::DefinedType;
use crate::formulation::output::{Object, Part, Value};

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

/// Template used for data query generation.
#[derive(Template)]
#[template(path = "data.rs", escape = "none")]
struct DataTemplate<'a> {
    pub name: &'a DefinedType,
    pub object: &'a Object,
    pub generator: &'a GeneratorCallback,
}

impl<'a> DataTemplate<'a> {
    /// Constructs a new `DataTemplate`.
    pub fn new(
        name: &'a DefinedType,
        object: &'a Object,
        generator: &'a GeneratorCallback,
    ) -> Self {
        Self { name, object, generator }
    }
}

// -------------------------------------------------------------------------------------------------

/// Template used for filter query generation.
#[derive(Template)]
#[template(path = "filter.rs", escape = "none")]
struct FilterTemplate<'a> {
    pub name: &'a DefinedType,
    pub object: &'a Object,
    pub generator: &'a GeneratorCallback,
}

impl<'a> FilterTemplate<'a> {
    /// Constructs a new `FilterTemplate`.
    pub fn new(
        name: &'a DefinedType,
        object: &'a Object,
        generator: &'a GeneratorCallback,
    ) -> Self {
        Self { name, object, generator }
    }
}

// -------------------------------------------------------------------------------------------------

/// Template used for update query generation.
#[derive(Template)]
#[template(path = "update.rs", escape = "none")]
struct UpdateTemplate<'a> {
    pub name: &'a DefinedType,
    pub object: &'a Object,
    pub generator: &'a GeneratorCallback,
}

impl<'a> UpdateTemplate<'a> {
    /// Constructs a new `UpdateTemplate`.
    pub fn new(
        name: &'a DefinedType,
        object: &'a Object,
        generator: &'a GeneratorCallback,
    ) -> Self {
        Self { name, object, generator }
    }
}

// -------------------------------------------------------------------------------------------------

/// Query generator.
pub struct Generator {
    name: DefinedType,
    object: Object,
}

impl Generator {
    /// Constructs a new `Generator`.
    pub fn new(name: DefinedType, object: Object) -> Self {
        Self { name, object }
    }

    /// Generates a data query.
    pub fn generate_data(self) -> proc_macro::TokenStream {
        let callback = GeneratorCallback::new();
        DataTemplate::new(&self.name, &self.object, &callback)
            .render()
            .expect("Render value template")
            .parse()
            .expect("Parse into TokenStream")
    }

    /// Generates a filter query.
    pub fn generate_filter(self) -> proc_macro::TokenStream {
        let callback = GeneratorCallback::new();
        FilterTemplate::new(&self.name, &self.object, &callback)
            .render()
            .expect("Render filter template")
            .parse()
            .expect("Parse into TokenStream")
    }

    /// Generates an update query.
    pub fn generate_update(self) -> proc_macro::TokenStream {
        let callback = GeneratorCallback::new();
        UpdateTemplate::new(&self.name, &self.object, &callback)
            .render()
            .expect("Render update template")
            .parse()
            .expect("Parse into TokenStream")
    }
}
