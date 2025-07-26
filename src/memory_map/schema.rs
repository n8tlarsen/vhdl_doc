use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use schemars::schema_for;
use serde_json::ser::PrettyFormatter;

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Protocol {
    #[doc = "An optional name for the protocol"]
    name: Option<String>,
    #[doc = "Maximum address in terms of dataMin"]
    address_max: Address,
    #[doc = "Minimum addressable data size in bytes"]
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

pub fn get_memory_map_schema() -> String {
    let schema = schema_for!(MemoryMap);
    let formatter = PrettyFormatter::with_indent(b"    ");
    let mut buf = Vec::new();
    let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
    serde::Serialize::serialize(&schema, &mut ser).expect("Failed to serialize schema");
    String::from_utf8(buf).expect("Failed to convert serial buffer to string")
}

