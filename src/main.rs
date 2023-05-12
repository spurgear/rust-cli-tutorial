#![allow(unused)]
use clap::Parser;
use std::fs::read_to_string;

/// search for a pattern in a given file and display the lines that contain it
#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let args: Cli = Cli::parse();
    let content = read_to_string(&args.path).expect("could not read file");

    for (i, line) in content.lines().enumerate() {
        if line.contains((&args.pattern)) {
            println!("{}: {}",i, line);
        }
    }
}
