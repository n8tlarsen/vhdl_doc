use schemars::schema_for;
use schemars::JsonSchema;
use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::ser::PrettyFormatter;
use std::collections::HashMap;
use std::fmt;

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(untagged)]
pub enum Address {
    Str(String),
    Int(u64),
}

fn hex_str_or_unsigned<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    struct HexVisitor;

    impl<'de> Visitor<'de> for HexVisitor {
        type Value = u64;

        fn expecting(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt.write_str("unsigned or string")
        }

        fn visit_i64<E>(self, val: i64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(val as u64)
        }

        fn visit_u64<E>(self, val: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(val)
        }

        fn visit_str<E>(self, val: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            if let Some(stripped) = val.strip_prefix("0x") {
                let deformat = stripped.to_string().replace("_", "");
                match u64::from_str_radix(&deformat, 16) {
                    Ok(parsed_int) => Ok(parsed_int),
                    Err(_) => Err(E::custom("failed to parse hex string")),
                }
            } else {
                Err(E::custom("failed to parse hex string"))
            }
        }
    }

    deserializer.deserialize_any(HexVisitor)
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Protocol {
    #[doc = "An optional name for the protocol"]
    name: Option<String>,
    #[doc = "Maximum address in terms of dataMin"]
    #[serde(deserialize_with = "hex_str_or_unsigned")]
    address_max: u64,
    #[doc = "Minimum addressable data size in bytes"]
    data_min: u8,
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(untagged)]
pub enum BitfieldStyle {
    #[doc = "Contiguous array of bit names starting at index 0. If array length is shorter than the field, the remainging bits are marked as \"Reserved\""]
    FromZero(Vec<String>),
    #[doc = "Discrete key-value pairs of bit names and indices"]
    Discrete(HashMap<String, u64>),
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum FieldType {
    Set,
    String(u64),
    Enum {
        length: u64,
        map: HashMap<String, u64>,
    },
    Bitfield {
        length: u64,
        bits: BitfieldStyle,
    },
    Unsigned(u64),
    Signed(u64),
    UFixed(i64, i64),
    SFixed(i64, i64),
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(untagged)]
pub enum OneOrMoreField {
    One(Box<Field>),
    More(Vec<Field>),
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(untagged)]
pub enum Value {
    String(String),
    Unsigned(u64),
    Signed(i64),
    Float(f64),
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
    value: Option<Value>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct MemoryMap {
    protocol: Protocol,
    #[serde(flatten)]
    field: Field,
}

pub fn get_memory_map_schema() -> String {
    let schema = schema_for!(MemoryMap);
    let formatter = PrettyFormatter::with_indent(b"    ");
    let mut buf = Vec::new();
    let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
    serde::Serialize::serialize(&schema, &mut ser).expect("Failed to serialize schema");
    String::from_utf8(buf).expect("Failed to convert serial buffer to string")
}
