// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Verification for instructions integrity.

use std::{cell::RefCell, collections::BTreeSet};

use chrono::{DateTime, Utc};

use crate::{
    definition::output::*,
    formulation::{generator::Generator, input::*, output::*},
};

const ENTITY: &str = "Failed to find an entity";

// -------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq)]
enum UpdateType {
    Update,
    Replacement,
}

// -------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq)]
enum QueryOperator {
    Eq,
    Gt,
    Gte,
    In,
    Lt,
    Lte,
    Ne,
    Nin,
}

impl QueryOperator {
    fn matches(&self, builtin: &BuiltInType, container: &Container) -> bool {
        if container.is_plain() {
            match builtin {
                BuiltInType::Bson => false,
                _ => true,
            }
        } else if container.is_array() {
            match self {
                Self::In | Self::Nin => true,
                _ => false,
            }
        } else {
            false
        }
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq)]
enum UpdateOperator {
    Inc,
    Min,
    Max,
    Mul,
    CurrentDate,
    Rename,
    Unset,
    Set,
    SetOnInsert,
    AddToSet,
    Pop,
    Pull,
    Push,
    PullAll,
}

impl UpdateOperator {
    pub fn escapes_container(&self) -> bool {
        match self {
            Self::AddToSet | Self::Pop | Self::Pull | Self::Push => true,
            _ => false,
        }
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq)]
enum Conversion {
    Data,
    Filter,
    Replacement,
    Update(UpdateOperator),
}

impl Conversion {
    pub fn escapes_container(&self) -> bool {
        match self {
            Self::Update(operator) => operator.escapes_container(),
            _ => false,
        }
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug)]
enum VariantInfo {
    Field(BuiltInType),
    Entity(Entity),
}

#[derive(Debug)]
struct MemberInfo {
    pub info: VariantInfo,
    pub variant: Variant,
    pub container: Container,
}

impl MemberInfo {
    pub fn new(spec: &Spec, variant: Variant, container: Container) -> Result<Self, Problem> {
        let info = match &variant {
            Variant::Struct(name) | Variant::Enum(name) | Variant::Union(name) => {
                VariantInfo::Entity(spec.find_entity(&name.name).expect(ENTITY).clone())
            }
            Variant::Field(builtin) => VariantInfo::Field(*builtin)
        };

        Ok(Self { info, variant, container })
    }

    pub fn to_cast(&self, escape_container: bool) -> Cast {
        Cast {
            variant: self.variant.clone(),
            container: if escape_container { Container::Plain } else { self.container.clone() },
        }
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Problem {
    MacroEmpty,
    MainDocAsStruct,
    MainDocNotDefined,
    QueryBothUpdateAndRepl,
    QueryEmpty,
    AttrWithDots,
    FieldsMissing,
    FieldNotFound,
    FieldOnEnum,
    FieldOnPlain,
    FieldAmbiguous,
    OperatorUnknown,
    OperatorIncorrect,
    ExpCode,
    ExpCodeComp,
    ExpCodeEnum,
    ExpCodeUnion,
    ExpObject,
    ExpKey,
    ExpPlain,
    ExpArray,
    ExpF64,
    ExpString,
    ExpOid,
    ExpBool,
    ExpDate,
    ExpI32,
    ExpI64,
    ExpBson,
    ExpDateObj,
    ExpEmptyString,
}

impl Problem {
    fn as_str(&self) -> &'static str {
        match self {
            Self::MacroEmpty => "The macro seems to be empty",
            Self::MainDocAsStruct => "Main document for has to be a structure",
            Self::MainDocNotDefined => "Main document for this collection is not defined",
            Self::QueryBothUpdateAndRepl => "The query contains both update and replacement fields",
            Self::QueryEmpty => "The query seems to be empty",
            Self::AttrWithDots => "Attributes in replacement/data document cannot contain dots (.)",
            Self::FieldsMissing => "Required fields are missing",
            Self::FieldNotFound => "No more fields can be specified for an enum",
            Self::FieldOnEnum => "No more fields can be specified for a plain field",
            Self::FieldOnPlain => "Field does not match in the schema",
            Self::FieldAmbiguous => "Fields that match many union variants are not supported yet",
            Self::OperatorUnknown => "Unknown query operator",
            Self::OperatorIncorrect => "This operator cannot be used with the declared type",
            Self::ExpCode => "This value is supported only in `code` mode",
            Self::ExpCodeComp => "Composed data are supported only in `code` mode",
            Self::ExpCodeEnum => "Enums are supported only in `code` mode",
            Self::ExpCodeUnion => "Unions are supported only in `code` mode",
            Self::ExpObject => "Expected an object",
            Self::ExpKey => "Expected a literal key",
            Self::ExpPlain => "Expected a type without container",
            Self::ExpArray => "Expected an array",
            Self::ExpF64 => "Expected a floating point value",
            Self::ExpString => "Expected a string",
            Self::ExpOid => "Expected an Object ID",
            Self::ExpBool => "Expected a boolean value",
            Self::ExpDate => "Expected a date in 'YYYY-mm-ddTHH:MM:ss' format",
            Self::ExpI32 => "Expected a 32-bit integer",
            Self::ExpI64 => "Expected a 64-bit integer",
            Self::ExpBson => "BSON objects are supported only in `code` mode",
            Self::ExpDateObj => r#"Expected `true` or object `{"$type":"timestamp"|"datetime"}`"#,
            Self::ExpEmptyString => "Expected an empty string"
        }
    }
}

#[derive(Clone)]
pub struct Verdict {
    pub problems: Vec<Problem>,
}

impl Verdict {
    fn new() -> Self {
        Self { problems: Vec::new() }
    }

