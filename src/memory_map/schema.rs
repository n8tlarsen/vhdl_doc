use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct Protocol {
    pub name: String,
    #[serde(rename = "addressMax")]
    pub address_max: u64,
    #[serde(rename = "dataMin")]
    pub data_min: u8,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub enum HexInteger {
    Str(String),
    Int(u64),
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(tag = "type")]
pub enum FieldType {
    Set,
    Numeric(String),
    String,
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
    pub name: String,
    pub address: Option<HexInteger>,
    #[serde(rename = "type")]
    pub access: Access,
    pub field_type: FieldType,
    pub contains: Option<Box<Field>>,
}

#[derive(JsonSchema)]
pub struct MemoryMap {
    pub name: String,
    pub protocol: Protocol,
    pub address: Option<HexInteger>,
    pub access: Access,
    pub field_type: FieldType,
    pub contains: Option<Box<Field>>,
}


