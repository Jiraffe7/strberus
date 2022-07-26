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

    /// Number of lines that the string should have
    #[clap(short, long, value_parser)]
    lines: usize,

    /// Patterns that the string should contain
    #[clap(short, long, alias = "patterns", multiple_values = true, value_parser)]
    matches: Vec<String>,

    /// Patterns that the string should not contain
    #[clap(short, long, multiple_values = true, value_parser)]
    excludes: Vec<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let num_lines = args.lines;
    let matches: Vec<regex::Regex> = args
        .matches
        .into_iter()
        .map(|p| match Regex::new(&p) {
            Ok(re) => re,
            Err(e) => {
                eprintln!("Invalid pattern: {p}: {e}");
                std::process::exit(1);
            }
        })
        .collect();

    let excludes: Vec<regex::Regex> = args
        .excludes
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

    let matches: Vec<&str> = matches
        .iter()
        .filter_map(|p| {
            if p.is_match(&input) {
                None
            } else {
                Some(p.as_str())
            }
        })
        .collect();
    let excludes: Vec<(&str, &str)> = excludes
        .iter()
        .filter_map(|p| p.find(&input).map(|m| (p.as_str(), m.as_str())))
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
    excludes.iter().for_each(|(p, m)| {
        eprintln!("pattern `{p}` should not be matched but found `{m}`");
        invalid = true;
    });

    if invalid {
        Err("input is invalid".into())
    } else {
        Ok(())
    }
}