    pub fn format(&self) -> String {
        let contents = self.problems.iter()
            .map(|p| "huus_macros_support::Problem::".to_string() + &format!("{:?}", p))
            .collect::<Vec<String>>()
            .join(", ");

        "vec![".to_string() + &contents + "]"
    }
}

// -------------------------------------------------------------------------------------------------

pub struct Validator<'a> {
    collection: SpannedCollection,
    object: ObjectTemplate,
    spec: &'a Spec,
    verdict: RefCell<Verdict>,
    testing: bool,
}

impl<'a> Validator<'a> {
    pub fn new(
        collection: SpannedCollection,
        object: ObjectTemplate,
        spec: &'a Spec,
        testing: bool,
    ) -> Self {
        Self { collection, object, spec, verdict: RefCell::new(Verdict::new()), testing }
    }

    pub fn verify_data(self) -> Result<Generator, Verdict> {
        let struct_spec = self.find_struct_for_collection(&self.collection.name)?;
        let object = self.convert_object(&struct_spec, self.object.clone(), Conversion::Data);
        self.make_generator(struct_spec.struct_name.clone(), object)
    }

    pub fn verify_filter(self) -> Result<Generator, Verdict> {
        let struct_spec = self.find_struct_for_collection(&self.collection.name)?;
        let object = self.convert_object(&struct_spec, self.object.clone(), Conversion::Filter);
        self.make_generator(struct_spec.struct_name.clone(), object)
    }

    pub fn verify_update(self) -> Result<Generator, Verdict> {
        let struct_spec = self.find_struct_for_collection(&self.collection.name)?;
        let object = match self.verify_update_type()? {
            UpdateType::Update => self.convert_update_object(&struct_spec, self.object.clone()),
            UpdateType::Replacement => {
                self.convert_object(&struct_spec, self.object.clone(), Conversion::Replacement)
            }
        };
        self.make_generator(struct_spec.struct_name.clone(), object)
    }
}

// -------------------------------------------------------------------------------------------------
// Helper search

impl<'a> Validator<'a> {
    fn find_struct_for_collection(&self, collection_name: &str) -> Result<&Struct, Verdict> {
        match self.spec.find_entity_for_collection(&collection_name) {
            Some(struct_spec) => match struct_spec {
                Entity::Struct(struct_spec) => Ok(struct_spec),
                _ => {
                    self.error(&self.collection.span, Problem::MainDocAsStruct);
                    Err(self.verdict.borrow().clone())
                }
            },
            None => {
                self.error(&self.collection.span, Problem::MainDocNotDefined);
                Err(self.verdict.borrow().clone())
            }
        }
    }

