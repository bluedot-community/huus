// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Contains functionalities for filter versions of `huus` structures.

use std::collections::{BTreeMap, HashMap};

use bson::{bson, doc};

use crate::conversions::{HuusIntoBson, HuusKey};
use crate::types;

// -------------------------------------------------------------------------------------------------

pub trait BuildFilter {
    fn build_filter(self) -> Filter;
}

pub trait BuildInnerFilter {
    fn build_filter(self, filter: String) -> Filter;
}

// -------------------------------------------------------------------------------------------------

impl BuildInnerFilter for types::Double {
    fn build_filter(self, field: String) -> Filter {
        Filter::with_field(field, bson::Bson::FloatingPoint(self))
    }
}

impl BuildInnerFilter for f32 {
    fn build_filter(self, field: String) -> Filter {
        Filter::with_field(field, bson::Bson::FloatingPoint(self as f64))
    }
}

impl<'a> BuildInnerFilter for &'a str {
    fn build_filter(self, field: String) -> Filter {
        Filter::with_field(field, bson::Bson::String(self.to_string()))
    }
}

impl BuildInnerFilter for String {
    fn build_filter(self, field: String) -> Filter {
        Filter::with_field(field, bson::Bson::String(self))
    }
}

impl BuildInnerFilter for types::ObjectId {
    fn build_filter(self, field: String) -> Filter {
        Filter::with_field(field, bson::Bson::ObjectId(self))
    }
}

impl BuildInnerFilter for bool {
    fn build_filter(self, field: String) -> Filter {
        Filter::with_field(field, bson::Bson::Boolean(self))
    }
}

impl BuildInnerFilter for types::Date {
    fn build_filter(self, field: String) -> Filter {
        Filter::with_field(field, bson::Bson::UtcDatetime(self))
    }
}

impl BuildInnerFilter for i32 {
    fn build_filter(self, field: String) -> Filter {
        Filter::with_field(field, bson::Bson::I32(self))
    }
}

impl BuildInnerFilter for types::TimeStamp {
    fn build_filter(self, field: String) -> Filter {
        Filter::with_field(field, self.huus_into_bson())
    }
}

impl BuildInnerFilter for i64 {
    fn build_filter(self, field: String) -> Filter {
        Filter::with_field(field, bson::Bson::I64(self))
    }
}

// -------------------------------------------------------------------------------------------------

fn vec_into_array<B>(elements: Vec<B>) -> bson::Array
where
    B: HuusIntoBson,
{
    let mut result: Vec<bson::Bson> = Vec::with_capacity(elements.len());
    for element in elements {
        result.push(element.huus_into_bson());
    }
    result
}

fn inner_filters_into_array<F>(elements: Vec<F>, field: String) -> bson::Bson
where
    F: BuildInnerFilter,
{
    let mut result: Vec<bson::Bson> = Vec::with_capacity(elements.len());
    for element in elements {
        result.push(element.build_filter(field.clone()).into_bson());
    }
    bson::Bson::Array(result)
}

fn filters_into_array<F>(elements: Vec<F>) -> bson::Bson
where
    F: BuildFilter,
{
    let mut result: Vec<bson::Bson> = Vec::with_capacity(elements.len());
    for element in elements {
        result.push(element.build_filter().into_bson());
    }
    bson::Bson::Array(result)
}

// -------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub enum Filters<F>
where
    F: BuildFilter,
{
    And(Vec<F>),
    Not(F),
    Nor(Vec<F>),
    Or(Vec<F>),
}

