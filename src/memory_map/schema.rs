use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Protocol {
    name: Option<String>,
    #[schemars(description = "Maximum address in terms of dataMin")]
    address_max: Address,
    #[schemars(description = "Minimum addressable data size in bytes")]
    data_min: u8,
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(untagged)]
pub enum Address {
    Str(String),
    Int(u64),
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all="lowercase")]
pub enum FieldType {
    Set,
    String(u64),
    Vector(u64),
    Unsigned(u64),
    Signed(u64),
    UFixed(i64,i64),
    SFixed(i64,i64),
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(untagged)]
pub enum OneOrMoreField {
    One(Box<Field>),
    More(Vec<Field>)
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(untagged)]
pub enum Value {
    String(String),
    Unsigned(u64),
    Signed(i64),
    Float(f64)
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub enum Access {
    #[serde(rename = "r")]
    Read,
    #[serde(rename = "w")]
    Write,
    #[serde(rename = "rw")]
    ReadWrite,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct Field {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access: Option<Access>,
    #[serde(rename = "type")]
    field_type: FieldType,
    #[serde(skip_serializing_if = "Option::is_none")]
    contains: Option<OneOrMoreField>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<Value>
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct MemoryMap {
    protocol: Protocol,
    #[serde(flatten)]
    field: Field
}


