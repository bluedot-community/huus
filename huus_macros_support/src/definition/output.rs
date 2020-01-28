// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Structures for code generation.

/// Represents a parsing error pointing to a part that failed aiding the error handler display pin
/// the error message to correct place in the code.
pub enum ParseError {
    /// Error parsing rust name.
    RustName(String),

    /// Error parsing database name.
    DbName(String),

    /// Error parsing the type name.
    Type(String),
}

/// Represent build-in (mongodb) type.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BuiltInType {
    /// Corresponds to a floating point.
    F64,

    /// Corresponds to a string.
    String,

    /// Corresponds to an object ID.
    ObjectId,

    /// Corresponds to a boolean value.
    Bool,

    /// Corresponds to a date.
    Date,

    /// Corresponds to a 32-bit integer.
    I32,

    /// Corresponds to a 64-bit integer.
    I64,

    /// Corresponds to a BSON object.
    Bson,
}

impl BuiltInType {
    /// Constructs `BuiltInType` enum from string.
    pub fn from_name(name: &str) -> Result<Self, ()> {
        match name {
            "f64" => Ok(BuiltInType::F64),
            "String" => Ok(BuiltInType::String),
            "ObjectId" => Ok(BuiltInType::ObjectId),
            "bool" => Ok(BuiltInType::Bool),
            "Date" => Ok(BuiltInType::Date),
            "i32" => Ok(BuiltInType::I32),
            "i64" => Ok(BuiltInType::I64),
            "Bson" => Ok(BuiltInType::Bson),
            _ => return Err(()),
        }
    }

    /// Specifies if the given type supports indexing.
    fn allows_indexing(&self) -> bool {
        match self {
            BuiltInType::String | BuiltInType::ObjectId => true,
            _ => false,
        }
    }

    /// Returns a name of `Data` type.
    pub fn to_data(&self) -> &'static str {
        match self {
            BuiltInType::F64 => "f64",
            BuiltInType::String => "String",
            BuiltInType::ObjectId => "huus::types::ObjectId",
            BuiltInType::Bool => "bool",
            BuiltInType::Date => "huus::types::Date",
            BuiltInType::I32 => "i32",
            BuiltInType::I64 => "i64",
            BuiltInType::Bson => "bson::Document",
        }
    }

    /// Returns a name of `Filter` type.
    pub fn to_filter(&self) -> &'static str {
        match self {
            BuiltInType::F64 => "huus::filters::F64Entry",
            BuiltInType::String => "huus::filters::StringEntry",
            BuiltInType::ObjectId => "huus::filters::ObjectIdEntry",
            BuiltInType::Bool => "huus::filters::BooleanEntry",
            BuiltInType::Date => "huus::filters::DateEntry",
            BuiltInType::I32 => "huus::filters::I32Entry",
            BuiltInType::I64 => "huus::filters::I64Entry",
            BuiltInType::Bson => "huus::filters::BsonEntry",
        }
    }

    /// Returns a name of `Value` type.
    pub fn to_value(&self) -> &'static str {
        match self {
            BuiltInType::F64 => "f64",
            BuiltInType::String => "String",
            BuiltInType::ObjectId => "huus::types::ObjectId",
            BuiltInType::Bool => "bool",
            BuiltInType::Date => "huus::types::Date",
            BuiltInType::I32 => "i32",
            BuiltInType::I64 => "i64",
            BuiltInType::Bson => "bson::Document",
        }
    }

    /// Returns a name of `Update` type.
    pub fn to_update(&self) -> &'static str {
        match self {
            BuiltInType::F64 => "huus::updates::F64Entry",
            BuiltInType::String => "huus::updates::StringEntry",
            BuiltInType::ObjectId => "huus::updates::ObjectIdEntry",
            BuiltInType::Bool => "huus::updates::BooleanEntry",
            BuiltInType::Date => "huus::updates::DateEntry",
            BuiltInType::I32 => "huus::updates::I32Entry",
            BuiltInType::I64 => "huus::updates::I64Entry",
            BuiltInType::Bson => "huus::updates::BsonEntry",
        }
    }

    /// Returns name of `bson::Bson` getter for the type represented by this structure.
    pub fn from_doc_getter(&self) -> &'static str {
        match self {
            BuiltInType::F64 => "get_f64",
            BuiltInType::String => "get_str",
            BuiltInType::ObjectId => "get_object_id",
            BuiltInType::Bool => "get_bool",
            BuiltInType::Date => "get_utc_datetime",
            BuiltInType::I32 => "get_i32",
            BuiltInType::I64 => "get_i64",
            BuiltInType::Bson => "get_document",
        }
    }

    /// Returns a code to converting thus BSON value to the underlying type.
    pub fn to_conversion(&self) -> &'static str {
        let output = match self {
            BuiltInType::F64 => "value",
            BuiltInType::String => "value.to_string()",
            BuiltInType::ObjectId => "value.clone()",
            BuiltInType::Bool => "value",
            BuiltInType::Date => "value.clone()",
            BuiltInType::I32 => "value",
            BuiltInType::I64 => "value",
            BuiltInType::Bson => "value.clone()",
        };
        output.into()
    }
}

