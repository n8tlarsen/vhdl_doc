use serde_json;
use std::fs;
use toml;
use vhdl_doc::memory_map::schema::MemoryMap;

#[test]
pub fn toml_to_json() {
    let contents = fs::read_to_string("tests/assets/memory_map.toml").expect("Failed to read file");
    let memory_map: MemoryMap = toml::from_str(&contents).expect("Failed to parse TOML");
    println!(
        "{}",
        serde_json::to_string_pretty(&memory_map).expect("Failed to serialize to JSON string")
    );
}

#[test]
pub fn json_to_toml() {
    let contents = fs::read_to_string("tests/assets/memory_map.json").expect("Failed to read file");
    let memory_map: MemoryMap = serde_json::from_str(&contents).expect("Failed to parse JSON");
    println!(
        "{}",
        toml::to_string_pretty(&memory_map).expect("Failed to serialize to TOML string")
    );
}

#[test]
pub fn toml_eval() {
    let contents = fs::read_to_string("tests/assets/memory_map.json").expect("Failed to read file");
    let mut memory_map: MemoryMap = serde_json::from_str(&contents).expect("Failed to parse JSON");
    memory_map
        .elaborate()
        .expect("Failed to elaborate memory map for TOML document.");
    println!(
        "{}",
        serde_json::to_string_pretty(&memory_map).expect("Failed to serialize to JSON string")
    );
}
