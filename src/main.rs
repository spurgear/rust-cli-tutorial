#![allow(unused)]
use clap::Parser;

/// search for a pattern in a given file and display the lines that contain it
#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let args: Cli = Cli::parse();
}
