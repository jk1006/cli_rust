use clap::{App, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn run(config: Config) -> MyResult<()> {
    dbg!(config);
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Julius Klotz")
        .about("Rust cat")
        .arg(
            Arg::with_name("number_lines")
            .short("n")
            .help("Show line numbers")
            .takes_value(false)
        )
        .arg(
            Arg::with_name("number_nonblank_lines")
            .short("b")
            .help("Show line numbers on blank lines")
            .takes_value(false)
        )
        .arg(
            Arg::with_name("files")
            .value_name("FILES")
            .help("Input files")
            .required(true)
            .min_values(1)
        )
        .get_matches();

    let files = matches.values_of_lossy("files").unwrap();
    let number_lines = matches.is_present("number_lines");
    let number_nonblank_lines = matches.is_present("number_nonblank_lines");
    Ok(Config { files, number_lines, number_nonblank_lines })
}
