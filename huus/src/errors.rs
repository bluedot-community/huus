// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Errors specific to this crate.

#[derive(Debug)]
pub enum ConversionError {
    MissingKey { key: String },
    WrongType { key: String },
    UnexpectedValue { value: String },
    IncorrectValue { value: String },
}

impl ConversionError {
    pub fn missing_key(key: String) -> Self {
        ConversionError::MissingKey { key }
    }

    pub fn wrong_type(key: String) -> Self {
        ConversionError::WrongType { key }
    }

    pub fn wrong_type_for_unknown_key() -> Self {
        ConversionError::WrongType { key: "<unknown>".to_string() }
    }

    pub fn unexpected_value(value: String) -> Self {
        ConversionError::UnexpectedValue { value }
    }

    pub fn incorrect_value(value: String) -> Self {
        ConversionError::IncorrectValue { value }
    }
}

impl std::error::Error for ConversionError {}

impl std::fmt::Display for ConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ConversionError::MissingKey { key } => write!(f, "Missing key: '{}'", key),
            ConversionError::WrongType { key } => write!(f, "Wrong type for key: '{}'", key),
            ConversionError::UnexpectedValue { value } => {
                write!(f, "Unexpected value. Found: '{}'", value)
            }
            ConversionError::IncorrectValue { value } => write!(f, "Incorrect value: '{}'", value),
        }
    }
}

#[derive(Debug)]
pub enum HuusError {
    Mongo(mongo_driver::MongoError),
    Conversion(ConversionError),
}

impl std::error::Error for HuusError {}

impl From<mongo_driver::MongoError> for HuusError {
    fn from(error: mongo_driver::MongoError) -> Self {
        HuusError::Mongo(error)
    }
}

impl From<ConversionError> for HuusError {
    fn from(error: ConversionError) -> Self {
        HuusError::Conversion(error)
    }
}

impl std::fmt::Display for HuusError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            HuusError::Mongo(err) => write!(f, "MongoDB: {}", err),
            HuusError::Conversion(err) => write!(f, "Huus: {}", err),
        }
    }
}
