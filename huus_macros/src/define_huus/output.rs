// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Generation of the code for `define_huus` macro.

use std::collections::HashMap;

use super::input::{
    BuiltInType, Container, DefinedType, Entity, Enum, EnumChoice, Member, PredefinedType, Spec,
    Struct, Union, UnionChoice, Variant,
};

const BUILD_FILTER_TRAIT: &str = "BuildFilter";
const BUILD_UPDATE_TRAIT: &str = "BuildUpdate";
const BUILD_INNER_FILTER_TRAIT: &str = "BuildInnerFilter";
const BUILD_INNER_UPDATE_TRAIT: &str = "BuildInnerUpdate";

const UNION_VARIANT_KEY: &str = "_huus_variant";

impl PredefinedType {
    pub fn to_data(&self) -> proc_macro2::TokenStream {
        let span = self.span.clone();
        let output = match self.variant {
            BuiltInType::F64 => quote::quote_spanned!(span=> f64),
            BuiltInType::String => quote::quote_spanned!(span=> String),
            BuiltInType::ObjectId => quote::quote_spanned!(span=> huus::types::ObjectId),
            BuiltInType::Bool => quote::quote_spanned!(span=> bool),
            BuiltInType::Date => quote::quote_spanned!(span=> huus::types::Date),
            BuiltInType::I32 => quote::quote_spanned!(span=> i32),
            BuiltInType::I64 => quote::quote_spanned!(span=> i64),
            BuiltInType::Bson => quote::quote_spanned!(span=> bson::Document),
        };
        output.into()
    }

    pub fn to_filter(&self) -> proc_macro2::TokenStream {
        let span = self.span.clone();
        let output = match self.variant {
            BuiltInType::F64 => quote::quote_spanned!(span=> huus::filters::F64Entry),
            BuiltInType::String => quote::quote_spanned!(span=> huus::filters::StringEntry),
            BuiltInType::ObjectId => quote::quote_spanned!(span=> huus::filters::ObjectIdEntry),
            BuiltInType::Bool => quote::quote_spanned!(span=> huus::filters::BooleanEntry),
            BuiltInType::Date => quote::quote_spanned!(span=> huus::filters::DateEntry),
            BuiltInType::I32 => quote::quote_spanned!(span=> huus::filters::I32Entry),
            BuiltInType::I64 => quote::quote_spanned!(span=> huus::filters::I64Entry),
            BuiltInType::Bson => quote::quote_spanned!(span=> huus::filters::BsonEntry),
        };
        output.into()
    }

    pub fn to_value(&self) -> proc_macro2::TokenStream {
        let span = self.span.clone();
        let output = match self.variant {
            BuiltInType::F64 => quote::quote_spanned!(span=> f64),
            BuiltInType::String => quote::quote_spanned!(span=> String),
            BuiltInType::ObjectId => quote::quote_spanned!(span=> huus::types::ObjectId),
            BuiltInType::Bool => quote::quote_spanned!(span=> bool),
            BuiltInType::Date => quote::quote_spanned!(span=> huus::types::Date),
            BuiltInType::I32 => quote::quote_spanned!(span=> i32),
            BuiltInType::I64 => quote::quote_spanned!(span=> i64),
            BuiltInType::Bson => quote::quote_spanned!(span=> bson::Document),
        };
        output.into()
    }

    pub fn to_update(&self) -> proc_macro2::TokenStream {
        let span = self.span.clone();
        let output = match self.variant {
            BuiltInType::F64 => quote::quote_spanned!(span=> huus::updates::F64Entry),
            BuiltInType::String => quote::quote_spanned!(span=> huus::updates::StringEntry),
            BuiltInType::ObjectId => quote::quote_spanned!(span=> huus::updates::ObjectIdEntry),
            BuiltInType::Bool => quote::quote_spanned!(span=> huus::updates::BooleanEntry),
            BuiltInType::Date => quote::quote_spanned!(span=> huus::updates::DateEntry),
            BuiltInType::I32 => quote::quote_spanned!(span=> huus::updates::I32Entry),
            BuiltInType::I64 => quote::quote_spanned!(span=> huus::updates::I64Entry),
            BuiltInType::Bson => quote::quote_spanned!(span=> huus::updates::BsonEntry),
        };
        output.into()
    }

    pub fn from_doc_getter(&self) -> proc_macro2::TokenStream {
        let output = match self.variant {
            BuiltInType::F64 => quote::quote! { get_f64 },
            BuiltInType::String => quote::quote! { get_str },
            BuiltInType::ObjectId => quote::quote! { get_object_id },
            BuiltInType::Bool => quote::quote! { get_bool },
            BuiltInType::Date => quote::quote! { get_utc_datetime },
            BuiltInType::I32 => quote::quote! { get_i32 },
            BuiltInType::I64 => quote::quote! { get_i64 },
            BuiltInType::Bson => quote::quote! { get_document },
        };
        output.into()
    }