    fn find_member(
        &self,
        struct_spec: &'a Struct,
        mut attribute: SpannedAttribute,
    ) -> Result<MemberInfo, Problem> {
        let part = attribute.pop().expect("No more attribute parts to check");
        if let Part::Key(key) = part.part {
            for member in struct_spec.members.iter() {
                if member.db_name == key {
                    // Ignore index parts in arrays
                    let mut container = member.container.clone();
                    if member.container.is_array() {
                        if attribute.next().map(|p| !p.is_key()).unwrap_or(false) {
                            let _ = attribute.pop();
                            container = Container::Plain;
                        }
                    }

                    let info = MemberInfo::new(&self.spec, member.variant.clone(), container)?;
                    return if attribute.len() == 0 {
                        // No more attribute parts to check - return the current member
                        Ok(info)
                    } else {
                        match &info.info {
                            VariantInfo::Entity(entity) => {
                                match entity {
                                    Entity::Struct(struct_spec) => {
                                        self.find_member(struct_spec, attribute)
                                    }
                                    Entity::Union(union_spec) => {
                                        self.peek_member(union_spec, attribute)
                                    }
                                    Entity::Enum(_) => {
                                        Err(Problem::FieldOnEnum)
                                    }
                                }
                            }
                            VariantInfo::Field(_) => {
                                Err(Problem::FieldOnPlain)
                            }
                        }
                    };
                }
            }
            Err(Problem::FieldNotFound)
        } else {
            Err(Problem::ExpKey)
        }
    }

    fn peek_member(
        &self,
        union_spec: &'a Union,
        attribute: SpannedAttribute,
    ) -> Result<MemberInfo, Problem> {
        let mut peeks = Vec::with_capacity(union_spec.choices.len());
        for choice in union_spec.choices.iter() {
            match self.spec.find_entity(&choice.variant.name).expect(ENTITY) {
                Entity::Struct(struct_spec) => {
                    if let Ok(member) = self.find_member(struct_spec, attribute.clone()) {
                        peeks.push(member);
                    }
                }
                _ => panic!("Union should be composed only of structures"),
            }
        }

        match peeks.len() {
            1 => Ok(peeks.pop().unwrap()),
            0 => Err(Problem::FieldNotFound),
            _ => Err(Problem::FieldAmbiguous),
        }
    }
}

// -------------------------------------------------------------------------------------------------
// Helper build and validation methods

impl<'a> Validator<'a> {
    fn verify_update_type(&self) -> Result<UpdateType, Verdict> {
        let mut has_updates = false;
        let mut has_replacements = false;

        for field in self.object.fields.iter() {
            if field.attr.is_updating() {
                has_updates = true;
            } else {
                has_replacements = true;
            }
        }

        if has_updates && has_replacements {
            self.error(&proc_macro::Span::call_site(), Problem::QueryBothUpdateAndRepl);
            Err(self.verdict.borrow().clone())
        } else if has_updates {
            Ok(UpdateType::Update)
        } else if has_replacements {
            Ok(UpdateType::Replacement)
        } else {
            self.error(&proc_macro::Span::call_site(), Problem::QueryEmpty);
            Err(self.verdict.borrow().clone())
        }
    }

    fn verify_attribute(
        &self,
        attr: &SpannedAttribute,
        conversion: Conversion,
    ) -> Result<(), Problem> {
        match conversion {
            Conversion::Replacement | Conversion::Data => {
                if attr.len() == 1 {
                    Ok(())
                } else {
                    Err(Problem::AttrWithDots)
                }
            }
            _ => Ok(()),
        }
    }

    fn convert_query_operator(&self, attr: &SpannedAttribute) -> Option<QueryOperator> {
        let composed = attr.to_composed();
        match composed.as_ref() {
            "$eq" => Some(QueryOperator::Eq),
            "$gt" => Some(QueryOperator::Gt),
            "$gte" => Some(QueryOperator::Gte),
            "$in" => Some(QueryOperator::In),
            "$lt" => Some(QueryOperator::Lt),
            "$lte" => Some(QueryOperator::Lte),
            "$ne" => Some(QueryOperator::Ne),
            "$nin" => Some(QueryOperator::Nin),
            _ => None
        }
    }

    fn convert_update_operator(&self, attr: &SpannedAttribute) -> Option<UpdateOperator> {
        let composed = attr.to_composed();
        match composed.as_ref() {
            "$addToSet" => Some(UpdateOperator::AddToSet),
            "$currentDate" => Some(UpdateOperator::CurrentDate),
            "$inc" => Some(UpdateOperator::Inc),
            "$max" => Some(UpdateOperator::Max),
            "$min" => Some(UpdateOperator::Min),
            "$mul" => Some(UpdateOperator::Mul),
            "$pop" => Some(UpdateOperator::Pop),
            "$pull" => Some(UpdateOperator::Pull),
            "$pullAll" => Some(UpdateOperator::PullAll),
            "$push" => Some(UpdateOperator::Push),
            "$rename" => Some(UpdateOperator::Rename),
            "$set" => Some(UpdateOperator::Set),
            "$setOnInsert" => Some(UpdateOperator::SetOnInsert),
            "$unset" => Some(UpdateOperator::Unset),
            _ => None
        }
    }

