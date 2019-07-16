// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Tests of `commands` module.

use bson::{bson, doc};

use huus::commands::*;

#[test]
fn create_indexes_command() {
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
    assert_eq!(*command.get_command().unwrap(), expected);
}

/// When document passed to insert command contains `_id` the document should not be changed.
#[test]
fn create_insert_command_with_id() {
    let collection = "collection".to_string();
    let doc = doc! { "_id": 0, "a": 1, "b": 2 };
    let command = InsertCommand::new(collection.clone(), doc.clone());
    assert_eq!(*command.get_document(), doc);
}

/// When document passed to insert command does not contain `_id`, random `_id` should be added.
#[test]
fn create_insert_command_without_id() {
    let collection = "collection".to_string();
    let doc = doc! { "a": 1, "b": 2 };
    let command = InsertCommand::new(collection.clone(), doc);

    assert!(command.get_document().get("_id").is_some());
    assert_eq!(*command.get_document().get("a").unwrap(), bson::Bson::I32(1));
    assert_eq!(*command.get_document().get("b").unwrap(), bson::Bson::I32(2));
}
