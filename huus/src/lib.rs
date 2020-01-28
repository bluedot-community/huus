// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This crate provides a way to define `mongo` database structure as `rust` data structures. This
//! approach helps to find problems with database queries (like wrong field names or wrong query
//! structure) in compilation-time.

// TODO: Provide documentation of whole crate.
// #![warn(missing_docs)]

pub mod commands;
pub mod conversions;
pub mod errors;
pub mod filters;
pub mod query;
pub mod types;
pub mod updates;
pub mod values;

pub mod models {
    /// Prelude for defining new types.
    pub mod prelude {
        pub use crate::conversions::{HuusIntoBson, IntoDoc};
        pub use crate::filters::{BuildFilter, BuildInnerFilter, Filters};
        pub use crate::updates::{BuildInnerUpdate, BuildUpdate};
    }
}

pub mod prelude {
    pub use crate::conversions::{FromDoc, HuusKey, IntoDoc};
    pub use crate::filters::{ArrayFilter, ComparisonFilter, ElementFilter, ObjectFilter};
    pub use crate::query::Query;
    pub use crate::updates::{
        ArrayUpdate, DateUpdate, ElementUpdate, FieldUpdate, NumericalUpdate, ObjectUpdate,
        Operator,
    };
    pub use crate::values::{PullValue, PushValue};
}
