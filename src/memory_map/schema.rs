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
    UFixed(u64,u64),
    SFixed(u64,u64),
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
    pub name: String,
    pub address: Option<HexInteger>,
    pub access: Access,
    #[serde(rename = "type")]
    pub field_type: String,
    pub contains: Option<OneOrMoreField>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct MemoryMap {
    pub name: String,
    pub protocol: Protocol,
    pub address: Option<HexInteger>,
    pub access: Access,
    #[serde(rename = "type")]
    pub field_type: FieldType,
    pub contains: Option<OneOrMoreField>,
}


