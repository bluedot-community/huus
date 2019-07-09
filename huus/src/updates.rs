// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Contains functionalities for update versions of `huus` structures.

use std::collections::{BTreeMap, HashMap};

use crate::conversions::{HuusEnum, HuusIntoBson};
use crate::values::BuildValue;
use crate::{types, values};

// -------------------------------------------------------------------------------------------------

pub trait BuildUpdate {
    fn build_update(self) -> Update;
}

pub trait BuildInnerUpdate {
    fn build_update(self, field: String) -> Update;
}

// -------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub enum PopOption {
    First,
    Last,
}

impl PopOption {
    fn as_number(&self) -> i32 {
        match self {
            PopOption::First => -1,
            PopOption::Last => 1,
        }
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub enum Operator {
    None,
    First,
}

impl Operator {
    fn to_string(&self) -> &'static str {
        match self {
            Operator::None => "",
            Operator::First => ".$",
        }
    }
}

// -------------------------------------------------------------------------------------------------

pub trait ObjectUpdate<U, V>
where
    U: BuildInnerUpdate,
    V: BuildValue,
{
    fn value(&mut self, value: V);
    fn dot(&mut self, update: U);
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

pub trait NumericalUpdate<V>
where
    V: BuildValue,
{
    fn inc(&mut self, value: V);
    fn min(&mut self, value: V);
    fn max(&mut self, value: V);
    fn mul(&mut self, value: V);
}

#[derive(Clone, Debug)]
pub enum Numerical<V>
where
    V: BuildValue,
{
    Inc(V),
    Min(V),
    Max(V),
    Mul(V),
}

impl<V> BuildInnerUpdate for Numerical<V>
where
    V: BuildValue,
{
    fn build_update(self, field: String) -> Update {
        match self {
            Numerical::Inc(value) => {
                Update::with_operator(UpdateOperator::Inc, field, value.build_value())
            }
            Numerical::Min(value) => {
                Update::with_operator(UpdateOperator::Min, field, value.build_value())
            }
            Numerical::Max(value) => {
                Update::with_operator(UpdateOperator::Max, field, value.build_value())
            }
            Numerical::Mul(value) => {
                Update::with_operator(UpdateOperator::Mul, field, value.build_value())
            }
        }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

pub trait FieldUpdate<V>
where
    V: BuildValue,
{
    fn rename(&mut self, new_name: String);
    fn set(&mut self, value: V);
    fn set_on_insert(&mut self, value: V);
    fn unset(&mut self);
}

#[derive(Clone, Debug)]
pub enum Field<V>
where
    V: BuildValue,
{
    Rename(String),
    Set(V),
    SetOnInsert(V),
    Unset,
}

impl<V> BuildInnerUpdate for Field<V>
where
    V: BuildValue,
{
    fn build_update(self, field: String) -> Update {
        match self {
            Field::Rename(value) => {
                Update::with_operator(UpdateOperator::Rename, field, value.build_value())
            }
            Field::Set(value) => {
                Update::with_operator(UpdateOperator::Set, field, value.build_value())
            }
            Field::SetOnInsert(value) => {
                Update::with_operator(UpdateOperator::SetOnInsert, field, value.build_value())
            }
            Field::Unset => Update::with_operator(UpdateOperator::Unset, field, true.build_value()),
        }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

pub trait ElementUpdate<V>
where
    V: BuildValue,
{
    fn set(&mut self, value: V, operator: Operator);
}

#[derive(Clone, Debug)]
pub enum Element<V>
where
    V: BuildValue,
{
    Set(V),
}

impl<V> BuildInnerUpdate for Element<V>
where
    V: BuildValue,
{
    fn build_update(self, field: String) -> Update {
        match self {
            Element::Set(value) => {
                Update::with_operator(UpdateOperator::Set, field, value.build_value())
            }
        }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

pub trait ArrayUpdate<V>
where
    V: BuildValue,
{
    fn add_to_set(&mut self, value: V, operator: Operator);
    fn pop(&mut self, option: PopOption, operator: Operator);
    fn pull(&mut self, value: V, operator: Operator);
    fn push(&mut self, value: V, operator: Operator);
    fn pull_all(&mut self, value: V, operator: Operator);
}

#[derive(Clone, Debug)]
pub enum Array<V>
where
    V: BuildValue,
{
    AddToSet(V),
    Pop(PopOption),
    Pull(V),
    Push(V),
    PullAll(V),
}

impl<V> BuildInnerUpdate for Array<V>
where
    V: BuildValue,
{
    fn build_update(self, field: String) -> Update {
        match self {
            Array::AddToSet(value) => {
                Update::with_operator(UpdateOperator::AddToSet, field, value.build_value())
            }
            Array::Pop(option) => {
                Update::with_operator(UpdateOperator::Pop, field, option.as_number().build_value())
            }
            Array::Pull(value) => {
                Update::with_operator(UpdateOperator::Pull, field, value.build_value())
            }
            Array::Push(value) => {
                Update::with_operator(UpdateOperator::Push, field, value.build_value())
            }
            Array::PullAll(value) => {
                Update::with_operator(UpdateOperator::PullAll, field, value.build_value())
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub enum F64Entry {
    Value(types::Double),
    Numerical(Numerical<types::Double>),
    Field(Field<types::Double>),
    Empty,
}

impl BuildInnerUpdate for F64Entry {
    fn build_update(self, field: String) -> Update {
        match self {
            F64Entry::Value(value) => Update::with_field(field, bson::Bson::FloatingPoint(value)),
            F64Entry::Numerical(value) => value.build_update(field),
            F64Entry::Field(value) => value.build_update(field),
            F64Entry::Empty => Update::empty(),
        }
    }
}

impl Default for F64Entry {
    fn default() -> Self {
        F64Entry::Empty
    }
}

impl std::convert::From<f64> for F64Entry {
    fn from(value: f64) -> F64Entry {
        F64Entry::Value(value)
    }
}

impl std::convert::From<f32> for F64Entry {
    fn from(value: f32) -> F64Entry {
        F64Entry::Value(value as f64)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Clone, Debug)]
pub enum StringEntry {
    Value(String),
    Field(Field<String>),
    Empty,
}

impl BuildInnerUpdate for StringEntry {
    fn build_update(self, field: String) -> Update {
        match self {
            StringEntry::Value(value) => Update::with_field(field, bson::Bson::String(value)),
            StringEntry::Field(value) => value.build_update(field),
            StringEntry::Empty => Update::empty(),
        }
    }
}

impl Default for StringEntry {
    fn default() -> Self {
        StringEntry::Empty
    }
}

impl std::convert::From<&str> for StringEntry {
    fn from(value: &str) -> StringEntry {
        StringEntry::Value(value.to_string())
    }
}

impl std::convert::From<String> for StringEntry {
    fn from(value: String) -> StringEntry {
        StringEntry::Value(value)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Clone, Debug)]
pub enum EnumEntry<E>
where
    E: HuusEnum,
{
    Value(E),
    Field(Field<String>),
    Empty,
}

impl<E> FieldUpdate<String> for EnumEntry<E>
where
    E: HuusEnum,
{
    fn rename(&mut self, new_name: String) {
        *self = EnumEntry::Field(Field::Rename(new_name));
    }

    fn set(&mut self, value: String) {
        *self = EnumEntry::Field(Field::Set(value));
    }

    fn set_on_insert(&mut self, value: String) {
        *self = EnumEntry::Field(Field::SetOnInsert(value));
    }

    fn unset(&mut self) {
        *self = EnumEntry::Field(Field::Unset);
    }
}

impl<E> BuildInnerUpdate for EnumEntry<E>
where
    E: HuusEnum,
{
    fn build_update(self, field: String) -> Update {
        match self {
            EnumEntry::Value(value) => {
                Update::with_field(field, bson::Bson::String(value.to_str().to_string()))
            }
            EnumEntry::Field(value) => value.build_update(field),
            EnumEntry::Empty => Update::empty(),
        }
    }
}

impl<E> Default for EnumEntry<E>
where
    E: HuusEnum,
{
    fn default() -> Self {
        EnumEntry::Empty
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Clone, Debug)]
pub enum ObjectEntry<U, V>
where
    U: BuildInnerUpdate,
    V: BuildValue,
{
    Value(V),
    Dot(U),
    Field(Field<V>),
    Empty,
}

impl<U, V> ObjectUpdate<U, V> for ObjectEntry<U, V>
where
    U: BuildInnerUpdate,
    V: BuildValue,
{
    fn value(&mut self, value: V) {
        *self = ObjectEntry::Value(value);
    }

    fn dot(&mut self, update: U) {
        *self = ObjectEntry::Dot(update);
    }
}

impl<U, V> FieldUpdate<V> for ObjectEntry<U, V>
where
    U: BuildInnerUpdate,
    V: BuildValue,
{
    fn rename(&mut self, new_name: String) {
        *self = ObjectEntry::Field(Field::Rename(new_name));
    }

    fn set(&mut self, value: V) {
        *self = ObjectEntry::Field(Field::Set(value));
    }

    fn set_on_insert(&mut self, value: V) {
        *self = ObjectEntry::Field(Field::SetOnInsert(value));
    }

    fn unset(&mut self) {
        *self = ObjectEntry::Field(Field::Unset);
    }
}

impl<U, V> BuildInnerUpdate for ObjectEntry<U, V>
where
    U: BuildInnerUpdate,
    V: BuildValue,
{
    fn build_update(self, field: String) -> Update {
        match self {
            ObjectEntry::Value(value) => Update::with_field(field, value.build_value().into_bson()),
            ObjectEntry::Dot(update) => update.build_update(field),
            ObjectEntry::Field(update) => update.build_update(field),
            ObjectEntry::Empty => Update::empty(),
        }
    }
}

impl<U, V> Default for ObjectEntry<U, V>
where
    U: BuildInnerUpdate,
    V: BuildValue,
{
    fn default() -> Self {
        ObjectEntry::Empty
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Clone, Debug)]
pub enum HashMapEntry<E, B>
where
    E: HuusEnum,
    B: HuusIntoBson,
{
    Value(HashMap<E, B>),
    Field(Field<HashMap<E, B>>),
    Empty,
}

impl<E, B> FieldUpdate<HashMap<E, B>> for HashMapEntry<E, B>
where
    E: HuusEnum,
    B: HuusIntoBson,
{
    fn rename(&mut self, new_name: String) {
        *self = HashMapEntry::Field(Field::Rename(new_name));
    }

    fn set(&mut self, value: HashMap<E, B>) {
        *self = HashMapEntry::Field(Field::Set(value));
    }

    fn set_on_insert(&mut self, value: HashMap<E, B>) {
        *self = HashMapEntry::Field(Field::SetOnInsert(value));
    }

    fn unset(&mut self) {
        *self = HashMapEntry::Field(Field::Unset);
    }
}

impl<E, B> BuildInnerUpdate for HashMapEntry<E, B>
where
    E: HuusEnum,
    B: HuusIntoBson,
{
    fn build_update(self, field: String) -> Update {
        match self {
            HashMapEntry::Value(value) => Update::with_field(field, value.huus_into_bson()),
            HashMapEntry::Field(update) => update.build_update(field),
            HashMapEntry::Empty => Update::empty(),
        }
    }
}

impl<E, B> Default for HashMapEntry<E, B>
where
    E: HuusEnum,
    B: HuusIntoBson,
{
    fn default() -> Self {
        HashMapEntry::Empty
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Clone, Debug)]
pub enum BTreeMapEntry<E, B>
where
    E: HuusEnum,
    B: HuusIntoBson,
{
    Value(BTreeMap<E, B>),
    Field(Field<BTreeMap<E, B>>),
    Empty,
}

impl<E, B> FieldUpdate<BTreeMap<E, B>> for BTreeMapEntry<E, B>
where
    E: HuusEnum,
    B: HuusIntoBson,
{
    fn rename(&mut self, new_name: String) {
        *self = BTreeMapEntry::Field(Field::Rename(new_name));
    }

    fn set(&mut self, value: BTreeMap<E, B>) {
        *self = BTreeMapEntry::Field(Field::Set(value));
    }

    fn set_on_insert(&mut self, value: BTreeMap<E, B>) {
        *self = BTreeMapEntry::Field(Field::SetOnInsert(value));
    }

    fn unset(&mut self) {
        *self = BTreeMapEntry::Field(Field::Unset);
    }
}

impl<E, B> BuildInnerUpdate for BTreeMapEntry<E, B>
where
    E: HuusEnum,
    B: HuusIntoBson,
{
    fn build_update(self, field: String) -> Update {
        match self {
            BTreeMapEntry::Value(value) => Update::with_field(field, value.huus_into_bson()),
            BTreeMapEntry::Field(update) => update.build_update(field),
            BTreeMapEntry::Empty => Update::empty(),
        }
    }
}

impl<E, B> Default for BTreeMapEntry<E, B>
where
    E: HuusEnum,
    B: HuusIntoBson,
{
    fn default() -> Self {
        BTreeMapEntry::Empty
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Clone, Debug)]
pub enum ArrayEntry<V>
where
    V: BuildValue,
{
    Array(Array<V>, Operator),
    Numerical(Numerical<V>),
    Element(Element<V>, Operator),
    Empty,
}

impl<V> ArrayUpdate<V> for ArrayEntry<V>
where
    V: BuildValue,
{
    fn add_to_set(&mut self, value: V, operator: Operator) {
        *self = ArrayEntry::Array(Array::AddToSet(value), operator);
    }

    fn pop(&mut self, option: PopOption, operator: Operator) {
        *self = ArrayEntry::Array(Array::Pop(option), operator);
    }

    fn pull(&mut self, value: V, operator: Operator) {
        *self = ArrayEntry::Array(Array::Pull(value), operator);
    }

    fn push(&mut self, value: V, operator: Operator) {
        *self = ArrayEntry::Array(Array::Push(value), operator);
    }

    fn pull_all(&mut self, value: V, operator: Operator) {
        *self = ArrayEntry::Array(Array::PullAll(value), operator);
    }
}

impl<V> ElementUpdate<V> for ArrayEntry<V>
where
    V: BuildValue,
{
    fn set(&mut self, value: V, operator: Operator) {
        *self = ArrayEntry::Element(Element::Set(value), operator);
    }
}

impl<V> BuildInnerUpdate for ArrayEntry<V>
where
    V: BuildValue,
{
    fn build_update(self, field: String) -> Update {
        match self {
            ArrayEntry::Array(operation, operator) => {
                operation.build_update(field + operator.to_string())
            }
            ArrayEntry::Numerical(operation) => operation.build_update(field),
            ArrayEntry::Element(operation, operator) => {
                operation.build_update(field + operator.to_string())
            }
            ArrayEntry::Empty => Update::empty(),
        }
    }
}

impl<V> Default for ArrayEntry<V>
where
    V: BuildValue,
{
    fn default() -> Self {
        ArrayEntry::Empty
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Clone, Debug)]
pub enum ObjectIdEntry {
    Value(types::ObjectId),
    Field(Field<types::ObjectId>),
    Empty,
}

impl FieldUpdate<bson::oid::ObjectId> for ObjectIdEntry {
    fn rename(&mut self, new_name: String) {
        *self = ObjectIdEntry::Field(Field::Rename(new_name));
    }

    fn set(&mut self, value: bson::oid::ObjectId) {
        *self = ObjectIdEntry::Field(Field::Set(value));
    }

    fn set_on_insert(&mut self, value: bson::oid::ObjectId) {
        *self = ObjectIdEntry::Field(Field::SetOnInsert(value));
    }

    fn unset(&mut self) {
        *self = ObjectIdEntry::Field(Field::Unset);
    }
}

impl BuildInnerUpdate for ObjectIdEntry {
    fn build_update(self, field: String) -> Update {
        match self {
            ObjectIdEntry::Value(value) => Update::with_field(field, bson::Bson::ObjectId(value)),
            ObjectIdEntry::Field(value) => value.build_update(field),
            ObjectIdEntry::Empty => Update::empty(),
        }
    }
}

impl Default for ObjectIdEntry {
    fn default() -> Self {
        ObjectIdEntry::Empty
    }
}

impl std::convert::From<types::ObjectId> for ObjectIdEntry {
    fn from(value: types::ObjectId) -> ObjectIdEntry {
        ObjectIdEntry::Value(value)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Clone, Debug)]
pub enum BooleanEntry {
    Value(bool),
    Field(Field<bool>),
    Empty,
}

impl FieldUpdate<bool> for BooleanEntry {
    fn rename(&mut self, new_name: String) {
        *self = BooleanEntry::Field(Field::Rename(new_name));
    }

    fn set(&mut self, value: bool) {
        *self = BooleanEntry::Field(Field::Set(value));
    }

    fn set_on_insert(&mut self, value: bool) {
        *self = BooleanEntry::Field(Field::SetOnInsert(value));
    }

    fn unset(&mut self) {
        *self = BooleanEntry::Field(Field::Unset);
    }
}

impl BuildInnerUpdate for BooleanEntry {
    fn build_update(self, field: String) -> Update {
        match self {
            BooleanEntry::Value(value) => Update::with_field(field, bson::Bson::Boolean(value)),
            BooleanEntry::Field(value) => value.build_update(field),
            BooleanEntry::Empty => Update::empty(),
        }
    }
}

impl Default for BooleanEntry {
    fn default() -> Self {
        BooleanEntry::Empty
    }
}

impl std::convert::From<bool> for BooleanEntry {
    fn from(value: bool) -> BooleanEntry {
        BooleanEntry::Value(value)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Clone, Debug)]
pub enum DateEntry {
    Value(types::Date),
    CurrentDate,
    Field(Field<types::Date>),
    Empty,
}

impl FieldUpdate<types::Date> for DateEntry {
    fn rename(&mut self, new_name: String) {
        *self = DateEntry::Field(Field::Rename(new_name));
    }

    fn set(&mut self, value: types::Date) {
        *self = DateEntry::Field(Field::Set(value));
    }

    fn set_on_insert(&mut self, value: types::Date) {
        *self = DateEntry::Field(Field::SetOnInsert(value));
    }

    fn unset(&mut self) {
        *self = DateEntry::Field(Field::Unset);
    }
}

impl BuildInnerUpdate for DateEntry {
    fn build_update(self, field: String) -> Update {
        match self {
            DateEntry::Value(value) => Update::with_field(field, bson::Bson::UtcDatetime(value)),
            DateEntry::CurrentDate => {
                let value = "date".to_string().build_value();
                Update::with_operator(UpdateOperator::CurrentDate, field, value)
            }
            DateEntry::Field(value) => value.build_update(field),
            DateEntry::Empty => Update::empty(),
        }
    }
}

impl Default for DateEntry {
    fn default() -> Self {
        DateEntry::Empty
    }
}

impl std::convert::From<types::Date> for DateEntry {
    fn from(value: types::Date) -> DateEntry {
        DateEntry::Value(value)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Clone, Debug)]
pub enum I32Entry {
    Value(i32),
    Numerical(Numerical<i32>),
    Field(Field<i32>),
    Empty,
}

impl NumericalUpdate<i32> for I32Entry {
    fn inc(&mut self, value: i32) {
        *self = I32Entry::Numerical(Numerical::Inc(value));
    }

    fn min(&mut self, value: i32) {
        *self = I32Entry::Numerical(Numerical::Min(value));
    }

    fn max(&mut self, value: i32) {
        *self = I32Entry::Numerical(Numerical::Max(value));
    }

    fn mul(&mut self, value: i32) {
        *self = I32Entry::Numerical(Numerical::Mul(value));
    }
}

impl BuildInnerUpdate for I32Entry {
    fn build_update(self, field: String) -> Update {
        match self {
            I32Entry::Value(value) => Update::with_field(field, bson::Bson::I32(value)),
            I32Entry::Numerical(value) => value.build_update(field),
            I32Entry::Field(value) => value.build_update(field),
            I32Entry::Empty => Update::empty(),
        }
    }
}

impl Default for I32Entry {
    fn default() -> Self {
        I32Entry::Empty
    }
}

impl std::convert::From<i32> for I32Entry {
    fn from(value: i32) -> I32Entry {
        I32Entry::Value(value)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Clone, Debug)]
pub enum TimeStampEntry {
    Value(types::TimeStamp),
    CurrentDate,
    Field(Field<types::TimeStamp>),
    Empty,
}

impl BuildInnerUpdate for TimeStampEntry {
    fn build_update(self, field: String) -> Update {
        match self {
            TimeStampEntry::Value(value) => Update::with_field(field, value.huus_into_bson()),
            TimeStampEntry::CurrentDate => {
                let value = "timestamp".to_string().build_value();
                Update::with_operator(UpdateOperator::CurrentDate, field, value)
            }
            TimeStampEntry::Field(value) => value.build_update(field),
            TimeStampEntry::Empty => Update::empty(),
        }
    }
}

impl Default for TimeStampEntry {
    fn default() -> Self {
        TimeStampEntry::Empty
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Clone, Debug)]
pub enum I64Entry {
    Value(i64),
    Numerical(Numerical<i64>),
    Field(Field<i64>),
    Empty,
}

impl BuildInnerUpdate for I64Entry {
    fn build_update(self, field: String) -> Update {
        match self {
            I64Entry::Value(value) => Update::with_field(field, bson::Bson::I64(value)),
            I64Entry::Numerical(value) => value.build_update(field),
            I64Entry::Field(value) => value.build_update(field),
            I64Entry::Empty => Update::empty(),
        }
    }
}

impl Default for I64Entry {
    fn default() -> Self {
        I64Entry::Empty
    }
}

impl std::convert::From<i64> for I64Entry {
    fn from(value: i64) -> I64Entry {
        I64Entry::Value(value)
    }
}

// -------------------------------------------------------------------------------------------------

enum UpdateOperator {
    Inc,
    Min,
    Max,
    Mul,
    Rename,
    Set,
    SetOnInsert,
    Unset,
    AddToSet,
    Pop,
    Pull,
    Push,
    PullAll,
    CurrentDate,
}

impl UpdateOperator {
    fn to_string(&self) -> &'static str {
        match self {
            UpdateOperator::Inc => "$inc",
            UpdateOperator::Min => "$min",
            UpdateOperator::Max => "$max",
            UpdateOperator::Mul => "$mul",
            UpdateOperator::Rename => "$rename",
            UpdateOperator::Set => "$set",
            UpdateOperator::SetOnInsert => "$setOnInsert",
            UpdateOperator::Unset => "$unset",
            UpdateOperator::AddToSet => "$addToSet",
            UpdateOperator::Pop => "$pop",
            UpdateOperator::Pull => "$pull",
            UpdateOperator::Push => "$push",
            UpdateOperator::PullAll => "$pullAll",
            UpdateOperator::CurrentDate => "$currentDate",
        }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Debug)]
struct UpdateInstruction {
    path: Vec<String>,
    value: values::Value,
}

impl UpdateInstruction {
    fn new(field: String, value: values::Value) -> Self {
        Self { path: vec![field], value: value }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Debug)]
pub struct Update {
    doc: bson::Document,
    inc_instructions: Vec<UpdateInstruction>,
    min_instructions: Vec<UpdateInstruction>,
    max_instructions: Vec<UpdateInstruction>,
    mul_instructions: Vec<UpdateInstruction>,
    rename_instructions: Vec<UpdateInstruction>,
    set_instructions: Vec<UpdateInstruction>,
    set_on_insert_instructions: Vec<UpdateInstruction>,
    unset_instructions: Vec<UpdateInstruction>,
    add_to_set_instructions: Vec<UpdateInstruction>,
    pop_instructions: Vec<UpdateInstruction>,
    pull_instructions: Vec<UpdateInstruction>,
    push_instructions: Vec<UpdateInstruction>,
    pull_all_instructions: Vec<UpdateInstruction>,
    current_date_instructions: Vec<UpdateInstruction>,
}

impl Update {
    pub fn empty() -> Self {
        Self {
            doc: bson::Document::new(),
            inc_instructions: Vec::new(),
            min_instructions: Vec::new(),
            max_instructions: Vec::new(),
            mul_instructions: Vec::new(),
            rename_instructions: Vec::new(),
            set_instructions: Vec::new(),
            set_on_insert_instructions: Vec::new(),
            unset_instructions: Vec::new(),
            add_to_set_instructions: Vec::new(),
            pop_instructions: Vec::new(),
            pull_instructions: Vec::new(),
            push_instructions: Vec::new(),
            pull_all_instructions: Vec::new(),
            current_date_instructions: Vec::new(),
        }
    }

    pub fn with_field(field: String, value: bson::Bson) -> Self {
        let mut update = Update::empty();
        update.doc.insert(field, value);
        update
    }

    fn with_operator(operator: UpdateOperator, field: String, value: values::Value) -> Self {
        let mut update = Update::empty();
        let instruction = UpdateInstruction::new(field, value);
        match operator {
            UpdateOperator::Inc => update.inc_instructions.push(instruction),
            UpdateOperator::Min => update.min_instructions.push(instruction),
            UpdateOperator::Max => update.max_instructions.push(instruction),
            UpdateOperator::Mul => update.mul_instructions.push(instruction),
            UpdateOperator::Rename => update.rename_instructions.push(instruction),
            UpdateOperator::Set => update.set_instructions.push(instruction),
            UpdateOperator::SetOnInsert => update.set_on_insert_instructions.push(instruction),
            UpdateOperator::Unset => update.unset_instructions.push(instruction),
            UpdateOperator::AddToSet => update.add_to_set_instructions.push(instruction),
            UpdateOperator::Pop => update.pop_instructions.push(instruction),
            UpdateOperator::Pull => update.pull_instructions.push(instruction),
            UpdateOperator::Push => update.push_instructions.push(instruction),
            UpdateOperator::PullAll => update.pull_all_instructions.push(instruction),
            UpdateOperator::CurrentDate => update.current_date_instructions.push(instruction),
        }
        update
    }

    pub fn incorporate(&mut self, update: Update) {
        fn incorporate(source: Vec<UpdateInstruction>, target: &mut Vec<UpdateInstruction>) {
            for instruction in source {
                target.push(instruction);
            }
        }

        for (key, value) in update.doc {
            self.doc.insert_bson(key, value);
        }

        incorporate(update.inc_instructions, &mut self.inc_instructions);
        incorporate(update.min_instructions, &mut self.min_instructions);
        incorporate(update.max_instructions, &mut self.max_instructions);
        incorporate(update.mul_instructions, &mut self.mul_instructions);
        incorporate(update.rename_instructions, &mut self.rename_instructions);
        incorporate(update.set_instructions, &mut self.set_instructions);
        incorporate(update.set_on_insert_instructions, &mut self.set_on_insert_instructions);
        incorporate(update.unset_instructions, &mut self.unset_instructions);
        incorporate(update.add_to_set_instructions, &mut self.add_to_set_instructions);
        incorporate(update.pop_instructions, &mut self.pop_instructions);
        incorporate(update.pull_instructions, &mut self.pull_instructions);
        incorporate(update.push_instructions, &mut self.push_instructions);
        incorporate(update.pull_all_instructions, &mut self.pull_all_instructions);
        incorporate(update.current_date_instructions, &mut self.current_date_instructions);
    }

    pub fn into_doc(self) -> bson::Document {
        fn build(
            result: &mut bson::Document,
            operator: &'static str,
            instructions: Vec<UpdateInstruction>,
        ) {
            if !instructions.is_empty() {
                let mut bson = bson::Document::new();
                for instruction in instructions.iter().rev() {
                    let mut path = String::new();
                    for field in &instruction.path {
                        path += &field;
                        path += ".";
                    }
                    path.pop();
                    bson.insert(path, instruction.value.clone().into_bson());
                }
                result.insert(operator, bson);
            }
        }

        let mut res = self.doc.clone();
        build(&mut res, UpdateOperator::Inc.to_string(), self.inc_instructions);
        build(&mut res, UpdateOperator::Min.to_string(), self.min_instructions);
        build(&mut res, UpdateOperator::Max.to_string(), self.max_instructions);
        build(&mut res, UpdateOperator::Mul.to_string(), self.mul_instructions);
        build(&mut res, UpdateOperator::Rename.to_string(), self.rename_instructions);
        build(&mut res, UpdateOperator::Set.to_string(), self.set_instructions);
        build(&mut res, UpdateOperator::SetOnInsert.to_string(), self.set_on_insert_instructions);
        build(&mut res, UpdateOperator::Unset.to_string(), self.unset_instructions);
        build(&mut res, UpdateOperator::AddToSet.to_string(), self.add_to_set_instructions);
        build(&mut res, UpdateOperator::Pop.to_string(), self.pop_instructions);
        build(&mut res, UpdateOperator::Pull.to_string(), self.pull_instructions);
        build(&mut res, UpdateOperator::Push.to_string(), self.push_instructions);
        build(&mut res, UpdateOperator::PullAll.to_string(), self.pull_all_instructions);
        build(&mut res, UpdateOperator::CurrentDate.to_string(), self.current_date_instructions);
        res
    }
}

impl From<Update> for bson::Bson {
    fn from(update: Update) -> bson::Bson {
        bson::Bson::Document(update.into_doc())
    }
}
