// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Tests for `huus_macros` crate checking errors reported during macro processing. They use specie
//! versions of `data`, `filter` and `update` macros generating a vector of `Problem`s instead of
//! actual code.

#![feature(proc_macro_hygiene)]

use huus_macros_support::Problem;

huus_macros::define_from!("test");

// -------------------------------------------------------------------------------------------------
// `data` macro

/// Control test without problems found.
#[test]
fn data_control() {
    let problems = huus_macros::data_testing! { ("coll_2")
        "data": {
            "int": 1,
            "str": "abc",
        },
        "str": "def",
    };
    assert_eq!(problems.len(), 0);
}

/// Macro is empty.
#[test]
fn data_macro_empty() {
    let problems = huus_macros::data_testing! { ("coll_2") };
    assert_eq!(problems, vec![Problem::MacroEmpty]);
}

/// Field `abc` does not exist in the schema.
#[test]
fn data_field_not_found() {
    let problems = huus_macros::data_testing! { ("coll_2")
        "abc": 3,
        "str": "def",
    };
    assert_eq!(problems, vec![Problem::FieldNotFound]);
}

/// Field is specified on a member `choice` defined as an enum.
#[test]
fn data_field_on_enum() {
    let problems = huus_macros::data_testing! { ("coll_3")
        "data": { "str": "abc" },
        "choice.abc": 3,
    };
    assert_eq!(problems, vec![Problem::AttrWithDots, Problem::FieldsMissing]);
}

/// Value was provided for map member `simple_map` which can be provided only in code mode.
#[test]
fn data_exp_code_comp() {
    let problems = huus_macros::data_testing! { ("coll_3")
        "data": { "str": "abc" },
        "simple_map": 4,
    };
    assert_eq!(problems, vec![Problem::ExpCodeComp, Problem::FieldsMissing]);
}

/// Value was provided for enum member `choice` which can be provided only in code mode.
#[test]
fn data_exp_code_enum() {
    let problems = huus_macros::data_testing! { ("coll_3")
        "data": { "str": "abc" },
        "choice": "choice_1",
    };
    assert_eq!(problems, vec![Problem::ExpCodeEnum, Problem::FieldsMissing]);
}

/// Value was provided for a union member `union` which can be provided only in code mode.
#[test]
fn data_exp_code_union() {
    let problems = huus_macros::data_testing! { ("coll_3")
        "data": { "str": "abc" },
        "union": { "str": "abc" },
    };
    assert_eq!(problems, vec![Problem::ExpCodeUnion, Problem::FieldsMissing]);
}

/// Value was provided for member `data` which expected an object.
#[test]
fn data_exp_object() {
    let problems = huus_macros::data_testing! { ("coll_3")
        "boolean": true,
        "data": 5,
    };
    assert_eq!(problems, vec![Problem::ExpObject, Problem::FieldsMissing]);
}

/// Value of another type was provided for member `indexed` which expected a string.
#[test]
fn data_exp_string() {
    let problems = huus_macros::data_testing! { ("coll_3")
        "data": { "str": "abc" },
        "indexed": 2,
    };
    assert_eq!(problems, vec![Problem::ExpString, Problem::FieldsMissing]);
}

/// Value of another type was provided for member `_id` which expected an object ID.
#[test]
fn data_exp_oid() {
    let problems = huus_macros::data_testing! { ("coll_3")
        "data": { "str": "abc" },
        "_id": "xyz",
    };
    assert_eq!(problems, vec![Problem::ExpOid, Problem::FieldsMissing]);
}

/// Value of another type was provided for member `boolean` which expected a `bool`.
#[test]
fn data_exp_bool() {
    let problems = huus_macros::data_testing! { ("coll_3")
        "data": { "str": "abc" },
        "boolean": 1,
    };
    assert_eq!(problems, vec![Problem::ExpBool, Problem::FieldsMissing]);
}

/// Value of another type was provided for member `date` which expected a date.
#[test]
fn data_exp_date() {
    let problems = huus_macros::data_testing! { ("coll_3")
        "data": { "str": "abc" },
        "date": "Tuesday",
    };
    assert_eq!(problems, vec![Problem::ExpDate, Problem::FieldsMissing]);
}

/// Value of another type was provided for member `data.int` which expected `i32`.
#[test]
fn data_exp_i32() {
    let problems = huus_macros::data_testing! { ("coll_3")
        "data": { "int": "abc", "str": "abc" },
        "boolean": true,
    };
    assert_eq!(problems, vec![Problem::ExpI32, Problem::FieldsMissing]);
}

