use std::{
    error::Error,
    fs,
    io::{self, Read},
};

use clap::Parser;
use regex::Regex;

/// strberus is a simple program to validate that an input string meet the required specifications.
#[derive(Parser, Debug)]
struct Args {
    /// File to validate. Uses stdin if not specified
    #[clap(short, long, value_parser)]
    file: Option<String>,

    /// Number of lines that the commit message should have
    #[clap(short, long, value_parser)]
    lines: usize,

    /// Patterns that the commit message should contain
    #[clap(short, long, multiple_values = true, value_parser)]
    patterns: Vec<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let num_lines = args.lines;
    let patterns: Vec<regex::Regex> = args
        .patterns
        .into_iter()
        .map(|p| match Regex::new(&p) {
            Ok(re) => re,
            Err(e) => {
                eprintln!("Invalid pattern: {p}: {e}");
                std::process::exit(1);
            }
        })
        .collect();

    let input: String = if let Some(file) = args.file {
        fs::read_to_string(file)?
    } else {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf)?;
        buf
    };

    let matches: Vec<&str> = patterns
        .iter()
        .filter_map(|p| {
            if p.is_match(&input) {
                None
            } else {
                Some(p.as_str())
            }
        })
        .collect();
    let count = input.lines().count();

    let mut invalid = false;
    if count < num_lines {
        eprintln!("input with {count} lines less than required {num_lines} lines");
        invalid = true;
    }
    matches.iter().for_each(|p| {
        eprintln!("pattern `{p}` not matched");
        invalid = true;
    });

    if invalid {
        Err("input is invalid".into())
    } else {
        Ok(())
    }
}
