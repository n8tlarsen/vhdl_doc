use clap::Parser;
use std::{env, fs, path::PathBuf};
use schemars::schema_for;
use toml;

use vhdl_doc::memory_map::schema::{self, MemoryMap};
use vhdl_doc::symbol::symbol::make_symbol;

fn default_path(p: &str) -> PathBuf {
    let mut path = env::current_dir().unwrap();
    path.push(p);
    path
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = default_path(".").into_os_string())]
    source_path: PathBuf,
    #[arg(short, long, default_value = default_path("doc").into_os_string())]
    doc_path: PathBuf,
}

fn main() {
    let args = Args::parse();
    fs::create_dir_all(args.doc_path.clone()).unwrap();
    make_symbol(args.doc_path);
    let schema = schema_for!(schema::MemoryMap);
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}