impl<F> BuildFilter for Filters<F>
where
    F: BuildFilter,
{
    fn build_filter(self) -> Filter {
        match self {
            Filters::And(filters) => {
                Filter::with_field("$and".to_string(), filters_into_array(filters))
            }
            Filters::Not(filter) => {
                Filter::with_field("$not".to_string(), filter.build_filter().into_bson())
            }
            Filters::Nor(filters) => {
                Filter::with_field("$nor".to_string(), filters_into_array(filters))
            }
            Filters::Or(filters) => {
                Filter::with_field("$or".to_string(), filters_into_array(filters))
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------

pub trait ObjectFilter<F, B>
where
    F: BuildInnerFilter,
    B: HuusIntoBson,
{
    fn value(&mut self, value: B);
    fn dot(&mut self, filter: F);
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

pub trait ComparisonFilter<B>
where
    B: HuusIntoBson,
{
    fn eq(&mut self, value: B);
    fn gt(&mut self, value: B);
    fn gte(&mut self, value: B);
    fn r#in(&mut self, value: Vec<B>);
    fn lt(&mut self, value: B);
    fn lte(&mut self, value: B);
    fn ne(&mut self, value: B);
    fn nin(&mut self, value: Vec<B>);
}

#[derive(Clone, Debug)]
pub enum Comparison<B>
where
    B: HuusIntoBson,
{
    Eq(B),
    Gt(B),
    Gte(B),
    In(Vec<B>),
    Lt(B),
    Lte(B),
    Ne(B),
    Nin(Vec<B>),
}

impl<F> BuildInnerFilter for Comparison<F>
where
    F: HuusIntoBson,
{
    fn build_filter(self, field: String) -> Filter {
        match self {
            Comparison::Eq(value) => {
                Filter::with_field(field, bson!({ "$eq": value.huus_into_bson() }))
            }
            Comparison::Gt(value) => {
                Filter::with_field(field, bson!({ "$gt": value.huus_into_bson() }))
            }
            Comparison::Gte(value) => {
                Filter::with_field(field, bson!({ "$gte": value.huus_into_bson() }))
            }
            Comparison::In(values) => {
                Filter::with_field(field, bson!({ "$in": vec_into_array(values) }))
            }
            Comparison::Lt(value) => {
                Filter::with_field(field, bson!({ "$lt": value.huus_into_bson() }))
            }
            Comparison::Lte(value) => {
                Filter::with_field(field, bson!({ "$lte": value.huus_into_bson() }))
            }
            Comparison::Ne(value) => {
                Filter::with_field(field, bson!({ "$ne": value.huus_into_bson() }))
            }
            Comparison::Nin(values) => {
                Filter::with_field(field, bson!({ "$nin": vec_into_array(values) }))
            }
        }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Clone, Debug)]
pub enum Logical<F>
where
    F: BuildInnerFilter,
{
    And(Vec<F>),
    Not(F),
    Nor(Vec<F>),
    Or(Vec<F>),
}

impl<F> BuildInnerFilter for Logical<F>
where
    F: BuildInnerFilter,
{
    fn build_filter(self, field: String) -> Filter {
        match self {
            Logical::And(filters) => {
                Filter::with_field("$and".to_string(), inner_filters_into_array(filters, field))
            }
            Logical::Not(filter) => {
                Filter::with_field("$not".to_string(), filter.build_filter(field).into_bson())
            }
            Logical::Nor(filters) => {
                Filter::with_field("$nor".to_string(), inner_filters_into_array(filters, field))
            }
            Logical::Or(filters) => {
                Filter::with_field("$or".to_string(), inner_filters_into_array(filters, field))
            }
        }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

pub trait ElementFilter {
    fn exists(&mut self, exists: bool);
    fn with_type(&mut self, bson_type: types::Type);
}

#[derive(Clone, Debug)]
pub enum Element {
    Exists(bool),
    Type(types::Type),
}

impl BuildInnerFilter for Element {
    fn build_filter(self, field: String) -> Filter {
        match self {
            Element::Exists(value) => Filter::with_field(field, bson!({ "$exists": value })),
            Element::Type(data_type) => {
                Filter::with_field(field, bson!({ "$type": data_type as i32 }))
            }
        }
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

pub trait ArrayFilter<B> {
    fn all(&mut self, array: Vec<B>);
    fn elem_match(&mut self, array: Vec<B>);
    fn size(&mut self, size: usize);
}

#[derive(Clone, Debug)]
pub enum Array<B>
where
    B: HuusIntoBson,
{
    All(Vec<B>),
    ElemMatch(Vec<B>),
    Size(usize),
}

impl<B> BuildInnerFilter for Array<B>
where
    B: HuusIntoBson,
{
    fn build_filter(self, field: String) -> Filter {
        match self {
            Array::All(values) => {
                Filter::with_field(field, bson!({ "$all": vec_into_array(values) }))
            }
            Array::ElemMatch(values) => {
                Filter::with_field(field, bson!({ "$elemMatch": vec_into_array(values) }))
            }
            Array::Size(size) => Filter::with_field(field, bson!({ "$size": size as i32 })),
        }
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub enum F64Entry {
    Value(types::Double),
    Comparison(Comparison<types::Double>),
    Element(Element),
    Empty,
}

impl BuildInnerFilter for F64Entry {
    fn build_filter(self, field: String) -> Filter {
        match self {
            F64Entry::Value(value) => Filter::with_field(field, bson::Bson::FloatingPoint(value)),
            F64Entry::Comparison(comparison) => comparison.build_filter(field),
            F64Entry::Element(element) => element.build_filter(field),
            F64Entry::Empty => Filter::empty(),
        }
    }
}

impl BuildInnerFilter for Option<F64Entry> {
    fn build_filter(self, field: String) -> Filter {
        match self {
            Some(entry) => entry.build_filter(field),
            None => Filter::empty(),
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
    Comparison(Comparison<String>),
    Element(Element),
    Empty,
}

impl ComparisonFilter<String> for StringEntry {
    fn eq(&mut self, value: String) {
        *self = StringEntry::Comparison(Comparison::Eq(value));
    }

    fn gt(&mut self, value: String) {
        *self = StringEntry::Comparison(Comparison::Gt(value));
    }

    fn gte(&mut self, value: String) {
        *self = StringEntry::Comparison(Comparison::Gte(value));
    }

    fn r#in(&mut self, value: Vec<String>) {
        *self = StringEntry::Comparison(Comparison::In(value));
    }

    fn lt(&mut self, value: String) {
        *self = StringEntry::Comparison(Comparison::Lt(value));
    }

    fn lte(&mut self, value: String) {
        *self = StringEntry::Comparison(Comparison::Lte(value));
    }

    fn ne(&mut self, value: String) {
        *self = StringEntry::Comparison(Comparison::Ne(value));
    }

    fn nin(&mut self, value: Vec<String>) {
        *self = StringEntry::Comparison(Comparison::Nin(value));
    }
}

impl BuildInnerFilter for StringEntry {
    fn build_filter(self, field: String) -> Filter {
        match self {
            StringEntry::Value(value) => Filter::with_field(field, bson::Bson::String(value)),
            StringEntry::Comparison(comparison) => comparison.build_filter(field),
            StringEntry::Element(element) => element.build_filter(field),
            StringEntry::Empty => Filter::empty(),
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
pub enum EnumEntry<K>
where
    K: HuusKey,
{
    Value(K),
    Comparison(Comparison<String>),
    Element(Element),
    Empty,
}

impl<K> BuildInnerFilter for EnumEntry<K>
where
    K: HuusKey,
{
    fn build_filter(self, field: String) -> Filter {
        match self {
            EnumEntry::Value(value) => {
                Filter::with_field(field, bson::Bson::String(value.to_str().to_string()))
            }
            EnumEntry::Comparison(comparison) => comparison.build_filter(field),
            EnumEntry::Element(element) => element.build_filter(field),
            EnumEntry::Empty => Filter::empty(),
        }
    }
}

impl<K> Default for EnumEntry<K>
where
    K: HuusKey,
{
    fn default() -> Self {
        EnumEntry::Empty
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Clone, Debug)]
pub enum ObjectEntry<F, B>
where
    F: BuildInnerFilter,
    B: HuusIntoBson,
{
    Value(B),
    Dot(F),
    Logical(Box<Logical<ObjectEntry<F, B>>>),
    Element(Element),
    Empty,
}

impl<F, B> ObjectFilter<F, B> for ObjectEntry<F, B>
where
    F: BuildInnerFilter,
    B: HuusIntoBson,
{
    fn value(&mut self, value: B) {
        *self = ObjectEntry::Value(value);
    }

    fn dot(&mut self, filter: F) {
        *self = ObjectEntry::Dot(filter);
    }
}

impl<F, B> BuildInnerFilter for ObjectEntry<F, B>
where
    F: BuildInnerFilter,
    B: HuusIntoBson,
{
    fn build_filter(self, field: String) -> Filter {
        match self {
            ObjectEntry::Value(value) => Filter::with_field(field, value.huus_into_bson()),
            ObjectEntry::Dot(value) => value.build_filter(field),
            ObjectEntry::Logical(logical) => logical.build_filter(field),
            ObjectEntry::Element(element) => element.build_filter(field),
            ObjectEntry::Empty => Filter::empty(),
        }
    }
}

impl<F, B> Default for ObjectEntry<F, B>
where
    F: BuildInnerFilter,
    B: HuusIntoBson,
{
    fn default() -> Self {
        ObjectEntry::Empty
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Clone, Debug)]
pub enum BTreeMapEntry<K, B>
where
    K: HuusKey,
    B: HuusIntoBson,
{
    Value(BTreeMap<K, B>),
    Logical(Box<Logical<BTreeMapEntry<K, B>>>),
    Element(Element),
    Empty,
}

impl<K, B> BuildInnerFilter for BTreeMapEntry<K, B>
where
    K: HuusKey,
    B: HuusIntoBson,
{
    fn build_filter(self, field: String) -> Filter {
        match self {
            BTreeMapEntry::Value(value) => Filter::with_field(field, value.huus_into_bson()),
            BTreeMapEntry::Logical(logical) => logical.build_filter(field),
            BTreeMapEntry::Element(element) => element.build_filter(field),
            BTreeMapEntry::Empty => Filter::empty(),
        }
    }
}

impl<K, B> Default for BTreeMapEntry<K, B>
where
    K: HuusKey,
    B: HuusIntoBson,
{
    fn default() -> Self {
        BTreeMapEntry::Empty
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Clone, Debug)]
pub enum HashMapEntry<K, B>
where
    K: HuusKey,
    B: HuusIntoBson,
{
    Value(HashMap<K, B>),
    Logical(Box<Logical<HashMapEntry<K, B>>>),
    Element(Element),
    Empty,
}

impl<K, B> BuildInnerFilter for HashMapEntry<K, B>
where
    K: HuusKey,
    B: HuusIntoBson,
{
    fn build_filter(self, field: String) -> Filter {
        match self {
            HashMapEntry::Value(value) => Filter::with_field(field, value.huus_into_bson()),
            HashMapEntry::Logical(logical) => logical.build_filter(field),
            HashMapEntry::Element(element) => element.build_filter(field),
            HashMapEntry::Empty => Filter::empty(),
        }
    }
}

impl<K, B> Default for HashMapEntry<K, B>
where
    K: HuusKey,
    B: HuusIntoBson,
{
    fn default() -> Self {
        HashMapEntry::Empty
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Clone, Debug)]
pub enum ArrayEntry<F, B>
where
    F: BuildInnerFilter,
    B: HuusIntoBson,
{
    Value(B),
    Dot(F),
    Array(Array<B>),
    Comparison(Comparison<B>),
    Element(Element),
    Empty,
}

impl<F, B> ObjectFilter<F, B> for ArrayEntry<F, B>
where
    F: BuildInnerFilter,
    B: HuusIntoBson,
{
    fn value(&mut self, value: B) {
        *self = ArrayEntry::Value(value);
    }

    fn dot(&mut self, filter: F) {
        *self = ArrayEntry::Dot(filter);
    }
}

impl<F, B> ArrayFilter<B> for ArrayEntry<F, B>
where
    F: BuildInnerFilter,
    B: HuusIntoBson,
{
    fn all(&mut self, array: Vec<B>) {
        *self = ArrayEntry::Array(Array::All(array));
    }

    fn elem_match(&mut self, array: Vec<B>) {
        *self = ArrayEntry::Array(Array::ElemMatch(array));
    }

    fn size(&mut self, size: usize) {
        *self = ArrayEntry::Array(Array::Size(size));
    }
}

impl<F, B> ComparisonFilter<B> for ArrayEntry<F, B>
where
    F: BuildInnerFilter,
    B: HuusIntoBson,
{
    fn eq(&mut self, value: B) {
        *self = ArrayEntry::Comparison(Comparison::Eq(value));
    }

    fn gt(&mut self, value: B) {
        *self = ArrayEntry::Comparison(Comparison::Gt(value));
    }

    fn gte(&mut self, value: B) {
        *self = ArrayEntry::Comparison(Comparison::Gte(value));
    }

    fn r#in(&mut self, value: Vec<B>) {
        *self = ArrayEntry::Comparison(Comparison::In(value));
    }

    fn lt(&mut self, value: B) {
        *self = ArrayEntry::Comparison(Comparison::Lt(value));
    }

    fn lte(&mut self, value: B) {
        *self = ArrayEntry::Comparison(Comparison::Lte(value));
    }

    fn ne(&mut self, value: B) {
        *self = ArrayEntry::Comparison(Comparison::Ne(value));
    }

    fn nin(&mut self, value: Vec<B>) {
        *self = ArrayEntry::Comparison(Comparison::Nin(value));
    }
}

impl<F, B> BuildInnerFilter for ArrayEntry<F, B>
where
    F: BuildInnerFilter,
    B: HuusIntoBson,
{
    fn build_filter(self, field: String) -> Filter {
        match self {
            ArrayEntry::Value(value) => Filter::with_field(field, value.huus_into_bson()),
            ArrayEntry::Dot(value) => value.build_filter(field),
            ArrayEntry::Array(array) => array.build_filter(field),
            ArrayEntry::Comparison(value) => value.build_filter(field),
            ArrayEntry::Element(element) => element.build_filter(field),
            ArrayEntry::Empty => Filter::empty(),
        }
    }
}

impl<F, B> Default for ArrayEntry<F, B>
where
    F: BuildInnerFilter,
    B: HuusIntoBson,
{
    fn default() -> Self {
        ArrayEntry::Empty
    }
}

impl<F, B> std::convert::From<Vec<B>> for ArrayEntry<F, B>
where
    F: BuildInnerFilter,
    B: HuusIntoBson,
{
    fn from(array: Vec<B>) -> ArrayEntry<F, B> {
        ArrayEntry::Array(Array::All(array))
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Clone, Debug)]
pub enum ObjectIdEntry {
    Value(types::ObjectId),
    Element(Element),
    Empty,
}

impl BuildInnerFilter for ObjectIdEntry {
    fn build_filter(self, field: String) -> Filter {
        match self {
            ObjectIdEntry::Value(value) => Filter::with_field(field, bson::Bson::ObjectId(value)),
            ObjectIdEntry::Element(element) => element.build_filter(field),
            ObjectIdEntry::Empty => Filter::empty(),
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
    Element(Element),
    Empty,
}

impl ElementFilter for BooleanEntry {
    fn exists(&mut self, exists: bool) {
        *self = BooleanEntry::Element(Element::Exists(exists));
    }

    fn with_type(&mut self, bson_type: types::Type) {
        *self = BooleanEntry::Element(Element::Type(bson_type));
    }
}

impl BuildInnerFilter for BooleanEntry {
    fn build_filter(self, field: String) -> Filter {
        match self {
            BooleanEntry::Value(value) => Filter::with_field(field, bson::Bson::Boolean(value)),
            BooleanEntry::Element(element) => element.build_filter(field),
            BooleanEntry::Empty => Filter::empty(),
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
    Comparison(Comparison<chrono::DateTime<chrono::Utc>>),
    Element(Element),
    Empty,
}

impl ComparisonFilter<types::Date> for DateEntry {
    fn eq(&mut self, value: types::Date) {
        *self = DateEntry::Comparison(Comparison::Eq(value));
    }

    fn gt(&mut self, value: types::Date) {
        *self = DateEntry::Comparison(Comparison::Gt(value));
    }

    fn gte(&mut self, value: types::Date) {
        *self = DateEntry::Comparison(Comparison::Gte(value));
    }

    fn r#in(&mut self, value: Vec<types::Date>) {
        *self = DateEntry::Comparison(Comparison::In(value));
    }

    fn lt(&mut self, value: types::Date) {
        *self = DateEntry::Comparison(Comparison::Lt(value));
    }

    fn lte(&mut self, value: types::Date) {
        *self = DateEntry::Comparison(Comparison::Lte(value));
    }

    fn ne(&mut self, value: types::Date) {
        *self = DateEntry::Comparison(Comparison::Ne(value));
    }

    fn nin(&mut self, value: Vec<types::Date>) {
        *self = DateEntry::Comparison(Comparison::Nin(value));
    }
}

impl BuildInnerFilter for DateEntry {
    fn build_filter(self, field: String) -> Filter {
        match self {
            DateEntry::Value(value) => Filter::with_field(field, bson::Bson::UtcDatetime(value)),
            DateEntry::Comparison(comparison) => comparison.build_filter(field),
            DateEntry::Element(element) => element.build_filter(field),
            DateEntry::Empty => Filter::empty(),
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

pub enum NullEntry {
    Element(Element),
    Empty,
}

impl BuildInnerFilter for NullEntry {
    fn build_filter(self, field: String) -> Filter {
        match self {
            NullEntry::Element(element) => element.build_filter(field),
            NullEntry::Empty => Filter::empty(),
        }
    }
}

impl Default for NullEntry {
    fn default() -> Self {
        NullEntry::Empty
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Clone, Debug)]
pub enum I32Entry {
    Value(i32),
    Comparison(Comparison<i32>),
    Element(Element),
    Empty,
}

impl BuildInnerFilter for I32Entry {
    fn build_filter(self, field: String) -> Filter {
        match self {
            I32Entry::Value(value) => Filter::with_field(field, bson::Bson::I32(value)),
            I32Entry::Comparison(comparison) => comparison.build_filter(field),
            I32Entry::Element(element) => element.build_filter(field),
            I32Entry::Empty => Filter::empty(),
        }
    }
}

impl Default for I32Entry {
    fn default() -> Self {
        I32Entry::Empty
    }
}

impl ElementFilter for I32Entry {
    fn exists(&mut self, exists: bool) {
        *self = I32Entry::Element(Element::Exists(exists));
    }

    fn with_type(&mut self, bson_type: types::Type) {
        *self = I32Entry::Element(Element::Type(bson_type));
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
    Comparison(Comparison<types::TimeStamp>),
    Element(Element),
    Empty,
}

impl BuildInnerFilter for TimeStampEntry {
    fn build_filter(self, field: String) -> Filter {
        match self {
            TimeStampEntry::Value(value) => Filter::with_field(field, value.huus_into_bson()),
            TimeStampEntry::Comparison(comparison) => comparison.build_filter(field),
            TimeStampEntry::Element(element) => element.build_filter(field),
            TimeStampEntry::Empty => Filter::empty(),
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
    Comparison(Comparison<i64>),
    Element(Element),
    Empty,
}

impl BuildInnerFilter for I64Entry {
    fn build_filter(self, field: String) -> Filter {
        match self {
            I64Entry::Value(value) => Filter::with_field(field, bson::Bson::I64(value)),
            I64Entry::Comparison(comparison) => comparison.build_filter(field),
            I64Entry::Element(element) => element.build_filter(field),
            I64Entry::Empty => Filter::empty(),
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

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Clone, Debug)]
pub enum BsonEntry {
    Value(bson::Document),
    Element(Element),
    Empty,
}

impl BuildInnerFilter for BsonEntry {
    fn build_filter(self, field: String) -> Filter {
        match self {
            BsonEntry::Value(value) => Filter::with_field(field, bson::Bson::Document(value)),
            BsonEntry::Element(element) => element.build_filter(field),
            BsonEntry::Empty => Filter::empty(),
        }
    }
}

impl Default for BsonEntry {
    fn default() -> Self {
        BsonEntry::Empty
    }
}

impl std::convert::From<bson::Document> for BsonEntry {
    fn from(value: bson::Document) -> BsonEntry {
        BsonEntry::Value(value)
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Filter {
    doc: bson::Document,
}

impl Filter {
    pub fn empty() -> Self {
        Self { doc: bson::Document::new() }
    }

    pub fn with_field(field: String, value: bson::Bson) -> Self {
        let mut filter = Filter::empty();
        filter.doc.insert(field, value);
        filter
    }

    pub fn into_doc(self) -> bson::Document {
        self.doc
    }

    pub fn into_bson(self) -> bson::Bson {
        bson::Bson::Document(self.doc)
    }

    pub fn incorporate(&mut self, filter: Filter) {
        for (key, value) in filter.doc {
            self.doc.insert_bson(key, value);
        }
    }
}
