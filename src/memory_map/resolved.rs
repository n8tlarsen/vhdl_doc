use crate::memory_map::{
    composite::Resolver,
    field::{FieldType, Value},
    Access, ResolveError,
};
use anyhow::anyhow;
use derive_more::Display;
use std::collections::{BTreeMap, HashMap};
use std::fmt;
use std::io::Write;
use tabled::Tabled;

use super::{Array, Cluster, Composite, DisplayOption, Entry, Field, MemoryMap, Name};

#[derive(Debug, Display, Clone)]
pub enum LinkOrType {
    #[display("[{text}]({link})")]
    Link {
        text: String,
        link: String,
    },
    Type(FieldType),
}

#[derive(Debug, Clone)]
pub struct Range {
    start: u64,
    end: u64,
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.start == self.end {
            write!(f, "{}", self.start)
        } else {
            write!(f, "{}..{}", self.start, self.end)
        }
    }
}

#[derive(Tabled, Debug, Clone)]
#[tabled(rename_all = "Upper Title Case")]
pub struct ResolvedEntry {
    /// Name of the entry
    name: String,
    /// Address of the entry
    address: u64,
    /// Entry accessibility
    access: Access,
    /// Entry type
    #[tabled(rename = "Type")]
    entry_type: LinkOrType,
    /// The unit of measurement of a numeric type. Ignored for other types.
    unit: DisplayOption<String>,
    /// The minimum allowed value of a numeric type. Ignored for other types.
    minimum: DisplayOption<f64>,
    /// The maximum allowed value of a numeric type. Ignored for other types.
    maximum: DisplayOption<f64>,
    /// The default value of the entry.
    value: Value,
}

#[derive(Default)]
pub struct ResolvedMemoryMap {
    entries: BTreeMap<String, Vec<ResolvedEntry>>,
    fields: BTreeMap<String, Vec<Field>>,
}

impl ResolvedMemoryMap {
    pub fn append_to_entry_table(
        &mut self,
        key: &str,
        new: ResolvedEntry,
    ) -> Result<(), ResolveError> {
        if let Some(tb) = self.entries.get_mut(key) {
            tb.push(new);
            Ok(())
        } else {
            Err(ResolveError::nonexist_entry_table(key))
        }
    }

    pub fn new_entry_table(&mut self, name: &str) -> Result<(), ResolveError> {
        let duplicate = self.entries.insert(name.to_string(), Vec::new());
        if duplicate.is_some() {
            Err(ResolveError::duplicate_entry_table(name))
        } else {
            Ok(())
        }
    }

    pub fn append_to_field_table(&mut self, key: &str, new: Field) -> Result<(), ResolveError> {
        if let Some(tb) = self.fields.get_mut(key) {
            tb.push(new);
            Ok(())
        } else {
            Err(ResolveError::nonexist_entry_table(key))
        }
    }

    pub fn new_field_table(&mut self, name: &str) -> Result<(), ResolveError> {
        let duplicate = self.fields.insert(name.to_string(), Vec::new());
        if duplicate.is_some() {
            Err(ResolveError::duplicate_field_table(name))
        } else {
            Ok(())
        }
    }

    pub fn render(&self) -> String {
        // self.render_recursive()
        "".to_string()
    }

    pub fn render_to_writer<W, E>(&self, writer: W) -> Result<(), E>
    where
        W: Write,
        E: std::error::Error,
    {
        Result::Ok(())
    }

    pub fn resolve(mm: &MemoryMap) -> Result<Self, ResolveError> {
        let mut resolved = ResolvedMemoryMap::default();
        let base_address = 0u64;
        let mut address = 0u64;
        let def_map = mm.get_def_map()?;
        // Recursively resolve the map
        for item in mm.map.iter() {
            match item {
                Composite::Entry(entry) => {
                    entry.resolve(&mut address, "Anonymous", &def_map, &mm.protocol);
                }
                Composite::Array(array) => {
                    array.resolve(&mut address, "Anonymous", &def_map, &mm.protocol);
                }
                Composite::Cluster(cluster) => {
                    let name = cluster.name();
                    resolved.new_entry_table(name)?;
                    cluster.resolve(&mut address, name, &def_map, &mm.protocol);
                }
                Composite::Reference { .. } => {}
                Composite::Map { .. } => {}
            }
        }
        Ok(resolved)
    }
}

pub struct MemoryTableIter {
    entry_iter: <BTreeMap<String, Vec<ResolvedEntry>> as IntoIterator>::IntoIter,
    field_iter: <BTreeMap<String, Vec<Field>> as IntoIterator>::IntoIter,
    next_field: bool,
}

pub enum EntryOrField {
    Entry((String, Vec<ResolvedEntry>)),
    Field((String, Vec<Field>)),
}

impl Iterator for MemoryTableIter {
    type Item = EntryOrField;
    fn next(&mut self) -> Option<Self::Item> {
        match self.next_field {
            false => match self.entry_iter.next() {
                Some(table) => Some(EntryOrField::Entry(table)),
                None => {
                    self.next_field = true;
                    self.next()
                }
            },
            true => self.field_iter.next().map(EntryOrField::Field),
        }
    }
}

impl IntoIterator for ResolvedMemoryMap {
    type Item = EntryOrField;
    type IntoIter = MemoryTableIter;
    fn into_iter(self) -> Self::IntoIter {
        MemoryTableIter {
            entry_iter: self.entries.into_iter(),
            field_iter: self.fields.into_iter(),
            next_field: false,
        }
    }
}

impl Extend<EntryOrField> for ResolvedMemoryMap {
    fn extend<T: IntoIterator<Item = EntryOrField>>(&mut self, iter: T) {
        for item in iter {
            match item {
                EntryOrField::Entry(entry) => {
                    self.entries.insert(entry.0, entry.1);
                }
                EntryOrField::Field(field) => {
                    self.fields.insert(field.0, field.1);
                }
            }
        }
    }
}
