// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Contains a trait representing all possible operations that may be performed on database.

use crate::conversions::IntoDoc;
use crate::{commands, conversions, filters};

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
    type Data: conversions::FromDoc + conversions::IntoDoc;
    type Insert: conversions::IntoDoc;
    type Filter: conversions::IntoDoc;
    type Update: conversions::IntoDoc;

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

    fn find_one(filter: Self::Filter) -> commands::FindOneCommand<Self::Data> {
        commands::FindOneCommand::new(Self::get_collection_name().to_string(), filter.into_doc())
    }

    fn find(filter: Self::Filter) -> commands::FindCommand<Self::Data> {
        commands::FindCommand::new(Self::get_collection_name().to_string(), filter.into_doc(), None)
    }

    // TODO: Provide a better way for defining logical oprations
    fn find_logical(filters: filters::Filters<Self::Filter>) -> commands::FindCommand<Self::Data> {
        commands::FindCommand::new(
            Self::get_collection_name().to_string(),
            filters.into_doc(),
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

    fn insert(data: Self::Insert) -> commands::InsertCommand {
        commands::InsertCommand::new(Self::get_collection_name().to_string(), data.into_doc())
    }

    fn insert_data(data: Self::Data) -> commands::InsertCommand {
        commands::InsertCommand::new(Self::get_collection_name().to_string(), data.into_doc())
    }

    fn update(filter: Self::Filter, update: Self::Update) -> commands::UpdateCommand {
        commands::UpdateCommand::new(
            Self::get_collection_name().to_string(),
            filter.into_doc(),
            update.into_doc(),
            commands::UpdateOptions::UpdateOne,
        )
    }

    fn update_many(filter: Self::Filter, update: Self::Update) -> commands::UpdateCommand {
        commands::UpdateCommand::new(
            Self::get_collection_name().to_string(),
            filter.into_doc(),
            update.into_doc(),
            commands::UpdateOptions::UpdateMany,
        )
    }

    fn remove_one(filter: Self::Filter) -> commands::RemoveCommand {
        commands::RemoveCommand::new(
            Self::get_collection_name().to_string(),
            filter.into_doc(),
            commands::RemoveOptions::RemoveOne,
        )
    }

    fn remove(filter: Self::Filter) -> commands::RemoveCommand {
        commands::RemoveCommand::new(
            Self::get_collection_name().to_string(),
            filter.into_doc(),
            commands::RemoveOptions::RemoveMany,
        )
    }
}
