pub mod composite;
pub mod field;
pub mod protocol;
pub mod resolved;
pub mod serde_helpers;

pub use composite::{Array, Cluster, Composite, Entry};
pub use field::Field;
pub use protocol::Protocol;
pub use serde_helpers::{DisplayOption, EnumMap, HexStrOrUnsigned, IntegerOrString};

use derive_more::Display;
use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};
use serde_json::ser::PrettyFormatter;
use serde_with::{formats::PreferOne, serde_as, DefaultOnNull, OneOrMany};
use std::collections::{BTreeMap, HashMap};
use std::fmt;

#[derive(Deserialize, Serialize, JsonSchema, Display, Default, Debug, Copy, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Access {
    /// Read-only access is permitted
    #[default]
    #[serde(rename = "r")]
    #[display("Read-only")]
    Read,
    /// Write-only access is permitted
    #[serde(rename = "w")]
    #[display("Write-only")]
    Write,
    /// Both read and write access is permitted
    #[serde(rename = "rw")]
    #[display("Read/Write")]
    ReadWrite,
}

#[derive(Debug, Clone)]
pub struct ResolveError {
    message: String,
}

impl ResolveError {
    fn duplicate<T>(_first: T, second: T) -> Self
    where
        T: Name,
    {
        let info = match second.type_name() {
            "Cluster" => " Resolve the conflict or consider using a reference.",
            _ => "",
        };
        ResolveError {
            message: format!(
                "Found duplicate {} instances with name \"{}\".{}",
                second.type_name().to_lowercase(),
                second.name(),
                info
            ),
        }
    }
    fn duplicate_entry_table(name: &str) -> Self {
        ResolveError {
            message: format!("Found duplicate cluster name \"{}\". Resolve the conflict or consider using a reference.", name),
        }
    }
    fn duplicate_field_table(name: &str) -> Self {
        ResolveError {
            message: format!("Found duplicate entry name \"{}\". Resolve the conflict or consider using a reference.", name),
        }
    }
    fn nonexist_entry_table(name: &str) -> Self {
        ResolveError {
            message: format!(
                "Internal error while adding entry; cluster name \"{}\" does not exist.",
                name
            ),
        }
    }
    fn nonexist_field_table(name: &str) -> Self {
        ResolveError {
            message: format!(
                "Internal error while adding field; entry name \"{}\" does not exist.",
                name
            ),
        }
    }
    fn def_not_found(item: String) -> Self {
        ResolveError {
            message: format!("Definition {} not found in document", item),
        }
    }
    fn map_not_found(item: String) -> Self {
        ResolveError {
            message: format!("Map file {} not found", item),
        }
    }
}

impl fmt::Display for ResolveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[serde_as]
#[derive(Deserialize, Serialize, JsonSchema, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct MemoryMap {
    #[serde(flatten)]
    pub(crate) protocol: Protocol,
    #[serde(rename = "&map")]
    #[serde_as(as = "OneOrMany<_,PreferOne>")]
    pub(crate) map: Vec<Composite>,
    #[serde(rename = "&def")]
    #[serde(default)]
    #[serde_as(as = "DefaultOnNull<OneOrMany<_,PreferOne>>")]
    pub(crate) def: Vec<Composite>,
}

impl MemoryMap {
    pub fn get_def_map(&self) -> Result<HashMap<String, &Composite>, ResolveError> {
        let mut def_map = HashMap::with_capacity(self.def.len());
        for def in &self.def {
            let def_string = def.name().to_string();
            if let Some(residual) = def_map.insert(def_string, def) {
                return Err(ResolveError::duplicate(def, residual));
            }
        }
        Ok(def_map)
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

pub trait Name {
    fn name(&self) -> &str;
    fn type_name(&self) -> &'static str;
}