    fn convert_object(
        &self,
        struct_spec: &Struct,
        template: ObjectTemplate,
        conversion: Conversion,
    ) -> Object {
        let mut object = Object::new();

        let required_fields = self.prepare_required_members(struct_spec, conversion);
        let mut visited_fields = BTreeSet::new();
        for field in template.fields {
            match self.verify_attribute(&field.attr, conversion) {
                Ok(conversion) => conversion,
                Err(problem) => {
                    self.error(&field.attr.span, problem);
                    continue;
                }
            }

            match self.find_member(struct_spec, field.attr.clone()) {
                Ok(member) => {
                    visited_fields.insert(field.attr.to_composed());
                    match self.convert_value(&member, field.value.value, conversion) {
                        Ok(value) => {
                            let attribute = field.attr.into_attribute();
                            let field = Field::new(attribute, value);
                            object.fields.push(field);
                        }
                        Err(problem) => {
                            self.error(&field.value.span, problem);
                        }
                    }
                }
                Err(problem) => {
                    self.error(&field.attr.span, problem);
                }
            }
        }

        if !required_fields.is_subset(&visited_fields) {
            self.error(&template.span, Problem::FieldsMissing);
        }

        object
    }

    fn convert_filter_object(
        &self,
        builtin: &BuiltInType,
        container: &Container,
        template: ObjectTemplate,
    ) -> Object {
        let mut object = Object::new();

        for field in template.fields {
            let operator = match self.convert_query_operator(&field.attr) {
                Some(operator) => operator,
                None => {
                    self.error(&field.attr.span, Problem::OperatorUnknown);
                    continue;
                }
            };

            if operator.matches(builtin, container) {
                match self.convert_filter_value(operator, builtin, field.value.value) {
                    Ok(value) => {
                        let attribute = field.attr.into_attribute();
                        let field = Field::new(attribute, value);
                        object.fields.push(field);
                    }
                    Err(problem) => {
                        self.error(&field.value.span, problem);
                    }
                }
            } else {
                self.error(&field.attr.span, Problem::OperatorIncorrect);
            }
        }

        object
    }

    fn convert_update_object(
        &self,
        struct_spec: &Struct,
        template: ObjectTemplate,
    ) -> Object {
        let mut object = Object::new();

        for field in template.fields {
            let operator = match self.convert_update_operator(&field.attr) {
                Some(operator) => operator,
                None => {
                    self.error(&field.attr.span, Problem::OperatorUnknown);
                    continue;
                }
            };

            match field.value.value {
                ValueTemplate::Object(obj) => {
                    let obj = self.convert_object(struct_spec, obj, Conversion::Update(operator));
                    let value = Value::Object(obj);
                    let attribute = field.attr.into_attribute();
                    let field = Field::new(attribute, value);
                    object.fields.push(field);
                }
                _ => {
                    self.error(&field.value.span, Problem::ExpObject);
                    continue;
                }
            }
        }

        object
    }

    fn convert_value(
        &self,
        member: &MemberInfo,
        template: ValueTemplate,
        conversion: Conversion,
    ) -> Result<Value, Problem> {
        // In case of `code` mode - the data will be checked at compile time
        if let ValueTemplate::Code(code) = template {
            let cast = member.to_cast(conversion.escapes_container());
            return Ok(Value::Code { code, cast });
        }

        // In case of hard-coded data - try to convert
        match conversion {
            Conversion::Update(op) => self.convert_update(&member, template, op),
            Conversion::Filter => self.convert_filter(&member.info, &member.container, template),
            _ => {
                if member.container.is_plain() {
                    match &member.info {
                        VariantInfo::Field(builtin) => {
                            self.convert_builtin_value(builtin, template)
                        }
                        VariantInfo::Entity(entity) => {
                            self.convert_defined_value(entity, template, conversion)
                        }
                    }
                } else {
                    Err(Problem::ExpCodeComp)
                }
            }
        }
    }

