// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Definitions of database data types.

use crate::definition::{interpreter::Interpreter, output::Schema};

lazy_static::lazy_static! {
    pub static ref SCHEMA: Schema = {
        let mut interpreter = Interpreter::new();

        let mut dir = std::path::PathBuf::new();
        dir.push(std::env::var("CARGO_MANIFEST_DIR").expect("Read CARGO_MANIFEST_DIR variable"));
        dir.push("huus");

        if dir.is_dir() {
            for entry in std::fs::read_dir(dir).expect("Failed to read `huus` directory") {
                let path = entry.expect("Failed to read a `huus` directory entry").path();
                let string = path.to_str().expect("Path is not UTF-8");
                if path.is_file() && string.ends_with(".huus.rs") {
                    interpreter = interpreter.parse_file(path)
                        .expect("Stopping `huus` compilation due to previous error");
                }
            }
        }

        interpreter.build()
            .verify()
            .expect("Stopping `huus` validation due to previous error")
            .into_schema()
    };
}
