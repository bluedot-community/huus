// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Tests of `commands` module.

use bson::{bson, doc};

use huus::commands::*;

#[test]
fn test_create_indexes_command() {
    let collection = "collection".to_string();
    let fields = vec!["abc.def".to_string(), "ghi".to_string(), "jkl".to_string()];
    let command = CreateIndexesCommand::new(collection.clone(), fields);
    let expected = doc! {
        "createIndexes": collection.clone(),
        "indexes": [{
            "name": collection.clone(),
            "key": {
                "abc.def": "text",
                "ghi": "text",
                "jkl": "text",
            },
        }],
    };
    assert_eq!(command.get_command(), expected);
}
