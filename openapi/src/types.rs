use serde_json::Value;

#[derive(Clone, Copy, Debug)]
pub enum CopyOrClone {
    Copy,
    Clone,
}

#[derive(Clone, Debug, Copy)]
pub enum TypeError {
    IsObject,
    NoType,
    Unhandled,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InferredEnum {
    pub parent: String,
    pub field: String,
    pub options: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InferredUnion {
    pub field: String,
    pub schema_variants: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InferredStruct {
    pub field: String,
    pub schema: Value,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InferredParams {
    pub method: String,
    pub rust_type: String,
    pub parameters: Value,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InferredObject {
    pub rust_type: String,
    pub schema: Value,
}

#[derive(Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum MethodTypes {
    List,
    Create,
    Retrieve,
    Update,
    Delete,
}
