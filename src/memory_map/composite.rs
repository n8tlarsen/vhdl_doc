use crate::memory_map::resolved::ResolvedEntry;
use crate::memory_map::{Access, Field, HexStrOrUnsigned, IntegerOrString, Name, Protocol};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::{formats::PreferOne, serde_as, DefaultOnNull, OneOrMany};
use std::collections::HashMap;

/// The resolver trait provides a common API for resolving optional fields of composite document
/// elements
pub trait Resolver {
    /// Resolve the composite type for tabular presentation
    fn resolve(
        &self,
        address: &mut u64,
        table: &str,
        def_map: &HashMap<String, &Composite>,
        protocol: &Protocol,
    );
    /// Return the size of the composite in bytes
    fn size(&self, def_map: &HashMap<String, &Composite>) -> u64;
}

/// The Index describes a list or series expansion for use with the Array composite
#[serde_as]
#[derive(Deserialize, Serialize, JsonSchema, Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(untagged)]
pub enum Index {
    Length(u64),
    Range { high: u64, low: u64 },
    List(#[serde_as(as = "OneOrMany<Option<IntegerOrString>, PreferOne>")] Vec<Option<String>>),
}

/// Arrays allow for list or series expansions of other elements.
#[serde_as]
#[derive(Deserialize, Serialize, JsonSchema, Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "camelCase")]
pub struct Array {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde_as(as = "Option<HexStrOrUnsigned>")]
    address: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access: Option<Access>,
    elements: Box<Composite>,
    index: Index,
    /// Increment or stride length in bytes
    #[serde_as(as = "Option<HexStrOrUnsigned>")]
    increment: Option<u64>,
}

impl Resolver for Array {
    fn resolve(
        &self,
        address: &mut u64,
        table: &str,
        def_map: &HashMap<String, &Composite>,
        protocol: &Protocol,
    ) {
        let index_slice: Vec<Option<String>> = match &self.index {
            Index::Length(len) => (0u64..*len)
                .into_iter()
                .map(|x| Some(x.to_string()))
                .collect(),
            Index::Range { high, low } => (*low..*high)
                .into_iter()
                .map(|x| Some(x.to_string()))
                .collect(),
            Index::List(list) => list.to_vec(),
        };
        let increment = if let Some(incr) = self.increment {
            incr
        } else {
            (*self.elements).size(def_map)
        };
        for i in index_slice {
            if let Some(index_string) = i {
                (*self.elements).resolve(address, table, def_map, protocol);
            }
            *address += increment;
        }
        match &(*self.elements) {
            Composite::Entry(entry) => {}
            Composite::Array(array) => {}
            Composite::Cluster(cluster) => cluster.resolve(address, table, def_map, protocol),
            Composite::Reference { .. } => {}
            Composite::Map { .. } => {}
        }
    }
    fn size(&self, def_map: &HashMap<String, &Composite>) -> u64 {
        0u64
    }
}

impl Name for Array {
    fn name(&self) -> &str {
        &self.name
    }
    fn type_name(&self) -> &'static str {
        "Array"
    }
}

/// Clusters represent groupings of elements and are akin to structs.
#[serde_as]
#[derive(Deserialize, Serialize, JsonSchema, Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Cluster {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<HexStrOrUnsigned>")]
    address: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access: Option<Access>,
    #[serde_as(as = "OneOrMany<_,PreferOne>")]
    elements: Vec<Composite>,
}

impl Resolver for Cluster {
    fn resolve(
        &self,
        address: &mut u64,
        table: &str,
        def_map: &HashMap<String, &Composite>,
        protocol: &Protocol,
    ) {
        for item in self.into_iter() {
            match item {
                Composite::Entry(entry) => {}
                Composite::Array(array) => array.resolve(address, table, def_map, protocol),
                Composite::Cluster(cluster) => cluster.resolve(address, table, def_map, protocol),
                Composite::Reference { .. } => {}
                Composite::Map { .. } => {}
            }
        }
    }
    fn size(&self, def_map: &HashMap<String, &Composite>) -> u64 {
        0u64
    }
}

impl Name for Cluster {
    fn name(&self) -> &str {
        &self.name
    }
    fn type_name(&self) -> &'static str {
        "Cluster"
    }
}

