use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct Protocol {
    name: String,
    #[serde(rename = "addressMax")]
    address_max: u64,
    #[serde(rename = "dataMin")]
    data_min: u8,
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(untagged)]
pub enum HexInteger {
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
    address: Option<HexInteger>,
    access: Access,
    #[serde(rename = "type")]
    field_type: String,
    contains: Option<OneOrMoreField>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct MemoryMap {
    name: String,
    protocol: Protocol,
    address: Option<HexInteger>,
    access: Access,
    #[serde(rename = "type")]
    field_type: FieldType,
    contains: Option<OneOrMoreField>,
}