/// Represents a used-defined type.
#[derive(Clone, Debug, PartialEq)]
pub struct DefinedType {
    /// Name of the user-defined type.
    pub name: String,
}

impl DefinedType {
    /// Constructs a new `DefinedType`.
    pub fn new(name: String) -> Self {
        Self { name }
    }

    /// Returns a name of `Data` type.
    pub fn to_data(&self) -> String {
        self.name.clone() + "Data"
    }

    /// Returns a name of `Insert` type.
    pub fn to_insert(&self) -> String {
        self.name.clone() + "Insert"
    }

    /// Returns a name of `Filter` type.
    pub fn to_filter(&self) -> String {
        self.name.clone() + "Filter"
    }

    /// Returns a name of `Value` type.
    pub fn to_value(&self) -> String {
        self.name.clone() + "Value"
    }

    /// Returns a name of `Update` type.
    pub fn to_update(&self) -> String {
        self.name.clone() + "Update"
    }
}

impl PartialEq<str> for DefinedType {
    fn eq(&self, name: &str) -> bool {
        self.name == name
    }
}

/// Represents a name of given type and the way it was defined.
#[derive(Clone, Debug, PartialEq)]
pub enum Variant {
    /// Corresponds to a built-in type.
    Field(BuiltInType),

    /// Corresponds to a user-defined structure.
    Struct(DefinedType),

    /// Corresponds to a user-defined enum.
    Enum(DefinedType),

    /// Corresponds to a user-defined union.
    Union(DefinedType),
}

impl Variant {
    /// Specifies if the given type supports indexing.
    fn allows_indexing(&self) -> bool {
        match self {
            Variant::Field(field) => field.allows_indexing(),
            _ => false,
        }
    }

    /// Returns a name of `Data` type.
    pub fn to_data(&self) -> String {
        match self {
            Variant::Field(field) => field.to_data().to_string(),
            Variant::Struct(name) => name.to_data(),
            Variant::Enum(name) => name.to_data(),
            Variant::Union(name) => name.to_data(),
        }
    }

    /// Returns a long version of name of `Filter` type.
    pub fn to_long_filter(&self) -> String {
        let output = match self {
            Variant::Field(field) => field.to_filter().to_string(),
            Variant::Struct(name) => {
                format!("huus::filters::ObjectEntry<{}, {}>", name.to_filter(), name.to_data())
            }
            Variant::Enum(name) => format!("huus::filters::EnumEntry<{}>", name.to_data()),
            Variant::Union(name) => {
                format!("huus::filters::ObjectEntry<{}, {}>", name.to_filter(), name.to_data())
            }
        };
        output.into()
    }

    /// Returns a short version of name of `Filter` type.
    pub fn to_short_filter(&self) -> String {
        match self {
            Variant::Field(field) => field.to_filter().to_string(),
            Variant::Struct(name) => name.to_filter(),
            Variant::Enum(name) => name.to_data(),
            Variant::Union(name) => name.to_filter(),
        }
    }