/// Value of another type was provided for member `bson` which expected a BSON object.
#[test]
fn data_exp_bson() {
    let problems = huus_macros::data_testing! { ("coll_3")
        "data": { "str": "abc" },
        "bson": "bson",
    };
    assert_eq!(problems, vec![Problem::ExpBson, Problem::FieldsMissing]);
}

// -------------------------------------------------------------------------------------------------
// `filter` macro

/// Control test without problems found.
#[test]
fn filter_control() {
    let problems = huus_macros::filter_testing! { ("coll_2")
        "data.int": 1,
        "data.str": "abc",
        "str": "def",
    };
    assert_eq!(problems.len(), 0);
}

/// Macro is empty.
#[test]
fn filter_macro_empty() {
    let problems = huus_macros::filter_testing! { ("coll_2") };
    assert_eq!(problems, vec![Problem::MacroEmpty]);
}

/// Field `abc` does not exist in the schema.
#[test]
fn filter_field_not_found() {
    let problems = huus_macros::filter_testing! { ("coll_2")
        "abc": 3,
        "str": "def",
    };
    assert_eq!(problems, vec![Problem::FieldNotFound]);
}

/// Field is specified on a member `choice` defined as an enum.
#[test]
fn filter_field_on_enum() {
    let problems = huus_macros::filter_testing! { ("coll_3")
        "data": { "str": "abc" },
        "choice.abc": 3,
    };
    assert_eq!(problems, vec![Problem::FieldOnEnum]);
}

/// Field is specified on a member `date` defined as a built-in type.
#[test]
fn filter_field_on_plain() {
    let problems = huus_macros::filter_testing! { ("coll_3")
        "data": { "str": "abc" },
        "date.abc": 3,
    };
    assert_eq!(problems, vec![Problem::FieldOnPlain]);
}

/// Used operator `$unk` is unknown.
#[test]
fn filter_operator_unknown() {
    let problems = huus_macros::filter_testing! { ("coll_3")
        "data": { "str": "abc" },
        "date": { "$unk": 4 },
    };
    assert_eq!(problems, vec![Problem::OperatorUnknown]);
}

/// The used operator `$gt` cannot be used with the type of `integers` which is an array.
#[test]
fn filter_operator_incorrect() {
    let problems = huus_macros::filter_testing! { ("coll_3")
        "data": { "str": "abc" },
        "integers": { "$gt": 4 },
    };
    assert_eq!(problems, vec![Problem::OperatorIncorrect]);
}

/// Composed members like `simple_map` expect their values to be provided in code mode.
#[test]
fn filter_exp_code_comp() {
    let problems = huus_macros::filter_testing! { ("coll_3")
        "data": { "str": "abc" },
        "simple_map": 4,
    };
    assert_eq!(problems, vec![Problem::ExpCodeComp]);
}

/// Enum members like `choice` expect their values to be provided in code mode.
#[test]
fn filter_exp_code_enum() {
    let problems = huus_macros::filter_testing! { ("coll_3")
        "data": { "str": "abc" },
        "choice": "choice_1",
    };
    assert_eq!(problems, vec![Problem::ExpCodeEnum]);
}

/// Union members like `simple_map` expect their values to be provided in code mode.
#[test]
fn filter_exp_code_union() {
    let problems = huus_macros::filter_testing! { ("coll_3")
        "data": { "str": "abc" },
        "union": { "str": "abc" },
    };
    assert_eq!(problems, vec![Problem::ExpCodeUnion]);
}

/// Value was provided for member `data` which expected an object.
#[test]
fn filter_exp_object() {
    let problems = huus_macros::filter_testing! { ("coll_3")
        "boolean": true,
        "data": 5,
    };
    assert_eq!(problems, vec![Problem::ExpObject]);
}

/// Index was specified in an attribute where not index is allowed according to the schema.
#[test]
fn filter_exp_key() {
    let problems = huus_macros::filter_testing! { ("coll_3")
        "data": { "str": "abc" },
        "array.1.2": 2,
    };
    assert_eq!(problems, vec![Problem::ExpKey]);
}

/// Value of another type was provided for member `indexed` which expected a string.
#[test]
fn filter_exp_string() {
    let problems = huus_macros::filter_testing! { ("coll_3")
        "data": { "str": "abc" },
        "indexed": 2,
    };
    assert_eq!(problems, vec![Problem::ExpString]);
}

/// Value of another type was provided for member `_id` which expected an object ID.
#[test]
fn filter_exp_oid() {
    let problems = huus_macros::filter_testing! { ("coll_3")
        "data": { "str": "abc" },
        "_id": "xyz",
    };
    assert_eq!(problems, vec![Problem::ExpOid]);
}

