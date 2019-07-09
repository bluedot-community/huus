// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Contains a trait representing all possible operations that may be performed on database.

use std::marker::PhantomData;

use crate::updates::BuildUpdate;
use crate::{commands, conversions, filters, updates};

pub mod filter {
    use bson::{bson, doc};

    pub fn all() -> bson::Document {
        doc! {}
    }

    pub fn text(pattern: String) -> bson::Document {
        doc! { "$text": { "$search": pattern } }
    }
}

pub trait Query: Sized {
    type Data: conversions::FromDoc;
    type Update: updates::BuildUpdate;

    fn get_collection_name() -> &'static str;
    fn get_indexed_fields() -> Vec<&'static str>;

    fn create_collection() -> commands::CreateCollectionCommand {
        commands::CreateCollectionCommand {
            collection_name: Self::get_collection_name().to_string(),
        }
    }

    fn drop_collection() -> commands::DropCollectionCommand {
        commands::DropCollectionCommand { collection_name: Self::get_collection_name().to_string() }
    }

    fn create_indexes() -> commands::CreateIndexesCommand {
        commands::CreateIndexesCommand {
            collection_name: Self::get_collection_name().to_string(),
            indexed_fields: Self::get_indexed_fields().iter().map(|f| f.to_string()).collect(),
        }
    }

    fn fetch_all() -> commands::FindCommand<Self::Data> {
        commands::FindCommand {
            collection_name: Self::get_collection_name().to_string(),
            filter: filter::all(),
            limit: None,
            phantom: PhantomData,
        }
    }

    fn find_one(self) -> commands::FindCommand<Self::Data>
    where
        Self: filters::BuildFilter,
    {
        commands::FindCommand {
            collection_name: Self::get_collection_name().to_string(),
            filter: self.build_filter().into_doc(),
            limit: Some(1),
            phantom: PhantomData,
        }
    }

    fn find(self) -> commands::FindCommand<Self::Data>
    where
        Self: filters::BuildFilter,
    {
        commands::FindCommand {
            collection_name: Self::get_collection_name().to_string(),
            filter: self.build_filter().into_doc(),
            limit: None,
            phantom: PhantomData,
        }
    }

    fn text_search(pattern: String) -> commands::FindCommand<Self::Data> {
        commands::FindCommand {
            collection_name: Self::get_collection_name().to_string(),
            filter: filter::text(pattern),
            limit: None,
            phantom: PhantomData,
        }
    }

    fn insert(self) -> commands::InsertCommand
    where
        Self: conversions::IntoDoc,
    {
        commands::InsertCommand {
            collection_name: Self::get_collection_name().to_string(),
            document: self.into_doc(),
        }
    }

    fn update(self, update: Self::Update) -> commands::UpdateCommand
    where
        Self: filters::BuildFilter,
    {
        commands::UpdateCommand {
            collection_name: Self::get_collection_name().to_string(),
            filter: self.build_filter().into_doc(),
            update: update.build_update().into_doc(),
            options: commands::UpdateOptions::UpdateOne,
        }
    }

    fn update_many(self, update: Self::Update) -> commands::UpdateCommand
    where
        Self: filters::BuildFilter,
    {
        commands::UpdateCommand {
            collection_name: Self::get_collection_name().to_string(),
            filter: self.build_filter().into_doc(),
            update: update.build_update().into_doc(),
            options: commands::UpdateOptions::UpdateMany,
        }
    }

    fn remove_one(self) -> commands::RemoveCommand
    where
        Self: filters::BuildFilter,
    {
        commands::RemoveCommand {
            collection_name: Self::get_collection_name().to_string(),
            filter: self.build_filter().into_doc(),
            options: commands::RemoveOptions::RemoveOne,
        }
    }

    fn remove(self) -> commands::RemoveCommand
    where
        Self: filters::BuildFilter,
    {
        commands::RemoveCommand {
            collection_name: Self::get_collection_name().to_string(),
            filter: self.build_filter().into_doc(),
            options: commands::RemoveOptions::RemoveMany,
        }
    }
}

impl<F> Query for filters::Filters<F>
where
    F: filters::BuildFilter + Query,
{
    type Data = <F as Query>::Data;
    type Update = <F as Query>::Update;

    fn get_collection_name() -> &'static str {
        <F as Query>::get_collection_name()
    }

    fn get_indexed_fields() -> Vec<&'static str> {
        <F as Query>::get_indexed_fields()
    }
}
