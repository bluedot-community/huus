// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Contains a trait representing all possible operations that may be performed on database.

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
        commands::CreateCollectionCommand::new(Self::get_collection_name().to_string())
    }

    fn drop_collection() -> commands::DropCollectionCommand {
        commands::DropCollectionCommand::new(Self::get_collection_name().to_string())
    }

    fn create_indexes() -> commands::CreateIndexesCommand {
        commands::CreateIndexesCommand::new(
            Self::get_collection_name().to_string(),
            Self::get_indexed_fields().iter().map(|f| f.to_string()).collect(),
        )
    }

    fn fetch_all() -> commands::FindCommand<Self::Data> {
        commands::FindCommand::new(Self::get_collection_name().to_string(), filter::all(), None)
    }

    fn find_one(self) -> commands::FindOneCommand<Self::Data>
    where
        Self: filters::BuildFilter,
    {
        commands::FindOneCommand::new(
            Self::get_collection_name().to_string(),
            self.build_filter().into_doc(),
        )
    }

    fn find(self) -> commands::FindCommand<Self::Data>
    where
        Self: filters::BuildFilter,
    {
        commands::FindCommand::new(
            Self::get_collection_name().to_string(),
            self.build_filter().into_doc(),
            None,
        )
    }

    fn text_search(pattern: String) -> commands::FindCommand<Self::Data> {
        commands::FindCommand::new(
            Self::get_collection_name().to_string(),
            filter::text(pattern),
            None,
        )
    }

    fn insert(self) -> commands::InsertCommand
    where
        Self: conversions::IntoDoc,
    {
        commands::InsertCommand::new(Self::get_collection_name().to_string(), self.into_doc())
    }

    fn update(self, update: Self::Update) -> commands::UpdateCommand
    where
        Self: filters::BuildFilter,
    {
        commands::UpdateCommand::new(
            Self::get_collection_name().to_string(),
            self.build_filter().into_doc(),
            update.build_update().into_doc(),
            commands::UpdateOptions::UpdateOne,
        )
    }

    fn update_many(self, update: Self::Update) -> commands::UpdateCommand
    where
        Self: filters::BuildFilter,
    {
        commands::UpdateCommand::new(
            Self::get_collection_name().to_string(),
            self.build_filter().into_doc(),
            update.build_update().into_doc(),
            commands::UpdateOptions::UpdateMany,
        )
    }

    fn remove_one(self) -> commands::RemoveCommand
    where
        Self: filters::BuildFilter,
    {
        commands::RemoveCommand::new(
            Self::get_collection_name().to_string(),
            self.build_filter().into_doc(),
            commands::RemoveOptions::RemoveOne,
        )
    }

    fn remove(self) -> commands::RemoveCommand
    where
        Self: filters::BuildFilter,
    {
        commands::RemoveCommand::new(
            Self::get_collection_name().to_string(),
            self.build_filter().into_doc(),
            commands::RemoveOptions::RemoveMany,
        )
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