/// Value of another type was provided for member `boolean` which expected a `bool`.
#[test]
fn filter_exp_bool() {
    let problems = huus_macros::filter_testing! { ("coll_3")
        "data": { "str": "abc" },
        "boolean": 1,
    };
    assert_eq!(problems, vec![Problem::ExpBool]);
}

/// Value of another type was provided for member `date` which expected a date.
#[test]
fn filter_exp_date() {
    let problems = huus_macros::filter_testing! { ("coll_3")
        "data": { "str": "abc" },
        "date": "Tuesday",
    };
    assert_eq!(problems, vec![Problem::ExpDate]);
}

/// Value of another type was provided for member `data.int` which expected `i32`.
#[test]
fn filter_exp_i32() {
    let problems = huus_macros::filter_testing! { ("coll_3")
        "data": { "int": "abc" },
        "boolean": true,
    };
    assert_eq!(problems, vec![Problem::ExpI32]);
}

/// Value of another type was provided for member `integers.1` which expected `i64`.
#[test]
fn filter_exp_i64() {
    let problems = huus_macros::filter_testing! { ("coll_3")
        "data": { "str": "abc" },
        "integers.1": "abc",
    };
    assert_eq!(problems, vec![Problem::ExpI64]);
}

/// Value of another type was provided for member `bson` which expected a BSON object.
#[test]
fn filter_exp_bson() {
    let problems = huus_macros::filter_testing! { ("coll_3")
        "data": { "str": "abc" },
        "bson": "bson",
    };
    assert_eq!(problems, vec![Problem::ExpBson]);
}

// -------------------------------------------------------------------------------------------------
// `update` macro

/// Control test without problems found.
#[test]
fn update_control() {
    let problems = huus_macros::update_testing! { ("coll_2")
        "$set": {
            "data.int": 1,
            "data.str": "abc",
            "str": "def",
        }
    };
    assert_eq!(problems.len(), 0);
}

/// Both operators and non-operator attributes used.
#[test]
fn update_query_both_update_and_repl() {
    let problems = huus_macros::update_testing! { ("coll_2")
        "$set": {
            "data.int": 1,
            "data.str": "abc",
        },
        "str": "def",
    };
    assert_eq!(problems, vec![Problem::QueryBothUpdateAndRepl]);
}

/// Macro is empty.
#[test]
fn update_query_empty() {
    let problems = huus_macros::update_testing! { ("coll_3") };
    assert_eq!(problems, vec![Problem::QueryEmpty]);
}

/// Attributes in replacement mode cannot contain dots.
#[test]
fn update_attr_with_dots() {
    let problems = huus_macros::update_testing! { ("coll_2")
        "data.int": 1,
        "data.str": "abc",
        "str": "def",
    };
    assert_eq!(problems, vec![Problem::AttrWithDots, Problem::AttrWithDots]);
}

/// Field `abc` does not exist in the schema.
#[test]
fn update_field_not_found() {
    let problems = huus_macros::update_testing! { ("coll_2")
        "abc": 3,
        "str": "def",
    };
    assert_eq!(problems, vec![Problem::FieldNotFound]);
}

/// Field is specified on a member `choice` defined as an enum.
#[test]
fn update_field_on_enum() {
    let problems = huus_macros::update_testing! { ("coll_3")
        "$set": {
            "data": { "str": "abc" },
            "choice.abc": 3,
        }
    };
    assert_eq!(problems, vec![Problem::FieldOnEnum]);
}

/// Field is specified on a member `date` defined as a built-in type.
#[test]
fn update_field_on_plain() {
    let problems = huus_macros::update_testing! { ("coll_3")
        "$set": {
            "data": { "str": "abc" },
            "date.abc": 3,
        }
    };
    assert_eq!(problems, vec![Problem::FieldOnPlain]);
}

/// Used operator `$unk` is unknown.
#[test]
fn update_operator_unknown() {
    let problems = huus_macros::update_testing! { ("coll_2")
        "$unk": {
            "data.int": 1,
            "data.str": "abc",
        },
        "$set": {
            "str": "def",
        }
    };
    assert_eq!(problems, vec![Problem::OperatorUnknown]);
}

/// The used operator `$push` cannot be uses with the type of `data.int` which is `i32`.
#[test]
fn update_operator_incorrect() {
    let problems = huus_macros::update_testing! { ("coll_2")
        "$set": {
            "str": "def",
        },
        "$push": {
            "data.int": 1,
        },
    };
    assert_eq!(problems, vec![Problem::OperatorIncorrect]);
}