impl IntoIterator for Cluster {
    type Item = Composite;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter()
    }
}

impl<'a> IntoIterator for &'a Cluster {
    type Item = &'a Composite;
    type IntoIter = std::slice::Iter<'a, Composite>;
    fn into_iter(self) -> Self::IntoIter {
        self.elements.iter()
    }
}

impl<'a> IntoIterator for &'a mut Cluster {
    type Item = &'a mut Composite;
    type IntoIter = std::slice::IterMut<'a, Composite>;
    fn into_iter(self) -> Self::IntoIter {
        self.elements.iter_mut()
    }
}

/// Entries are the fundamental unit of memory. They can be further described with field
/// definitions.
#[serde_as]
#[derive(Deserialize, Serialize, JsonSchema, Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Entry {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<HexStrOrUnsigned>")]
    address: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access: Option<Access>,
    /// Length of the entry in bytes
    bytes: u32,
    #[serde(default)]
    #[serde_as(as = "DefaultOnNull<OneOrMany<_,PreferOne>>")]
    fields: Vec<Field>,
}

impl Resolver for Entry {
    fn resolve(
        &self,
        address: &mut u64,
        table: &str,
        def_map: &HashMap<String, &Composite>,
        protocol: &Protocol,
    ) {
    }
    fn size(&self, _def_map: &HashMap<String, &Composite>) -> u64 {
        self.bytes.into()
    }
}

impl Name for Entry {
    fn name(&self) -> &str {
        &self.name
    }
    fn type_name(&self) -> &'static str {
        "Entry"
    }
}

/// The composite type collects the memory map types into an enum for serde processing
#[derive(Deserialize, Serialize, JsonSchema, Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(untagged)]
pub enum Composite {
    Array(Array),
    Cluster(Cluster),
    Entry(Entry),
    Reference {
        #[serde(rename = "*ref")]
        #[garde(pattern(r"[-_ A-Za-z0-9\/]*"))]
        reference: String,
    },
    Map {
        #[serde(rename = "*map")]
        #[garde(pattern(r"[-_ A-Za-z0-9\/]*"))]
        map: String,
    },
}

impl Resolver for Composite {
    fn resolve(
        &self,
        address: &mut u64,
        table: &str,
        def_map: &HashMap<String, &Composite>,
        protocol: &Protocol,
    ) {
        match self {
            Composite::Array(array) => array.resolve(address, table, def_map, protocol),
            Composite::Cluster(cluster) => cluster.resolve(address, table, def_map, protocol),
            Composite::Entry(entry) => entry.resolve(address, table, def_map, protocol),
            Composite::Reference { reference } => {}
            Composite::Map { map } => {}
        }
    }
    fn size(&self, def_map: &HashMap<String, &Composite>) -> u64 {
        0u64
    }
}

impl Name for Composite {
    fn name(&self) -> &str {
        match self {
            Composite::Array(array) => array.name(),
            Composite::Cluster(cluster) => cluster.name(),
            Composite::Entry(entry) => entry.name(),
            Composite::Reference { reference } => reference,
            Composite::Map { map } => map,
        }
    }
    fn type_name(&self) -> &'static str {
        match self {
            Composite::Array(array) => array.type_name(),
            Composite::Cluster(cluster) => cluster.type_name(),
            Composite::Entry(entry) => entry.type_name(),
            Composite::Reference { .. } => "Reference",
            Composite::Map { .. } => "Map",
        }
    }
}

impl Name for &Composite {
    fn name(&self) -> &str {
        match self {
            Composite::Array(array) => array.name(),
            Composite::Cluster(cluster) => cluster.name(),
            Composite::Entry(entry) => entry.name(),
            Composite::Reference { reference } => reference,
            Composite::Map { map } => map,
        }
    }
    fn type_name(&self) -> &'static str {
        match self {
            Composite::Array(array) => array.type_name(),
            Composite::Cluster(cluster) => cluster.type_name(),
            Composite::Entry(entry) => entry.type_name(),
            Composite::Reference { .. } => "Reference",
            Composite::Map { .. } => "Map",
        }
    }
}
