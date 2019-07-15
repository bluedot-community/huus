// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Provides structures representing `mongodb` commands. They are the lowest level of abstraction
//! provided by this crate.

use std::marker::PhantomData;

use bson::{bson, doc};

use crate::conversions::FromDoc;
use crate::errors::HuusError;

// -------------------------------------------------------------------------------------------------

pub mod options {
    pub fn find(limit: u32) -> mongo_driver::CommandAndFindOptions {
        let mut options = mongo_driver::CommandAndFindOptions::default();
        options.limit = limit;
        options
    }

    pub fn update_many() -> mongo_driver::collection::UpdateOptions {
        let mut options = mongo_driver::collection::UpdateOptions::default();
        options.update_flags.add(mongo_driver::flags::UpdateFlag::MultiUpdate);
        options
    }

    pub fn remove_one() -> mongo_driver::collection::RemoveOptions {
        let mut options = mongo_driver::collection::RemoveOptions::default();
        options.remove_flags.add(mongo_driver::flags::RemoveFlag::SingleRemove);
        options
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, PartialEq)]
pub struct CreateCollectionCommand {
    pub(crate) collection_name: String,
}

impl CreateCollectionCommand {
    pub fn new(collection_name: String) -> Self {
        Self { collection_name }
    }

    pub fn execute(&self, db: &mongo_driver::database::Database) -> Result<(), HuusError> {
        if !db.has_collection(self.collection_name.clone())? {
            db.create_collection(self.collection_name.clone(), None)?;
        }
        Ok(())
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, PartialEq)]
pub struct DropCollectionCommand {
    pub(crate) collection_name: String,
}

impl DropCollectionCommand {
    pub fn new(collection_name: String) -> Self {
        Self { collection_name }
    }

    pub fn execute(&self, db: &mongo_driver::database::Database) -> Result<(), HuusError> {
        if db.has_collection(self.collection_name.clone())? {
            let mut collection = db.get_collection(self.collection_name.clone());
            collection.drop()?;
        }
        Ok(())
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, PartialEq)]
pub struct CreateIndexesCommand {
    pub(crate) collection_name: String,
    pub(crate) indexed_fields: Vec<String>,
}

impl CreateIndexesCommand {
    pub fn new(collection_name: String, indexed_fields: Vec<String>) -> Self {
        Self { collection_name, indexed_fields }
    }

    pub fn get_command(&self) -> bson::Document {
        let mut keys = bson::Document::new();
        for key in self.indexed_fields.iter() {
            keys.insert_bson(key.clone(), bson::Bson::String("text".to_string()));
        }

        doc! {
            "createIndexes": self.collection_name.clone(),
            "indexes": [{
                "name": self.collection_name.clone(),
                "key": keys,
            }],
        }
    }

    pub fn execute(&self, db: &mongo_driver::database::Database) -> Result<(), HuusError> {
        if self.indexed_fields.len() > 0 {
            db.command_simple(self.get_command(), None)?;
        }
        Ok(())
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, PartialEq)]
pub struct FindOneCommand<Data>
where
    Data: FromDoc,
{
    pub(crate) collection_name: String,
    pub(crate) filter: bson::Document,
    pub(crate) phantom: PhantomData<Data>,
}

impl<Data> FindOneCommand<Data>
where
    Data: FromDoc,
{
    pub fn new(collection_name: String, filter: bson::Document) -> Self {
        Self { collection_name, filter, phantom: PhantomData }
    }

    pub fn get_filter(&self) -> &bson::Document {
        &self.filter
    }

    pub fn execute(
        &self,
        db: &mongo_driver::database::Database,
    ) -> Result<Option<Data>, HuusError> {
        let collection = db.get_collection(self.collection_name.as_bytes());
        let filter = self.get_filter();
        let options = self.get_options();
        let response = collection.find(&filter, options.as_ref())?;
        for entry in response {
            return Ok(Some(Data::from_doc(entry?)?));
        }
        Ok(None)
    }

    fn get_options(&self) -> Option<mongo_driver::CommandAndFindOptions> {
        Some(options::find(1))
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, PartialEq)]
pub struct FindCommand<Data>
where
    Data: FromDoc,
{
    pub(crate) collection_name: String,
    pub(crate) filter: bson::Document,
    pub(crate) limit: Option<u32>,
    pub(crate) phantom: PhantomData<Data>,
}