    pub fn to_conversion(&self) -> proc_macro2::TokenStream {
        let output = match self.variant {
            BuiltInType::F64 => quote::quote! { value },
            BuiltInType::String => quote::quote! { value.to_string() },
            BuiltInType::ObjectId => quote::quote! { value.clone() },
            BuiltInType::Bool => quote::quote! { value },
            BuiltInType::Date => quote::quote! { value.clone() },
            BuiltInType::I32 => quote::quote! { value },
            BuiltInType::I64 => quote::quote! { value },
            BuiltInType::Bson => quote::quote! { value.clone() },
        };
        output.into()
    }
}

impl DefinedType {
    const DATA_POSTFIX: &'static str = "Data";
    const FILTER_POSTFIX: &'static str = "Filter";
    const VALUE_POSTFIX: &'static str = "Value";
    const UPDATE_POSTFIX: &'static str = "Update";

    fn to_data(&self) -> proc_macro2::Ident {
        proc_macro2::Ident::new(&(self.name.clone() + Self::DATA_POSTFIX), self.span)
    }

    fn to_filter(&self) -> proc_macro2::Ident {
        proc_macro2::Ident::new(&(self.name.clone() + Self::FILTER_POSTFIX), self.span)
    }

    fn to_value(&self) -> proc_macro2::Ident {
        proc_macro2::Ident::new(&(self.name.clone() + Self::VALUE_POSTFIX), self.span)
    }

    fn to_update(&self) -> proc_macro2::Ident {
        proc_macro2::Ident::new(&(self.name.clone() + Self::UPDATE_POSTFIX), self.span)
    }
}

