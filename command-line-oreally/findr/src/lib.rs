use crate::EntryType::*;
use clap::{Parser, ValueEnum};
use regex::Regex;
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Eq, PartialEq, Clone, ValueEnum)]
enum EntryType {
    #[value(name = "d")]
    Dir,
    #[value(name = "f")]
    File,
    #[value(name = "l")]
    Link,
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Config {
    #[arg(default_values = ["."], help="Search paths")]
    paths: Vec<String>,

    // looks like better use clap v2 and follow book, this doesn help
    #[arg(short, long = "name", help = "Name", value_parser=test_val)]
    names: Vec<Regex>,

    #[arg(value_enum, short = 't', long = "type", help = "Entry type")]
    entry_types: Vec<EntryType>,
}

pub fn get_args() -> MyResult<Config> {
    Ok(Config::parse())
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{config:#?}");
    Ok(())
}

fn test_val(s: &str) -> Result<String, String> {
    println!("test_val s: {s}");
    Ok(s.to_string())
}