    fn convert_builtin_value(
        &self,
        builtin: &BuiltInType,
        template: ValueTemplate,
    ) -> Result<Value, Problem> {
        match builtin {
            BuiltInType::F64 => match template {
                ValueTemplate::Unquoted(string) => match string.parse() {
                    Ok(value) => Ok(Value::F64(value)),
                    Err(_) => Err(Problem::ExpF64),
                },
                _ => Err(Problem::ExpF64),
            },
            BuiltInType::String => match template {
                ValueTemplate::Quoted(value) => Ok(Value::String(value)),
                _ => Err(Problem::ExpString),
            },
            BuiltInType::ObjectId => match template {
                ValueTemplate::Quoted(string) | ValueTemplate::Unquoted(string) => {
                    match bson::oid::ObjectId::with_string(&string) {
                        Ok(value) => Ok(Value::ObjectId(value)),
                        Err(_) => Err(Problem::ExpOid),
                    }
                }
                _ => Err(Problem::ExpOid),
            },
            BuiltInType::Bool => match template {
                ValueTemplate::Unquoted(string) => {
                    if string == "true" {
                        Ok(Value::Bool(true))
                    } else if string == "false" {
                        Ok(Value::Bool(false))
                    } else {
                        Err(Problem::ExpBool)
                    }
                }
                _ => Err(Problem::ExpBool),
            },
            BuiltInType::Date => match template {
                ValueTemplate::Quoted(string) => match string.parse::<DateTime<Utc>>() {
                    Ok(date) => Ok(Value::Date(date)),
                    Err(_) => Err(Problem::ExpDate),
                },
                _ => Err(Problem::ExpDate),
            },
            BuiltInType::I32 => match template {
                ValueTemplate::Unquoted(string) => match string.parse() {
                    Ok(value) => Ok(Value::I32(value)),
                    Err(_) => Err(Problem::ExpI32),
                },
                _ => Err(Problem::ExpI32),
            },
            BuiltInType::I64 => match template {
                ValueTemplate::Unquoted(string) => match string.parse() {
                    Ok(value) => Ok(Value::I64(value)),
                    Err(_) => Err(Problem::ExpI64),
                },
                _ => Err(Problem::ExpI64),
            },
            BuiltInType::Bson => Err(Problem::ExpBson),
        }
    }

    fn convert_defined_value(
        &self,
        entity: &Entity,
        template: ValueTemplate,
        conversion: Conversion,
    ) -> Result<Value, Problem> {
        match entity {
            Entity::Struct(struct_spec) => match template {
                ValueTemplate::Object(object) => {
                    Ok(Value::Object(self.convert_object(struct_spec, object, conversion)))
                }
                _ => Err(Problem::ExpObject),
            },
            Entity::Enum(_) => Err(Problem::ExpCodeEnum),
            Entity::Union(_) => Err(Problem::ExpCodeUnion),
        }
    }

    fn convert_filter(
        &self,
        variant: &VariantInfo,
        container: &Container,
        template: ValueTemplate,
    ) -> Result<Value, Problem> {
        match variant {
            VariantInfo::Field(builtin) => {
                match template {
                    ValueTemplate::Object(object) => {
                        Ok(Value::Object(self.convert_filter_object(builtin, container, object)))
                    }
                    _ => {
                        if container.is_plain() {
                            self.convert_builtin_value(builtin, template)
                        } else {
                            return Err(Problem::ExpCodeComp);
                        }
                    }
                }
            }
            VariantInfo::Entity(entity) => {
                self.convert_defined_value(entity, template, Conversion::Filter)
            }
        }
    }

