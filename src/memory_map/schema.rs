use anyhow::anyhow;
use log::{debug, error, info, warn};
use schemars::schema_for;
use schemars::JsonSchema;
use serde::de::Error;
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
    /// String type; value is the length of the string in bytes.
    String(u64),
    /// Enumerated type
    /// Represented by the vhdl type `std_logic_vector(length-1 downto 0)`
    Enum {
        length: u32,
        map: HashMap<String, u32>,
    },
    /// Bitfield with named indices
    /// Represented by the vhdl type `std_logic_vector(length-1 downto 0)`
    Bitfield { length: u32, bits: BitfieldStyle },
    /// Unsigned numeric type; value is length of the field in bits.
    /// Defined by length and representing the vhdl type `signed(length-1 downto 0)`.
    Unsigned(u32),
    /// Signed numeric type; value is length of the field in bits.
    /// Defined by length and representing the vhdl type `unsigned(length-1 downto 0)`
    Signed(u32),
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
    UFixed { high: i32, low: i32 },
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
    SFixed { high: i32, low: i32 },
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(untagged)]
pub enum OneOrMoreField {
    One(Box<Field>),
    More(Vec<Field>),
}

fn ascii_only_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let string = String::deserialize(deserializer)?;
    if string.is_ascii() {
        Ok(string)
    } else {
        Err(D::Error::custom(format!(
            "string {} contains non-ascii characters",
            string
        )))
    }
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(untagged)]
pub enum Value {
    #[serde(deserialize_with = "ascii_only_string")]
    String(String),
    Unsigned(u64),
    Signed(i64),
    Float(f64),
}

#[derive(Deserialize, Serialize, JsonSchema, Default, Debug, Copy, Clone)]
pub enum Access {
    /// Read-only access is permitted
    #[default]
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
    /// required to explicitly specify the address. If no prior field exists, the renderer will
    /// either inherit the address of the parent FieldType::Set or start at zero.
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
    /// The default value of the field. Ignored for FieldType::Set
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<Value>,
    /// The unit of measurement of a numeric type. Ignored for other types.
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: Option<String>,
    /// The minimum allowed value of a numeric type. Ignored for other types.
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<f64>,
    /// The maximum allowed value of a numeric type. Ignored for other types.
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<f64>,
    /// Rendered range. For sets, this is the minimum and maxiumum addresses contained within the
    /// set. For numeric types this is the minimum and maximum values. For other types, this is a
    /// description of possible values.
    #[serde(skip_deserializing)]
    range: String,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct MemoryMap {
    protocol: Protocol,
    #[serde(flatten)]
    field: Field,
}

impl Field {
    pub fn render(&mut self, protocol: &Protocol) -> Result<(), anyhow::Error> {
        self.render_recursive(
            protocol,
            &self.access.unwrap_or_default(),
            &mut self.address.unwrap_or_default(),
        )
    }

    fn render_recursive(
        &mut self,
        protocol: &Protocol,
        parent_access: &Access,
        running_address: &mut u64,
    ) -> Result<(), anyhow::Error> {
        match &mut self.field_type {
            FieldType::Set => {
                if let Some(container) = &mut self.contains {
                    match container {
                        OneOrMoreField::One(field) => {
                            (**field).render_recursive(protocol, parent_access, running_address)
                        }
                        OneOrMoreField::More(fields) => {
                            for field in fields.iter_mut() {
                                (*field).render_recursive(
                                    protocol,
                                    parent_access,
                                    running_address,
                                )?;
                            }
                            Ok(())
                        }
                    }
                } else {
                    let error = anyhow!(
                        "Schema error. Field type 'set' was provided, but key 'contains' was not"
                    );
                    error!("{}", error);
                    Err(error)
                }
            }
            &mut FieldType::String(length) => {
                self.render_field_type_string(&length, protocol, parent_access, running_address)
            }
            FieldType::Enum { length, map } => Ok(()),
            FieldType::Bitfield { length, bits } => Ok(()),
            &mut FieldType::Unsigned(length) => {
                self.render_field_type_unsigned(&length, protocol, parent_access, running_address)
            }
            FieldType::Signed(length) => Ok(()),
            FieldType::UFixed { high, low } => Ok(()),
            FieldType::SFixed { high, low } => Ok(()),
        }
    }

    fn render_field_type_string(
        &mut self,
        length: &u64,
        protocol: &Protocol,
        parent_access: &Access,
        running_address: &mut u64,
    ) -> Result<(), anyhow::Error> {
        // Validate the value and length
        if let Some(value) = &self.value {
            if let Value::String(string) = value {
                if (string.len() as u64) > *length {
                    let error = anyhow!("provided string value is longer than the field type");
                    error!("{}", error);
                    return Err(error);
                }
            } else {
                let error = anyhow!("provided value doesn't match the field type");
                error!("{}", error);
                return Err(error);
            }
        }
        // Render access field
        if self.access.is_none() {
            self.access = Some(*parent_access)
        }
        // Update the addresses
        let my_address = if self.address.is_some() {
            self.address.unwrap()
        } else {
            self.address = Some(*running_address);
            *running_address
        };
        if (my_address + *length) > protocol.address_max {
            let error = anyhow!(format!(
                "Field {} with address {} and length {} would overflow the protocol maximum address {}",
                self.name,
                my_address,
                *length,
                protocol.address_max,
            ));
            error!("{}", error);
            return Err(error);
        }
        *running_address = my_address + *length;
        Ok(())
    }

    fn render_field_type_unsigned(
        &mut self,
        length: &u32,
        protocol: &Protocol,
        parent_access: &Access,
        running_address: &mut u64,
    ) -> Result<(), anyhow::Error> {
        // Validate the value and length
        if let Some(value) = &self.value {
            if let Value::Unsigned(number) = value {
                if *number > 2u64.pow(*length) {
                    let error = anyhow!(format!(
                        "numeric value {} requires more than {} bits specified by the field type",
                        *number, *length
                    ));
                    error!("{}", error);
                    return Err(error);
                }
            } else {
                let error = anyhow!("provided value doesn't match the field type");
                error!("{}", error);
                return Err(error);
            }
        }
        // Render access field
        if self.access.is_none() {
            self.access = Some(*parent_access)
        }
        // Update the addresses
        let my_address = if self.address.is_some() {
            self.address.unwrap()
        } else {
            self.address = Some(*running_address);
            *running_address
        };
        let mut bytes = ((*length as f32) / 8f32).ceil() as u32;
        if bytes < protocol.data_min as u32 {
            bytes = protocol.data_min as u32;
        }
        if (my_address + (bytes as u64)) > protocol.address_max {
            let error = anyhow!(format!(
                "Field {} with address {} and length {} would overflow the protocol maximum address {}",
                self.name,
                my_address,
                *length,
                protocol.address_max,
            ));
            error!("{}", error);
            return Err(error);
        }
        *running_address = my_address + (bytes as u64);
        Ok(())
    }
}

pub fn get_memory_map_schema() -> String {
    let schema = schema_for!(MemoryMap);
    let formatter = PrettyFormatter::with_indent(b"    ");
    let mut buf = Vec::new();
    let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
    serde::Serialize::serialize(&schema, &mut ser).expect("Failed to serialize schema");
    String::from_utf8(buf).expect("Failed to convert serial buffer to string")
}