impl<Data> FindCommand<Data>
where
    Data: FromDoc,
{
    pub fn new(collection_name: String, filter: bson::Document, limit: Option<u32>) -> Self {
        Self { collection_name, filter, limit, phantom: PhantomData }
    }

    pub fn get_filter(&self) -> &bson::Document {
        &self.filter
    }

    pub fn execute(&self, db: &mongo_driver::database::Database) -> Result<Vec<Data>, HuusError> {
        let collection = db.get_collection(self.collection_name.as_bytes());
        let filter = self.get_filter();
        let options = self.get_options();
        let response = collection.find(&filter, options.as_ref())?;
        let mut result = if let Some(limit) = self.limit {
            Vec::with_capacity(limit as usize)
        } else {
            Vec::new()
        };
        for entry in response {
            result.push(Data::from_doc(entry?)?);
        }
        Ok(result)
    }

    fn get_options(&self) -> Option<mongo_driver::CommandAndFindOptions> {
        if let Some(limit) = self.limit {
            Some(options::find(limit))
        } else {
            None
        }
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, PartialEq)]
pub struct InsertCommand {
    pub(crate) collection_name: String,
    pub(crate) document: bson::Document,
}

impl InsertCommand {
    pub fn new(collection_name: String, document: bson::Document) -> Self {
        Self { collection_name, document }
    }

    pub fn execute(
        &self,
        db: &mongo_driver::database::Database,
    ) -> Result<bson::oid::ObjectId, HuusError> {
        let collection = db.get_collection(self.collection_name.as_bytes());

        match self.document.get_object_id("_id") {
            Ok(id) => {
                collection.insert(&self.document, None)?;
                Ok(id.clone())
            }
            Err(..) => {
                let id = bson::oid::ObjectId::new().expect("Generate new ObjectId");
                let mut document = self.document.clone();
                document.insert("_id", id.clone());
                collection.insert(&document, None)?;
                Ok(id)
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, PartialEq)]
pub enum UpdateOptions {
    UpdateOne,
    UpdateMany,
}

#[derive(Debug, PartialEq)]
pub struct UpdateCommand {
    pub(crate) collection_name: String,
    pub(crate) filter: bson::Document,
    pub(crate) update: bson::Document,
    pub(crate) options: UpdateOptions,
}

impl UpdateCommand {
    pub fn new(
        collection_name: String,
        filter: bson::Document,
        update: bson::Document,
        options: UpdateOptions,
    ) -> Self {
        Self { collection_name, filter, update, options }
    }

    pub fn execute(&self, db: &mongo_driver::database::Database) -> Result<(), HuusError> {
        let collection = db.get_collection(self.collection_name.as_bytes());
        collection.update(&self.filter, &self.update, self.get_options().as_ref())?;
        Ok(())
    }

    fn get_options(&self) -> Option<mongo_driver::collection::UpdateOptions> {
        match self.options {
            UpdateOptions::UpdateOne => None,
            UpdateOptions::UpdateMany => Some(options::update_many()),
        }
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, PartialEq)]
pub enum RemoveOptions {
    RemoveOne,
    RemoveMany,
}

#[derive(Debug, PartialEq)]
pub struct RemoveCommand {
    pub(crate) collection_name: String,
    pub(crate) filter: bson::Document,
    pub(crate) options: RemoveOptions,
}

impl RemoveCommand {
    pub fn new(collection_name: String, filter: bson::Document, options: RemoveOptions) -> Self {
        Self { collection_name, filter, options }
    }

    pub fn execute(&self, db: &mongo_driver::database::Database) -> Result<(), HuusError> {
        let collection = db.get_collection(self.collection_name.as_bytes());
        collection.remove(&self.filter, self.get_options().as_ref())?;
        Ok(())
    }

    fn get_options(&self) -> Option<mongo_driver::collection::RemoveOptions> {
        match self.options {
            RemoveOptions::RemoveOne => Some(options::remove_one()),
            RemoveOptions::RemoveMany => None,
        }
    }
}
