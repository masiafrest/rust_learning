use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Config {
    /// Input file(s)
    #[arg(default_values = ["-"])]
    files: Vec<String>,

    /// Show word count
    #[arg(short, long, default_value_t = true)]
    words: bool,

    /// Show byte count
    #[arg(short = 'c', long, group = "cm", default_value_t = true)]
    bytes: bool,

    /// Show character count
    #[arg(short = 'm', long, group = "cm")]
    chars: bool,

    /// Show line count
    #[arg(short, long, default_value_t = true)]
    lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Config::parse();
    Ok(matches)
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in &config.files {
        match open(filename) {
            Err(e) => eprintln!("{}, {}", filename, e),
            Ok(_) => println!("opened {}", filename),
        }
    }

    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    // println!("{}", filename);
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
