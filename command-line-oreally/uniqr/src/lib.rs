use clap::Parser;
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
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

pub fn get_args() -> MyResult<Config> {
    let matches = Config::parse();
    Ok(matches)
}

pub fn run(config: Config) -> MyResult<()> {
    print!("{:?}", config);
    Ok(())
}

pub fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
