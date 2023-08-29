use clap::Parser;
// use std::error::Error;

// type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser,Debug)]
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
    #[arg(short = 'm', long, group = "cm" )]
    chars: bool,

    /// Show line count 
    #[arg(short, long, default_value_t = true)]
    lines: bool,
}

pub fn get_args() ->() {
  let matches = Config::parse();
  println!("{:#?}", matches)
}

// pub fn run(config: Config) -> MyResult<()> {}