    /// Returns a name of `Value` type.
    pub fn to_value(&self) -> String {
        match self {
            Variant::Field(field) => field.to_value().to_string(),
            Variant::Struct(name) => name.to_value(),
            Variant::Enum(name) => name.to_value(),
            Variant::Union(name) => name.to_value(),
        }
    }

    /// Returns a long version of name of `Update` type.
    pub fn to_long_update(&self) -> String {
        match self {
            Variant::Field(field) => field.to_update().to_string(),
            Variant::Struct(name) => {
                format!("huus::updates::ObjectEntry<{}, {}>", name.to_update(), name.to_value())
            }
            Variant::Enum(name) => format!("huus::updates::EnumEntry<{}>", name.to_value()),
            Variant::Union(name) => {
                format!("huus::updates::ObjectEntry<{}, {}>", name.to_update(), name.to_value())
            }
        }
    }

    /// Returns a short version of name of `Update` type.
    pub fn to_short_update(&self) -> String {
        match self {
            Variant::Field(field) => field.to_update().to_string(),
            Variant::Struct(name) => name.to_update(),
            Variant::Enum(name) => name.to_update(),
            Variant::Union(name) => name.to_update(),
        }
    }

    /// Returns name of `bson::Bson` getter for the type represented by this structure.
    pub fn from_doc_getter(&self) -> &'static str {
        match self {
            Variant::Field(field) => field.from_doc_getter(),
            Variant::Struct(_) => "get_document",
            Variant::Enum(_) => "get_str",
            Variant::Union(_) => "get_document",
        }
    }

    /// Returns a code to converting thus BSON value to the underlying type.
    pub fn to_conversion(&self) -> String {
        match self {
            Variant::Field(field) => field.to_conversion().to_string(),
            Variant::Struct(name) => format!("{}::from_doc(value.clone())?", name.to_data()),
            Variant::Enum(name) => format!("{}::from_str(&value)?", name.to_data()),
            Variant::Union(name) => format!("{}::from_doc(value.clone())?", name.to_data()),
        }
    }
}

/// Represents the type of container for member.
#[derive(Clone, Debug, PartialEq)]
pub enum Container {
    /// Corresponds to `Vec`.
    Array,

    /// Corresponds to `BTreeMap`.
    BTreeMap(Variant),

    /// Corresponds to `HashMap`.
    HashMap(Variant),

    /// Corresponds to a type not contained in any container.
    Plain,
}

impl Container {
    /// Returns `true` if the type is not contained in any container.
    pub fn is_plain(&self) -> bool {
        *self == Self::Plain
    }

    /// Returns `true` if the type is inside an array.
    pub fn is_array(&self) -> bool {
        *self == Self::Array
    }
}

/// Represents a structure member (database object field).
#[derive(Clone, Debug)]
pub struct Member {
    /// Name to be used in generated code.
    pub rust_name: String,

    /// Name to be used in database queries.
    pub db_name: String,

    /// Name of the type.
    pub variant: Variant,

    /// Container the data is to be stored in.
    pub container: Container,

    /// Specifies if the member is optional.
    pub is_optional: bool,

    /// Specifies if an index should be created for the given database field.
    pub is_indexed: bool,
}

impl Member {
    /// Constructs a new `Member`.
    pub fn new(
        rust_name: String,
        db_name: String,
        variant: Variant,
        container: Container,
        is_optional: bool,
        is_indexed: bool,
    ) -> Result<Self, ParseError> {
        // Check if the name is allowed
        const FORBIDDEN_PREFIX: &'static str = "_huus";
        if rust_name.starts_with(FORBIDDEN_PREFIX) {
            let msg = format!("Member name cannot start with '{}'", FORBIDDEN_PREFIX);
            return Err(ParseError::RustName(msg));
        }

        // Check if the database name contains only allowed characters
        for character in db_name.chars() {
            if !character.is_alphanumeric() && character != '_' {
                let msg = format!(
                    "Field name can contain only alphanumerics and underscore, but '{}' was used",
                    character
                );
                return Err(ParseError::DbName(msg));
            }
        }

        // Check if indexing is requested only for types supporting indexing
        if is_indexed && !variant.allows_indexing() {
            let msg = "Indexing not supported for this type".to_string();
            return Err(ParseError::Type(msg));
        }

        Ok(Member { rust_name, db_name, variant, container, is_optional, is_indexed })
    }

