use clap::Parser;
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, Write},
};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Config {
    #[arg(default_value = "-", help = "input file")]
    in_file: String,

    #[arg(help = "output file")]
    out_file: Option<String>,

    #[arg(short, long, help = "show count")]
    count: bool,
}

#[derive(PartialEq, Debug)]
pub struct FileInfo {
    num_lines: usize,
    num_bytes: usize,
    num_chars: usize,
    num_words: usize,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Config::parse();
    Ok(matches)
}

pub fn run(config: Config) -> MyResult<()> {
    let mut file = open(&config.in_file).map_err(|e| format!("{}: {}", config.in_file, e))?;
    let mut out_file: Box<dyn Write> = match &config.out_file {
        Some(out_name) => Box::new(File::create(out_name)?),
        _ => Box::new(io::stdout()),
    };
    // know as buffer saving in memory, ill call it line
    let mut line = String::new();
    let mut prev_line = String::new();
    let mut count: u64 = 0;

    let mut print = |count: u64, text: &str| -> MyResult<()> {
        if count > 0 {
            if config.count {
                write!(out_file, "{:>4} {}", count, text)?;
            } else {
                write!(out_file, "{}", text)?;
            }
        }
        Ok(())
    };

    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }
        if line.trim_end() != prev_line.trim_end() {
            print(count, &prev_line)?;
            prev_line = line.clone();
            count = 0;
        }
        count += 1;
        line.clear()
    }
    print(count, &prev_line)?;
    Ok(())
}

pub fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
