use clap::Parser;
use std::{env, path::PathBuf};

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
    #[arg(short, long, default_value = default_path("docs").into_os_string())]
    docs_path: PathBuf,
}

fn main() {
    let args = Args::parse();
    println!("Source: {:?}, Output {:?}", args.source_path, args.docs_path);
}