    /// Returns a name of `Data` type.
    pub fn to_data(&self) -> String {
        let variant = self.variant.to_data();
        match &self.container {
            Container::Array => format!("Vec<{}>", variant),
            Container::HashMap(key_variant) => {
                let key = key_variant.to_data();
                format!("std::collections::HashMap<{}, {}>", key, variant)
            }
            Container::BTreeMap(key_variant) => {
                let key = key_variant.to_data();
                format!("std::collections::BTreeMap<{}, {}>", key, variant)
            }
            Container::Plain => variant,
        }
    }

    /// Returns a name of `Filter` type.
    pub fn to_filter(&self) -> String {
        match &self.container {
            Container::Array => {
                let key = self.variant.to_short_filter();
                let value = self.variant.to_data();
                format!("huus::filters::ArrayEntry<{}, {}>", key, value)
            }
            Container::BTreeMap(key_variant) => {
                let key = key_variant.to_data();
                let value = self.variant.to_data();
                format!("huus::filters::BTreeMapEntry<{}, {}>", key, value)
            }
            Container::HashMap(key_variant) => {
                let key = key_variant.to_data();
                let value = self.variant.to_data();
                format!("huus::filters::HashMapEntry<{}, {}>", key, value)
            }
            Container::Plain => self.variant.to_long_filter(),
        }
    }

    /// Returns a name of `Value` type.
    pub fn to_value(&self) -> String {
        // TODO: Add separate entries for maps.
        match &self.container {
            Container::Array => format!("huus::values::ArrayEntry<{}>", self.variant.to_value()),
            Container::HashMap(key_variant) => {
                let key = key_variant.to_value();
                let value = self.variant.to_data();
                format!("huus::values::Entry<std::collections::HashMap<{}, {}>>", key, value)
            }
            Container::BTreeMap(key_variant) => {
                let key = key_variant.to_value();
                let value = self.variant.to_data();
                format!("huus::values::Entry<std::collections::BTreeMap<{}, {}>>", key, value)
            }
            Container::Plain => format!("huus::values::Entry<{}>", self.variant.to_value()),
        }
    }

    /// Returns a name of `Update` type.
    pub fn to_update(&self) -> String {
        match &self.container {
            Container::Array => {
                let update = self.variant.to_short_update();
                let value = self.variant.to_value();
                format!("huus::updates::ArrayEntry<{}, {}>", update, value)
            }
            Container::BTreeMap(key_variant) => {
                let key = key_variant.to_data();
                let value = self.variant.to_data();
                format!("huus::updates::BTreeMapEntry<{}, {}>", key, value)
            }
            Container::HashMap(key_variant) => {
                let key = key_variant.to_data();
                let value = self.variant.to_data();
                format!("huus::updates::HashMapEntry<{}, {}>", key, value)
            }
            Container::Plain => self.variant.to_long_update(),
        }
    }

    /// Returns name of `bson::Bson` getter for the type represented by this structure.
    pub fn from_doc_getter(&self) -> &'static str {
        match self.container {
            Container::Array => "get_array",
            Container::HashMap(_) => "get_document",
            Container::BTreeMap(_) => "get_document",
            Container::Plain => self.variant.from_doc_getter(),
        }
    }

    /// Returns a code to converting thus BSON value to the underlying type.
    pub fn to_conversion(&self) -> String {
        match self.container {
            Container::Array => "value.clone().huus_into_struct()?".to_string(),
            Container::HashMap(_) => "value.clone().huus_into_struct()?".to_string(),
            Container::BTreeMap(_) => "value.clone().huus_into_struct()?".to_string(),
            Container::Plain => self.variant.to_conversion(),
        }
    }

    /// Returns a code initializing a default value of the underlying type.
    pub fn to_default(&self) -> Option<&str> {
        match self.container {
            Container::Array => Some("Vec::new()"),
            Container::HashMap(_) => Some("std::collections::HashMap::new()"),
            Container::BTreeMap(_) => Some("std::collections::BTreeMap::new()"),
            Container::Plain => None,
        }
    }
}

