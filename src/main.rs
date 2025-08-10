use clap::Parser;
use std::{env, fs, path::PathBuf};
use vhdl_doc::memory_map::schema::get_memory_map_schema;
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
    println!("{}", get_memory_map_schema());
}