/// Composed values like arrays have to be provided in code mode.
#[test]
fn update_exp_code() {
    let problems = huus_macros::update_testing! { ("coll_3")
        "$pull": {
            "integers": { "$in": 1 },
        },
        "$set": {
            "boolean": true,
        }
    };
    assert_eq!(problems, vec![Problem::ExpCode]);
}

/// Enum members like `simple_map` expect their values to be provided in code mode.
#[test]
fn update_exp_code_enum() {
    let problems = huus_macros::update_testing! { ("coll_3")
        "$set": {
            "data": { "str": "abc" },
            "choice": "choice_1",
        }
    };
    assert_eq!(problems, vec![Problem::ExpCodeEnum]);
}

/// Union members like `simple_map` expect their values to be provided in code mode.
#[test]
fn update_exp_code_union() {
    let problems = huus_macros::update_testing! { ("coll_3")
        "$set": {
            "data": { "str": "abc" },
            "union": { "str": "abc" },
        }
    };
    assert_eq!(problems, vec![Problem::ExpCodeUnion]);
}

/// Value was provided for member `data` which expected an object.
#[test]
fn update_exp_object() {
    let problems = huus_macros::update_testing! { ("coll_3")
        "$set": {
            "boolean": true,
            "data": 5,
        }
    };
    assert_eq!(problems, vec![Problem::ExpObject]);
}

/// Index was specified in an attribute where not index is allowed according to the schema.
#[test]
fn update_exp_key() {
    let problems = huus_macros::update_testing! { ("coll_3")
        "$set": {
            "data": { "str": "abc" },
            "array.1.2": 2,
        }
    };
    assert_eq!(problems, vec![Problem::ExpKey]);
}

/// Value of another type was provided for member `indexed` which expected a string.
#[test]
fn update_exp_string() {
    let problems = huus_macros::update_testing! { ("coll_3")
        "$set": {
            "data": { "str": "abc" },
            "indexed": 2,
        }
    };
    assert_eq!(problems, vec![Problem::ExpString]);
}

/// Value of another type was provided for member `_id` which expected an object ID.
#[test]
fn update_exp_oid() {
    let problems = huus_macros::update_testing! { ("coll_3")
        "$set": {
            "data": { "str": "abc" },
            "_id": "xyz",
        }
    };
    assert_eq!(problems, vec![Problem::ExpOid]);
}

/// Value of another type was provided for member `boolean` which expected a `bool`.
#[test]
fn update_exp_bool() {
    let problems = huus_macros::update_testing! { ("coll_3")
        "$set": {
            "data": { "str": "abc" },
            "boolean": 1,
        }
    };
    assert_eq!(problems, vec![Problem::ExpBool]);
}

/// Value of another type was provided for member `date` which expected a date.
#[test]
fn update_exp_date() {
    let problems = huus_macros::update_testing! { ("coll_3")
        "$set": {
            "data": { "str": "abc" },
            "date": "Tuesday",
        }
    };
    assert_eq!(problems, vec![Problem::ExpDate]);
}

/// Value of another type was provided for member `data.int` which expected `i32`.
#[test]
fn update_exp_i32() {
    let problems = huus_macros::update_testing! { ("coll_3")
        "$set": {
            "data": { "int": "abc", "str": "abc" },
            "boolean": true,
        }
    };
    assert_eq!(problems, vec![Problem::ExpI32]);
}

/// Value of another type was provided for member `integers.1` which expected `i64`.
#[test]
fn update_exp_i64() {
    let problems = huus_macros::update_testing! { ("coll_3")
        "$set": {
            "data": { "str": "abc" },
            "integers.1": "abc",
        }
    };
    assert_eq!(problems, vec![Problem::ExpI64]);
}

/// The value assigned to `date` is not formatted according to requirements of `$currentDate`
/// operator.
#[test]
fn update_exp_date_obj() {
    let problems = huus_macros::update_testing! { ("coll_3")
        "$set": {
            "data": { "str": "abc" },
        },
        "$currentDate": {
            "date": { "$unk": "abc" }
        }
    };
    assert_eq!(problems, vec![Problem::ExpDateObj]);
}

/// `$unset` operator expects the values to be empty strings.
#[test]
fn update_exp_empty_string() {
    let problems = huus_macros::update_testing! { ("coll_2")
        "$set": {
            "data": { "str": "abc" },
        },
        "$unset": {
            "str": ".",
        }
    };
    assert_eq!(problems, vec![Problem::ExpEmptyString]);
}