impl Variant {
    pub fn to_data(&self, span: proc_macro2::Span) -> proc_macro2::TokenStream {
        let output = match self {
            Variant::Field(field) => field.to_data(),
            Variant::Struct(name) => {
                let ident = name.to_data();
                quote::quote_spanned!(span=> #ident)
            }
            Variant::Enum(name) => {
                let ident = name.to_data();
                quote::quote_spanned!(span=> #ident)
            }
            Variant::Union(name) => {
                let ident = name.to_data();
                quote::quote_spanned!(span=> #ident)
            }
        };
        output.into()
    }

    pub fn to_long_filter(&self, span: proc_macro2::Span) -> proc_macro2::TokenStream {
        let output = match self {
            Variant::Field(field) => field.to_filter(),
            Variant::Struct(name) => {
                let filter_type = name.to_filter();
                let data_type = name.to_data();
                quote::quote_spanned!(span=> huus::filters::ObjectEntry<#filter_type, #data_type>)
            }
            Variant::Enum(name) => {
                let ident = name.to_data();
                quote::quote_spanned!(span=> huus::filters::EnumEntry<#ident>)
            }
            Variant::Union(name) => {
                let filter_type = name.to_filter();
                let data_type = name.to_data();
                quote::quote_spanned!(span=> huus::filters::ObjectEntry<#filter_type, #data_type>)
            }
        };
        output.into()
    }

    pub fn to_short_filter(&self, span: proc_macro2::Span) -> proc_macro2::TokenStream {
        let output = match self {
            Variant::Field(field) => field.to_filter(),
            Variant::Struct(name) => {
                let filter_type = name.to_filter();
                quote::quote_spanned!(span=> #filter_type)
            }
            Variant::Enum(name) => {
                let ident = name.to_data();
                quote::quote_spanned!(span=> #ident)
            }
            Variant::Union(name) => {
                let filter_type = name.to_filter();
                quote::quote_spanned!(span=> #filter_type)
            }
        };
        output.into()
    }

    pub fn to_value(&self, span: proc_macro2::Span) -> proc_macro2::TokenStream {
        let output = match self {
            Variant::Field(field) => field.to_value(),
            Variant::Struct(name) => {
                let ident = name.to_value();
                quote::quote_spanned!(span=> #ident)
            }
            Variant::Enum(name) => {
                let ident = name.to_value();
                quote::quote_spanned!(span=> #ident)
            }
            Variant::Union(name) => {
                let ident = name.to_value();
                quote::quote_spanned!(span=> #ident)
            }
        };
        output.into()
    }

    pub fn to_update(&self, span: proc_macro2::Span) -> proc_macro2::TokenStream {
        let output = match self {
            Variant::Field(field) => field.to_update(),
            Variant::Struct(name) => {
                let update_type = name.to_update();
                let value_type = name.to_value();
                quote::quote_spanned!(span=> huus::updates::ObjectEntry<#update_type, #value_type>)
            }
            Variant::Enum(name) => {
                let ident = name.to_value();
                quote::quote_spanned!(span=> huus::updates::EnumEntry<#ident>)
            }
            Variant::Union(name) => {
                let update_type = name.to_update();
                let value_type = name.to_value();
                quote::quote_spanned!(span=> huus::updates::ObjectEntry<#update_type, #value_type>)
            }
        };
        output.into()
    }

    pub fn from_doc_getter(&self) -> proc_macro2::TokenStream {
        let output = match self {
            Variant::Field(field) => field.from_doc_getter(),
            Variant::Struct(_) => quote::quote! { get_document },
            Variant::Enum(_) => quote::quote! { get_str },
            Variant::Union(_) => quote::quote! { get_document },
        };
        output.into()
    }

    pub fn to_conversion(&self, span: proc_macro2::Span) -> proc_macro2::TokenStream {
        let output = match self {
            Variant::Field(field) => field.to_conversion(),
            Variant::Struct(name) => {
                let ident = name.to_data();
                quote::quote! { #ident::from_doc(value.clone())? }
            }
            Variant::Enum(name) => {
                let ident = name.to_data();
                quote::quote_spanned!(span=> #ident::from_str(&value)?)
            }
            Variant::Union(name) => {
                let ident = name.to_data();
                quote::quote! { #ident::from_doc(value.clone())? }
            }
        };
        output.into()
    }
}

impl Member {
    fn to_data(&self) -> proc_macro2::TokenStream {
        let variant = self.variant.to_data(self.variant_span);
        match &self.container {
            Container::Array => quote::quote! { Vec<#variant> },
            Container::HashMap(key_variant) => {
                let key_ident = key_variant.to_data(self.variant_span);
                quote::quote! { std::collections::HashMap<#key_ident, #variant> }
            }
            Container::BTreeMap(key_variant) => {
                let key_ident = key_variant.to_data(self.variant_span);
                quote::quote! { std::collections::BTreeMap<#key_ident, #variant> }
            }
            Container::Plain => variant,
        }
    }

    fn to_filter(&self) -> proc_macro2::TokenStream {
        match &self.container {
            Container::Array => {
                let data_type = self.variant.to_data(self.variant_span);
                let filter_type = self.variant.to_short_filter(self.variant_span);
                quote::quote! { huus::filters::ArrayEntry<#filter_type, #data_type> }
            }
            Container::BTreeMap(key_variant) => {
                let key_ident = key_variant.to_data(self.variant_span);
                let data_type = self.variant.to_data(self.variant_span);
                quote::quote! { huus::filters::BTreeMapEntry<#key_ident, #data_type> }
            }
            Container::HashMap(key_variant) => {
                let key_ident = key_variant.to_data(self.variant_span);
                let data_type = self.variant.to_data(self.variant_span);
                quote::quote! { huus::filters::HashMapEntry<#key_ident, #data_type> }
            }
            Container::Plain => self.variant.to_long_filter(self.variant_span),
        }
    }

    fn to_value(&self) -> proc_macro2::TokenStream {
        match &self.container {
            Container::Array => {
                let variant = self.variant.to_data(self.variant_span);
                quote::quote! { huus::values::Array<#variant> }
            }
            Container::HashMap(key_variant) => {
                let key_ident = key_variant.to_value(self.variant_span);
                let variant = self.variant.to_data(self.variant_span);
                quote::quote! { std::collections::HashMap<#key_ident, #variant> }
            }
            Container::BTreeMap(key_variant) => {
                let key_ident = key_variant.to_value(self.variant_span);
                let variant = self.variant.to_data(self.variant_span);
                quote::quote! { std::collections::BTreeMap<#key_ident, #variant> }
            }
            Container::Plain => self.variant.to_value(self.variant_span),
        }
    }

    fn to_update(&self) -> proc_macro2::TokenStream {
        match &self.container {
            Container::Array => {
                let variant = self.variant.to_value(self.variant_span);
                quote::quote! { huus::updates::ArrayEntry<#variant> }
            }
            Container::BTreeMap(key_variant) => {
                let key_ident = key_variant.to_data(self.variant_span);
                let variant = self.variant.to_data(self.variant_span);
                quote::quote! { huus::updates::BTreeMapEntry<#key_ident, #variant> }
            }
            Container::HashMap(key_variant) => {
                let key_ident = key_variant.to_data(self.variant_span);
                let variant = self.variant.to_data(self.variant_span);
                quote::quote! { huus::updates::HashMapEntry<#key_ident, #variant> }
            }
            Container::Plain => self.variant.to_update(self.variant_span),
        }
    }

    pub fn make_struct_member_code(&self) -> proc_macro2::TokenStream {
        let rust_name = proc_macro2::Ident::new(&self.rust_name, self.rust_name_span);
        let type_name = if self.is_optional {
            let variant = self.to_data();
            quote::quote! { Option<#variant> }
        } else {
            self.to_data()
        };
        quote::quote! { pub #rust_name: #type_name, }
    }

    pub fn make_filter_member_code(&self) -> proc_macro2::TokenStream {
        let rust_name = proc_macro2::Ident::new(&self.rust_name, self.rust_name_span);
        let type_name = self.to_filter();
        quote::quote! { pub #rust_name: #type_name, }
    }

    pub fn make_value_member_code(&self) -> proc_macro2::TokenStream {
        let rust_name = proc_macro2::Ident::new(&self.rust_name, self.rust_name_span);
        let type_name = self.to_value();
        quote::quote! { pub #rust_name: Option<#type_name>, }
    }

    pub fn make_update_member_code(&self) -> proc_macro2::TokenStream {
        let rust_name = proc_macro2::Ident::new(&self.rust_name, self.rust_name_span);
        let type_name = self.to_update();
        quote::quote! { pub #rust_name: #type_name, }
    }

    pub fn make_from_doc_code(&self) -> proc_macro2::TokenStream {
        let rust_name = proc_macro2::Ident::new(&self.rust_name, self.rust_name_span);
        let db_name = self.db_name.clone();
        let (from_doc_getter, conversion) = match self.container {
            Container::Array => {
                (quote::quote! { get_array }, quote::quote! { value.clone().huus_into_struct()? })
            }
            Container::HashMap(_) => (
                quote::quote! { get_document },
                quote::quote! { value.clone().huus_into_struct()? },
            ),
            Container::BTreeMap(_) => (
                quote::quote! { get_document },
                quote::quote! { value.clone().huus_into_struct()? },
            ),
            Container::Plain => {
                (self.variant.from_doc_getter(), self.variant.to_conversion(self.variant_span))
            }
        };

        if self.is_optional {
            quote::quote! {
                #rust_name: match doc.#from_doc_getter(#db_name) {
                    Ok(value) => Some({ #conversion }),
                    Err(bson::ordered::ValueAccessError::NotPresent) => None,
                    Err(bson::ordered::ValueAccessError::UnexpectedType) => {
                        return Err(huus::errors::ConversionError::wrong_type(#db_name.to_string()))
                    }
                },
            }
        } else {
            quote::quote! {
                #rust_name: match doc.#from_doc_getter(#db_name) {
                    Ok(value) => { #conversion }
                    Err(bson::ordered::ValueAccessError::NotPresent) => {
                        return Err(huus::errors::ConversionError::missing_key(#db_name.to_string()))
                    }
                    Err(bson::ordered::ValueAccessError::UnexpectedType) => {
                        return Err(huus::errors::ConversionError::wrong_type(#db_name.to_string()))
                    }
                },
            }
        }
    }

    pub fn make_into_doc_code(&self) -> proc_macro2::TokenStream {
        let rust_name = proc_macro2::Ident::new(&self.rust_name, self.rust_name_span);
        let db_name = self.db_name.clone();

        if self.is_optional {
            quote::quote! {
                if let Some(data) = self.#rust_name {
                    doc.insert(#db_name, data.huus_into_bson());
                }
            }
        } else {
            quote::quote! { doc.insert(#db_name, self.#rust_name.huus_into_bson()); }
        }
    }

    pub fn make_filter_build_code(&self, coll_name: &Option<String>) -> proc_macro2::TokenStream {
        let rust_name = proc_macro2::Ident::new(&self.rust_name, self.rust_name_span);
        let db_name = self.db_name.clone();
        let field_name = match coll_name {
            Some(_) => quote::quote! { #db_name.to_string() },
            None => quote::quote! { field.clone() + "." + #db_name },
        };
        quote::quote! { filter.incorporate(self.#rust_name.build_filter(#field_name)); }
    }

    pub fn make_filter_default_code(&self) -> proc_macro2::TokenStream {
        let rust_name = proc_macro2::Ident::new(&self.rust_name, self.rust_name_span);
        let type_name = self.to_filter();
        quote::quote! { #rust_name: <#type_name>::default(), }
    }

    pub fn make_value_build_code(&self) -> proc_macro2::TokenStream {
        let rust_name = proc_macro2::Ident::new(&self.rust_name, self.rust_name_span);
        let db_name = self.db_name.clone();
        quote::quote! {
            if let Some(data) = self.#rust_name {
                value.insert(#db_name.to_string(), data.build_value().into_bson());
            }
        }
    }

    pub fn make_value_default_code(&self) -> proc_macro2::TokenStream {
        let rust_name = proc_macro2::Ident::new(&self.rust_name, self.rust_name_span);
        quote::quote! { #rust_name: None, }
    }

    pub fn make_update_build_code(&self, coll_name: &Option<String>) -> proc_macro2::TokenStream {
        if self.db_name != "_id" {
            let rust_name = proc_macro2::Ident::new(&self.rust_name, self.rust_name_span);
            let db_name = self.db_name.clone();
            let field_name = match coll_name {
                Some(_) => quote::quote! { #db_name.to_string() },
                None => quote::quote! { field.clone() + "." + #db_name },
            };
            quote::quote! { update.incorporate(self.#rust_name.build_update(#field_name)); }
        } else {
            quote::quote!()
        }
    }

    pub fn make_update_default_code(&self) -> proc_macro2::TokenStream {
        let rust_name = proc_macro2::Ident::new(&self.rust_name, self.rust_name_span);
        let type_name = self.to_update();
        quote::quote! { #rust_name: <#type_name>::default(), }
    }
}

impl EnumChoice {
    fn make_data_choice_code(&self) -> proc_macro2::TokenStream {
        let span = self.rust_name_span.clone();
        let choice_ident = proc_macro2::Ident::new(&self.rust_name, span);
        quote::quote_spanned!(span=> #choice_ident)
    }

    fn make_from_string_code(&self, enum_ident: &proc_macro2::Ident) -> proc_macro2::TokenStream {
        let span = self.rust_name_span.clone();
        let db_name = &self.db_name;
        let choice_ident = proc_macro2::Ident::new(&self.rust_name, span);
        quote::quote! { #db_name => Ok(#enum_ident::#choice_ident), }
    }

    fn make_to_string_code(&self, enum_ident: &proc_macro2::Ident) -> proc_macro2::TokenStream {
        let span = self.rust_name_span.clone();
        let db_name = &self.db_name;
        let choice_ident = proc_macro2::Ident::new(&self.rust_name, span);
        quote::quote! { #enum_ident::#choice_ident => #db_name, }
    }

    fn make_value_choice_code(&self) -> proc_macro2::TokenStream {
        let span = self.rust_name_span.clone();
        let choice_ident = proc_macro2::Ident::new(&self.rust_name, span);
        quote::quote_spanned!(span=> #choice_ident)
    }
}

impl UnionChoice {
    fn make_data_choice_code(&self) -> proc_macro2::TokenStream {
        let span = self.rust_name_span.clone();
        let choice_ident = proc_macro2::Ident::new(&self.rust_name, span);
        let variant = self.variant.to_data();
        quote::quote_spanned!(span=> #choice_ident(#variant),)
    }

    fn make_from_doc_code(&self, union_ident: &proc_macro2::Ident) -> proc_macro2::TokenStream {
        let span = self.rust_name_span.clone();
        let name = &self.db_name;
        let choice_ident = proc_macro2::Ident::new(&self.rust_name, span);
        let variant = self.variant.to_data();
        quote::quote_spanned!(span=> #name => {
            Ok(#union_ident::#choice_ident(#variant::from_doc(doc)?))
        })
    }

    fn make_into_doc_code(&self, union_ident: &proc_macro2::Ident) -> proc_macro2::TokenStream {
        let span = self.rust_name_span.clone();
        let name = &self.db_name;
        let choice_ident = proc_macro2::Ident::new(&self.rust_name, span);
        quote::quote_spanned!(span=> #union_ident::#choice_ident(data) => {
            let mut doc = data.into_doc();
            doc.insert_bson(#UNION_VARIANT_KEY.to_string(), bson::Bson::String(#name.to_string()));
            doc
        })
    }

    fn make_filter_choice_code(&self) -> proc_macro2::TokenStream {
        let span = self.rust_name_span.clone();
        let choice_ident = proc_macro2::Ident::new(&self.rust_name, span);
        let variant = self.variant.to_filter();
        quote::quote_spanned!(span=> #choice_ident(#variant),)
    }

    fn make_filter_build_code(&self, union_ident: &proc_macro2::Ident) -> proc_macro2::TokenStream {
        let span = self.rust_name_span.clone();
        let choice_ident = proc_macro2::Ident::new(&self.rust_name, span);
        quote::quote_spanned!(span=> #union_ident::#choice_ident(filter) => {
            filter.build_filter(field)
        })
    }

    fn make_value_choice_code(&self) -> proc_macro2::TokenStream {
        let span = self.rust_name_span.clone();
        let ident = proc_macro2::Ident::new(&self.rust_name, span);
        let variant = &self.variant.to_value();
        quote::quote_spanned!(span=> #ident(#variant),)
    }

    fn make_value_build_code(&self, union_ident: &proc_macro2::Ident) -> proc_macro2::TokenStream {
        let span = self.rust_name_span.clone();
        let name = &self.db_name;
        let choice_ident = proc_macro2::Ident::new(&self.rust_name, span);
        quote::quote_spanned!(span=> #union_ident::#choice_ident(value) => {
            match value.build_value().into_bson() {
                bson::Bson::Document(mut doc) => {
                    let value = bson::Bson::String(#name.to_string());
                    doc.insert_bson(#UNION_VARIANT_KEY.to_string(), value);
                    huus::values::Value::new(bson::Bson::Document(doc))
                }
                _ => panic!("Huus: Failed to cast union into a document"),
            }
        })
    }

    fn make_update_choice_code(&self) -> proc_macro2::TokenStream {
        let span = self.rust_name_span.clone();
        let ident = proc_macro2::Ident::new(&self.rust_name, span);
        let variant = self.variant.to_update();
        quote::quote_spanned!(span=> #ident(#variant),)
    }

    fn make_update_build_code(&self, union_ident: &proc_macro2::Ident) -> proc_macro2::TokenStream {
        let span = self.rust_name_span.clone();
        let name = &self.db_name;
        let choice_ident = proc_macro2::Ident::new(&self.rust_name, span);
        quote::quote_spanned!(span=> #union_ident::#choice_ident(update) => {
            let key = field.clone() + "." + #UNION_VARIANT_KEY;
            let value = bson::Bson::String(#name.to_string());
            let variant_update = huus::updates::Update::with_field(key, value);
            let mut result = update.build_update(field);
            result.incorporate(variant_update);
            result
        })
    }
}

// -------------------------------------------------------------------------------------------------

struct IndexedFields<'a> {
    spec: &'a Spec,
    fields: HashMap<String, Vec<String>>,
}

impl<'a> IndexedFields<'a> {
    fn new(spec: &'a Spec) -> Self {
        Self { spec: spec, fields: HashMap::new() }
    }

    fn prepare(mut self) -> HashMap<String, Vec<String>> {
        for entity in self.spec.entities.iter() {
            self.prepare_entity(entity);
        }
        self.fields
    }

    fn prepare_entity(&mut self, entity: &Entity) {
        match entity {
            Entity::Struct(struct_spec) => {
                if self.fields.get(&struct_spec.struct_name.name).is_none() {
                    self.prepare_struct(struct_spec);
                }
            }
            Entity::Enum(enum_spec) => {
                if self.fields.get(&enum_spec.name.name).is_none() {
                    self.prepare_enum(enum_spec);
                }
            }
            Entity::Union(union_spec) => {
                if self.fields.get(&union_spec.name.name).is_none() {
                    self.prepare_union(union_spec);
                }
            }
        }
    }

    fn prepare_struct(&mut self, struct_spec: &Struct) {
        let mut indexed_fields = Vec::new();
        for member in struct_spec.members.iter() {
            match &member.variant {
                Variant::Field(_) => {
                    if member.is_indexed {
                        indexed_fields.push(member.db_name.clone());
                    }
                }
                Variant::Struct(variant) | Variant::Union(variant) => {
                    let entity = self
                        .spec
                        .find_entity(&variant.name)
                        .expect(&format!("Failed to find '{}'", variant.name));
                    self.prepare_entity(entity);
                    let struct_indexed_fields = self
                        .fields
                        .get(&variant.name)
                        .expect(&format!("Failed to find indexed fields for '{}'", variant.name));

                    let keys = match &member.container {
                        Container::Array | Container::Plain => Vec::new(),
                        Container::BTreeMap(variant) | Container::HashMap(variant) => {
                            match variant {
                                Variant::Field(_) => Vec::new(),
                                Variant::Struct(key_type) | Variant::Enum(key_type) | Variant::Union(key_type) => {
                                    let entity = self
                                        .spec
                                        .find_entity(&key_type.name);
                                    match entity {
                                        Some(Entity::Enum(enum_spec)) => enum_spec.to_db_names(),
                                        _ => Vec::new(),
                                    }
                                }
                            }
                        }
                    };

                    let base = member.db_name.clone() + ".";
                    if keys.len() > 0 {
                        for key in keys {
                            for field in struct_indexed_fields.iter() {
                                indexed_fields.push(base.clone() + &key + "." + field);
                            }
                        }
                    } else {
                        for field in struct_indexed_fields.iter() {
                            indexed_fields.push(base.clone() + field);
                        }
                    }
                }
                Variant::Enum(_) => {}
            };
        }

        self.fields.insert(struct_spec.struct_name.name.clone(), indexed_fields);
    }

    fn prepare_enum(&mut self, enum_spec: &Enum) {
        self.fields.insert(enum_spec.name.name.clone(), Vec::new());
    }

    fn prepare_union(&mut self, union_spec: &Union) {
        self.fields.insert(union_spec.name.name.clone(), Vec::new());
    }
}

// -------------------------------------------------------------------------------------------------

pub fn make_output(spec: Spec) -> proc_macro::TokenStream {
    let mut entities = Vec::new();
    let indexed_fields_map = IndexedFields::new(&spec).prepare();
    for entity in spec.entities {
        entities.push(match entity {
            Entity::Struct(struct_spec) => {
                let msg =
                    format!("Failed fo find indexed fields for '{}'", struct_spec.struct_name.name);
                let indexed_fields =
                    indexed_fields_map.get(&struct_spec.struct_name.name).expect(&msg);
                make_struct_output(struct_spec, indexed_fields.clone())
            }
            Entity::Enum(enum_spec) => make_enum_output(enum_spec),
            Entity::Union(union_spec) => make_union_output(union_spec),
        });
    }
    quote::quote!(#( #entities )*).into()
}

fn make_struct_output(spec: Struct, indexed_fields: Vec<String>) -> proc_macro2::TokenStream {
    let data_name = spec.struct_name.to_data();
    let filter_name = spec.struct_name.to_filter();
    let update_name = spec.struct_name.to_update();
    let value_name = spec.struct_name.to_value();
    let indexed_fields_1 = &indexed_fields;
    let indexed_fields_2 = indexed_fields.clone();

    let (
        filter_trait_name,
        filter_build_signature,
        update_trait_name,
        update_build_signature,
        queries_code,
    ) = match &spec.collection_name {
        Some(collection_name) => (
            proc_macro2::Ident::new(BUILD_FILTER_TRAIT, spec.struct_name_span),
            quote::quote! { fn build_filter(self) },
            proc_macro2::Ident::new(BUILD_UPDATE_TRAIT, spec.struct_name_span),
            quote::quote! { fn build_update(self) },
            quote::quote! {
                impl huus::query::Query for #data_name {
                    type Data = #data_name;
                    type Update = #update_name;
                    fn get_collection_name() -> &'static str {
                        #collection_name
                    }
                    fn get_indexed_fields() -> Vec<&'static str> {
                        vec![#( #indexed_fields_1, )*]
                    }
                }
                impl huus::query::Query for #filter_name {
                    type Data = #data_name;
                    type Update = #update_name;
                    fn get_collection_name() -> &'static str {
                        #collection_name
                    }
                    fn get_indexed_fields() -> Vec<&'static str> {
                        vec![#( #indexed_fields_2, )*]
                    }
                }
            },
        ),
        None => (
            proc_macro2::Ident::new(BUILD_INNER_FILTER_TRAIT, spec.struct_name_span),
            quote::quote! { fn build_filter(self, field: String) },
            proc_macro2::Ident::new(BUILD_INNER_UPDATE_TRAIT, spec.struct_name_span),
            quote::quote! { fn build_update(self, field: String) },
            quote::quote!(),
        ),
    };

    let mut struct_members = Vec::new();
    let mut from_doc_builds = Vec::new();
    let mut into_doc_builds = Vec::new();
    let mut filter_members = Vec::new();
    let mut filter_builds = Vec::new();
    let mut filter_defaults = Vec::new();
    let mut value_members = Vec::new();
    let mut value_builds = Vec::new();
    let mut value_defaults = Vec::new();
    let mut update_members = Vec::new();
    let mut update_builds = Vec::new();
    let mut update_defaults = Vec::new();
    for member in spec.members {
        struct_members.push(member.make_struct_member_code());
        from_doc_builds.push(member.make_from_doc_code());
        into_doc_builds.push(member.make_into_doc_code());
        filter_members.push(member.make_filter_member_code());
        filter_builds.push(member.make_filter_build_code(&spec.collection_name));
        filter_defaults.push(member.make_filter_default_code());
        value_members.push(member.make_value_member_code());
        value_builds.push(member.make_value_build_code());
        value_defaults.push(member.make_value_default_code());
        update_members.push(member.make_update_member_code());
        update_builds.push(member.make_update_build_code(&spec.collection_name));
        update_defaults.push(member.make_update_default_code());
    }

    let output = quote::quote! {
        #[derive(Clone, Debug, PartialEq)]
        pub struct #data_name {
            #( #struct_members )*
        }
        impl huus::conversions::FromDoc for #data_name {
            fn from_doc(doc: bson::Document)
            -> Result<#data_name, huus::errors::ConversionError> {
                use huus::conversions::{HuusKey, HuusIntoStruct};
                Ok(#data_name {
                    #( #from_doc_builds )*
                })
            }
        }
        impl huus::conversions::IntoDoc for #data_name {
            fn into_doc(self) -> bson::Document {
                use huus::conversions::HuusIntoBson;
                let mut doc = bson::Document::new();
                #( #into_doc_builds )*
                doc
            }
        }
        #[derive(Clone, Debug)]
        pub struct #filter_name {
            #( #filter_members )*
        }
        impl huus::filters::#filter_trait_name for #filter_name {
            #filter_build_signature -> huus::filters::Filter {
                let mut filter = huus::filters::Filter::empty();
                #( #filter_builds )*
                filter
            }
        }
        impl Default for #filter_name {
            fn default() -> Self {
                Self { #( #filter_defaults )* }
            }
        }
        #[derive(Clone, Debug)]
        pub struct #value_name {
            #( #value_members )*
        }
        impl huus::values::BuildValue for #value_name {
            fn build_value(self) -> huus::values::Value {
                use huus::conversions::HuusIntoBson;
                let mut value = huus::values::ObjectValue::empty();
                #( #value_builds )*
                value.build_value()
            }
        }
        impl Default for #value_name {
            fn default() -> Self {
                Self { #( #value_defaults )* }
            }
        }
        #[derive(Clone, Debug)]
        pub struct #update_name {
            #( #update_members )*
        }
        impl huus::updates::#update_trait_name for #update_name {
            #update_build_signature -> huus::updates::Update {
                let mut update = huus::updates::Update::empty();
                #( #update_builds )*
                update
            }
        }
        impl Default for #update_name {
            fn default() -> Self {
                Self { #( #update_defaults )* }
            }
        }
        #queries_code
    };
    output.into()
}

fn make_enum_output(spec: Enum) -> proc_macro2::TokenStream {
    let data_name = spec.name.to_data();
    let value_name = spec.name.to_value();

    let mut data_choices = Vec::new();
    let mut data_from_string_builds = Vec::new();
    let mut data_to_string_builds = Vec::new();
    let mut value_choices = Vec::new();
    let mut value_from_string_builds = Vec::new();
    let mut value_to_string_builds = Vec::new();
    for choice in spec.choices {
        data_choices.push(choice.make_data_choice_code());
        data_from_string_builds.push(choice.make_from_string_code(&data_name));
        data_to_string_builds.push(choice.make_to_string_code(&data_name));
        value_choices.push(choice.make_value_choice_code());
        value_from_string_builds.push(choice.make_from_string_code(&value_name));
        value_to_string_builds.push(choice.make_to_string_code(&value_name));
    }

    let output = quote::quote! {
        #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum #data_name {
            #( #data_choices, )*
        }
        impl huus::conversions::HuusKey for #data_name {
            fn from_str(string: &str) -> Result<Self, huus::errors::ConversionError> {
                match string {
                    #( #data_from_string_builds )*
                    _ => Err(huus::errors::ConversionError::incorrect_value(string.to_string())),
                }
            }
            fn to_str(&self) -> &'static str {
                match self {
                    #( #data_to_string_builds )*
                }
            }
        }
        impl huus::conversions::HuusIntoBson for #data_name {
            fn huus_into_bson(self) -> bson::Bson {
                use huus::conversions::HuusKey;
                bson::Bson::String(self.to_str().to_string())
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum #value_name {
            #( #value_choices, )*
        }
        impl huus::values::BuildValue for #value_name {
            fn build_value(self) -> huus::values::Value {
                use huus::conversions::HuusKey;
                huus::values::Value::new(bson::Bson::String(self.to_str().to_string()))
            }
        }
        impl huus::conversions::HuusKey for #value_name {
            fn from_str(string: &str) -> Result<Self, huus::errors::ConversionError> {
                match string {
                    #( #value_from_string_builds )*
                    _ => Err(huus::errors::ConversionError::incorrect_value(string.to_string())),
                }
            }
            fn to_str(&self) -> &'static str {
                match self {
                    #( #value_to_string_builds )*
                }
            }
        }
    };
    output.into()
}

fn make_union_output(spec: Union) -> proc_macro2::TokenStream {
    let data_name = spec.name.to_data();
    let filter_name = spec.name.to_filter();
    let update_name = spec.name.to_update();
    let value_name = spec.name.to_value();

    let mut data_choices = Vec::new();
    let mut from_doc_builds = Vec::new();
    let mut into_doc_builds = Vec::new();
    let mut filter_choices = Vec::new();
    let mut filter_builds = Vec::new();
    let mut value_choices = Vec::new();
    let mut value_builds = Vec::new();
    let mut update_choices = Vec::new();
    let mut update_builds = Vec::new();
    for choice in spec.choices {
        data_choices.push(choice.make_data_choice_code());
        from_doc_builds.push(choice.make_from_doc_code(&data_name));
        into_doc_builds.push(choice.make_into_doc_code(&data_name));
        filter_choices.push(choice.make_filter_choice_code());
        filter_builds.push(choice.make_filter_build_code(&filter_name));
        value_choices.push(choice.make_value_choice_code());
        value_builds.push(choice.make_value_build_code(&value_name));
        update_choices.push(choice.make_update_choice_code());
        update_builds.push(choice.make_update_build_code(&update_name));
    }

    let output = quote::quote! {
        #[derive(Clone, Debug, PartialEq)]
        pub enum #data_name {
            #( #data_choices )*
        }
        impl huus::conversions::FromDoc for #data_name {
            fn from_doc(doc: bson::Document)
            -> Result<#data_name, huus::errors::ConversionError> {
                use huus::errors::ConversionError;
                match doc.get_str(#UNION_VARIANT_KEY) {
                    Ok(name) => {
                        match name {
                            #( #from_doc_builds )*
                            _ => Err(ConversionError::unexpected_value(name.to_string())),
                        }
                    }
                    Err(_) => {
                        Err(ConversionError::missing_key(#UNION_VARIANT_KEY.to_string()))
                    }
                }
            }
        }
        impl huus::conversions::IntoDoc for #data_name {
            fn into_doc(self) -> bson::Document {
                match self {
                    #( #into_doc_builds )*
                }
            }
        }
        #[derive(Clone, Debug)]
        pub enum #filter_name {
            #( #filter_choices )*
        }
        impl huus::filters::BuildInnerFilter for #filter_name {
            fn build_filter(self, field: String) -> huus::filters::Filter {
                match self {
                    #( #filter_builds )*
                }
            }
        }
        #[derive(Clone, Debug)]
        pub enum #value_name {
            #( #value_choices )*
        }
        impl huus::values::BuildValue for #value_name {
            fn build_value(self) -> huus::values::Value {
                match self {
                    #( #value_builds )*
                }
            }
        }
        #[derive(Clone, Debug)]
        pub enum #update_name {
            #( #update_choices )*
        }
        impl huus::updates::BuildInnerUpdate for #update_name {
            fn build_update(self, field: String) -> huus::updates::Update {
                match self {
                    #( #update_builds )*
                }
            }
        }
    };
    output.into()
}
