use schemars::schema_for;
use schemars::JsonSchema;
use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::ser::PrettyFormatter;
use std::collections::HashMap;
use std::fmt;

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

fn maybe_hex_str_or_unsigned<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Some(hex_str_or_unsigned(deserializer)?))
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Protocol {
    /// An optional name for the protocol
    name: Option<String>,
    /// Maximum address in terms of dataMin.
    /// Accepts '0x' prefixed hex strings with underscores allowed between digits to enhance readability
    #[serde(deserialize_with = "hex_str_or_unsigned")]
    address_max: u64,
    /// Minimum addressable data size in bytes
    data_min: u8,
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(untagged)]
pub enum BitfieldStyle {
    /// Contiguous array of bit names starting at index 0.
    /// If array length is shorter than the field, the remainging bits are marked as 'Reserved'
    FromZero(Vec<String>),
    /// Discrete key-value pairs of bit names and indices
    Discrete(HashMap<String, u64>),
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum FieldType {
    /// Group of other types, typically used to describe a contiguous block of registers
    Set,
    /// String type
    String(u64),
    /// Enumerated type
    Enum {
        length: u64,
        map: HashMap<String, u64>,
    },
    /// Bitfield with named indices
    Bitfield { length: u64, bits: BitfieldStyle },
    /// Unsigned numeric type.
    /// Defined by length and representing the vhdl type `signed(length-1 downto 0)`
    Unsigned(u64),
    /// Signed numeric type.
    /// Defined by length and representing the vhdl type `unsigned(length-1 downto 0)`
    Signed(u64),
    /// Unsigned fixed point numeric type.
    /// Defined by the high and low subscripts typically representing the vhdl type
    /// `ufixed(high downto low)`.
    /// The binary point is located `low` places from the least significant digit.
    /// For exxample:
    /// ```toml
    /// ufixed.high = 11
    /// ufixed.low  = -4
    /// ```
    /// equates to:
    /// ``` vhdl
    /// ufixed(11 downto -4)
    /// ```
    /// and results in the binary fixed point form 000000000000.0000 with a resolution of
    /// 2^{-4}, a maximum value of (2^16 - 1) / (2^4), and a minimum value of 0.
    UFixed { high: i64, low: i64 },
    /// Signed fixed point numeric type.
    /// Defined by the high and low subscripts typically representing the vhdl type
    /// `sfixed(high downto low)`.
    /// The binary point is located `low` places from the least significant digit.
    /// For exxample:
    /// ```toml
    /// sfixed.high = 11
    /// sfixed.low  = -4
    /// ```
    /// equates to:
    /// ``` vhdl
    /// sfixed(11 downto -4)
    /// ```
    /// and results in the binary fixed point form 000000000000.0000 with a resolution of
    /// 2^{-4}, a maximum value of (2^{16-1} - 1) / (2^4), and a minimum value of
    /// -(2^{16-1} - 1) / (2^4).
    SFixed { high: i64, low: i64 },
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
    /// Read-only access is permitted
    #[serde(rename = "r")]
    Read,
    /// Write-only access is permitted
    #[serde(rename = "w")]
    Write,
    /// Both read and write access is permitted
    #[serde(rename = "rw")]
    ReadWrite,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct Field {
    name: String,
    /// Memory address. If no address is provided, the renderer will assume the field
    /// is packed directly following the previously defined address. If padding is desired to
    /// ensure allignment to Protocol.data_min, and the data type is smaller than data_min, it is
    /// required to explicitly specify the address.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, deserialize_with = "maybe_hex_str_or_unsigned")]
    address: Option<u64>,
    /// Register access permission.
    /// If no access permission is specified, the renterer will assume the field inherits
    /// access from its parent context.
    #[serde(skip_serializing_if = "Option::is_none")]
    access: Option<Access>,
    /// Field type
    #[serde(rename = "type")]
    field_type: FieldType,
    /// A single field object or an array of field objects. Used only when Field.FieldType is
    /// FieldType::Set.
    #[serde(skip_serializing_if = "Option::is_none")]
    contains: Option<OneOrMoreField>,
    /// The default value of the field
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