/// Represents an enum variant.
#[derive(Clone, Debug)]
pub struct EnumChoice {
    /// Name to be used in code.
    pub rust_name: String,

    /// Name to be used in database.
    pub db_name: String,
}

impl EnumChoice {
    /// Constructs a new `EnumChoice`.
    pub fn new(rust_name: String, db_name: String) -> Self {
        Self { rust_name, db_name }
    }
}

/// Represents an enum variant.
#[derive(Clone, Debug)]
pub struct UnionChoice {
    /// Name to be used in code.
    pub rust_name: String,

    /// Name to be used in database.
    pub db_name: String,

    /// Name of the corresponding structure.
    pub variant: DefinedType,
}

impl UnionChoice {
    /// Constructs a new `UnionChoice`.
    pub fn new(rust_name: String, db_name: String, variant: DefinedType) -> Self {
        Self { rust_name, db_name, variant }
    }
}

/// Represents a structure.
#[derive(Clone, Debug)]
pub struct Struct {
    /// Name of the structure.
    pub struct_name: DefinedType,

    /// Name of the collection. If specified this is the type of the main document stored in that
    /// collection. For embedded documents the collection name should be `None`.
    pub collection_name: Option<String>,

    /// List of all members of this structure (fields in the database object).
    pub members: Vec<Member>,

    /// List of fields (including fields in embedded documents) that should be indexed.
    pub indexed_fields: Vec<String>,
}

/// Represents an enum.
#[derive(Clone, Debug)]
pub struct Enum {
    /// Name of the enum.
    pub name: DefinedType,

    /// List of possible enum variants.
    pub choices: Vec<EnumChoice>,
}

impl Enum {
    /// Prepares a list of all possible enum values as represented in the database.
    pub fn to_db_names(&self) -> Vec<String> {
        let mut result = Vec::with_capacity(self.choices.len());
        for choice in self.choices.iter() {
            result.push(choice.db_name.clone());
        }
        result
    }
}

/// Represents an union.
#[derive(Clone, Debug)]
pub struct Union {
    /// Name of the union.
    pub name: DefinedType,

    /// List of possible union variants.
    pub choices: Vec<UnionChoice>,
}

/// Holds information about parsed entities (structures, enums and unions).
#[derive(Clone, Debug)]
pub enum Entity {
    /// Holds information about parsed structure.
    Struct(Struct),

    /// Holds information about parsed enum.
    Enum(Enum),

    /// Holds information about parsed union.
    Union(Union),
}

/// Holds information about all parsed entities.
pub struct Schema {
    /// A list of all parsed entities.
    pub entities: Vec<Entity>,
}

impl Schema {
    /// Constructs a new `Schema`.
    pub fn new() -> Self {
        Self { entities: Vec::new() }
    }

    /// Searches for an entity with the given name.
    pub fn find_entity(&self, name: &str) -> Option<&Entity> {
        for entity in self.entities.iter() {
            match entity {
                Entity::Struct(struct_spec) => {
                    if struct_spec.struct_name == *name {
                        return Some(entity);
                    }
                }
                Entity::Enum(enum_spec) => {
                    if enum_spec.name == *name {
                        return Some(entity);
                    }
                }
                Entity::Union(union_spec) => {
                    if union_spec.name == *name {
                        return Some(entity);
                    }
                }
            }
        }
        None
    }

    /// Searches for a structure that corresponds to the main document in the given collection.
    pub fn find_entity_for_collection(&self, name: &str) -> Option<&Struct> {
        for entity in self.entities.iter() {
            match entity {
                Entity::Struct(struct_spec) => {
                    if let Some(collection_name) = &struct_spec.collection_name {
                        if collection_name == name {
                            return Some(struct_spec);
                        }
                    }
                }
                _ => {}
            }
        }
        None
    }
}
