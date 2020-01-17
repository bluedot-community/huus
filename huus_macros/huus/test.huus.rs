pub enum Enum1 {
    Choice1 as "choice_1",
    Choice2 as "choice_2",
}

pub struct Doc1 {
    integer as "int": i32?,
    string as "str": String+,
    array: Vec String?,
}

pub enum Union1 {
    Choice1 as "choice_1": Doc1,
    Choice2 as "choice_2": Doc2,
}

pub struct Doc2 in "coll_2" {
    data: Doc1?,
    string as "str": String?,
}

pub struct Doc3 in "coll_3" {
    object_id as "_id": ObjectId,
    data: Doc1,
    array: Vec Doc1,
    simple_map: BTreeMap String String,
    nested_map: BTreeMap Enum1 Doc1,
    boolean: bool,
    date: Date,
    indexed: String+,
    integers: Vec i64,
    choice: Enum1,
    union: Union1,
    bson: Bson,
}