    fn convert_update(
        &self,
        member: &MemberInfo,
        template: ValueTemplate,
        operator: UpdateOperator,
    ) -> Result<Value, Problem> {
        match operator {
            UpdateOperator::Inc |
            UpdateOperator::Min |
            UpdateOperator::Max |
            UpdateOperator::Mul |
            UpdateOperator::Set |
            UpdateOperator::SetOnInsert => {
                if member.container.is_plain() {
                    match &member.info {
                        VariantInfo::Field(builtin) => {
                            self.convert_builtin_value(builtin, template)
                        }
                        VariantInfo::Entity(entity) => {
                            self.convert_defined_value(entity, template, Conversion::Data)
                        }
                    }
                } else {
                    Err(Problem::ExpPlain)
                }
            }
            UpdateOperator::CurrentDate => {
                if member.container.is_plain() {
                    match &member.info {
                        VariantInfo::Field(BuiltInType::Date) => self.convert_date_value(template),
                        _ => Err(Problem::OperatorIncorrect)
                    }
                } else {
                    Err(Problem::ExpPlain)
                }
            }
            UpdateOperator::AddToSet |
            UpdateOperator::Pop |
            UpdateOperator::Push => {
                if member.container.is_array() {
                    match &member.info {
                        VariantInfo::Field(builtin) => {
                            self.convert_builtin_value(builtin, template)
                        }
                        VariantInfo::Entity(entity) => {
                            self.convert_defined_value(entity, template, Conversion::Data)
                        }
                    }
                } else {
                    Err(Problem::OperatorIncorrect)
                }
            }
            UpdateOperator::Pull => {
                if member.container.is_array() {
                    self.convert_filter(&member.info, &Container::Plain, template)
                } else {
                    Err(Problem::ExpArray)
                }
            }
            UpdateOperator::PullAll => {
                Err(Problem::ExpCode)
            }
            UpdateOperator::Unset => {
                if template.is_empty_string() {
                    Ok(Value::String(String::new()))
                } else {
                    Err(Problem::ExpEmptyString)
                }
            }
            UpdateOperator::Rename => {
                match template {
                    ValueTemplate::Quoted(string) => Ok(Value::String(string)),
                    _ => Err(Problem::ExpString)
                }
            }
        }
    }

    fn convert_filter_value(
        &self,
        operator: QueryOperator,
        builtin: &BuiltInType,
        value: ValueTemplate,
    ) -> Result<Value, Problem> {
        match operator {
            QueryOperator::Eq |
            QueryOperator::Ne |
            QueryOperator::Gt |
            QueryOperator::Gte |
            QueryOperator::Lt |
            QueryOperator::Lte => {
                if let ValueTemplate::Code(code) = value {
                    Ok(Value::new_builtin_code(builtin.clone(), Container::Plain, code))
                } else {
                    self.convert_builtin_value(builtin, value)
                }
            }
            QueryOperator::In |
            QueryOperator::Nin => {
                if let ValueTemplate::Code(code) = value {
                    Ok(Value::new_builtin_code(builtin.clone(), Container::Array, code))
                } else {
                    Err(Problem::ExpCode)
                }
            }
        }
    }

    fn convert_date_value(
        &self,
        template: ValueTemplate,
    ) -> Result<Value, Problem> {
        match template {
            ValueTemplate::Unquoted(string) => {
                if string == "true" {
                    return Ok(Value::Bool(true))
                }
            }
            ValueTemplate::Object(object) => {
                if object.fields.len() == 1 {
                    let field = object.fields.first().unwrap();
                    if field.attr.to_composed() == "$type" {
                        match &field.value.value {
                            ValueTemplate::Quoted(string) => {
                                if (string == "timestamp") || (string == "datetime") {
                                    return Ok(Value::String(string.clone()))
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
            _ => {}
        }
        Err(Problem::ExpDateObj)
    }
}

// -------------------------------------------------------------------------------------------------
// Other helpers

impl<'a> Validator<'a> {
    fn make_generator(&self, name: DefinedType, object: Object) -> Result<Generator, Verdict> {
        if object.fields.len() == 0  {
            self.error(&proc_macro::Span::call_site(), Problem::MacroEmpty);
        }

        if self.verdict.borrow().problems.len() == 0  {
            Ok(Generator::new(name, object))
        } else {
            Err(self.verdict.borrow().clone())
        }
    }

    fn prepare_required_members(
        &self,
        struct_spec: &Struct,
        conversion: Conversion,
    ) -> BTreeSet<String> {
        let mut fields = BTreeSet::new();
        match conversion {
            Conversion::Data => {
                for member in struct_spec.members.iter() {
                    if (!member.is_optional) && (!member.container.is_array()) {
                        fields.insert(member.db_name.clone());
                    }
                }
            }
            _ => {}
        }
        fields
    }

    fn error(&self, span: &proc_macro::Span, problem: Problem) {
        self.verdict.borrow_mut().problems.push(problem);
        if !self.testing {
            span.error(problem.as_str()).emit();
        }
    }
}
