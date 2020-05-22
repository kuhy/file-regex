use regex::bytes::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::process::exit;
use structopt::StructOpt;

/// Test if the given pattern matches the content of a file.
#[derive(StructOpt)]
struct Cli {
    /// The regex pattern
    pattern: String,
    /// The path to the file to read
    path: String,
}

fn main() {
    let args = Cli::from_args();
    let mut buffer = Vec::new();
    match File::open(&args.path) {
        Ok(mut content) => { content.read_to_end(&mut buffer).unwrap(); }
        Err(error) => {
            eprintln!("Error while reading a file: {}", error);
            exit(2);
        }
    };
    match Regex::new(&args.pattern) {
        Ok(regex) => {
            if regex.is_match(&buffer) {
                println!("Match!");
                exit(0);
            } else {
                eprintln!("The pattern does not match file contents!");
                exit(1);
            }
        }
        Err(error) => {
            eprintln!("Invalid pattern: {}", error);
            exit(1);
        }
    };
}
