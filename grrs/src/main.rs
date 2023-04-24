#![allow(unused)]
use clap::Parser;

// let pattern: std::string::String = std::env::args().nth(1).expect("no pattern given");
// let path: std::path::PathBuf = std::env::args().nth(2).expect("no path given");

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path).expect("could not read file");
    // println!("Hello, world!");
    for line in content.lines() {
        if line.contains(&args.pattern){
            println!("{}",line);
        }
    }
}
