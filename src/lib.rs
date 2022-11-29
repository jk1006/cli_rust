use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn run(config: Config) -> MyResult<()> {
    for file in config.files {
        match open(&file) {
            Err(err) => eprintln!("Failed to open {}: {}", file, err),
            Ok(content) => {
                print_lines_of_file(content, config.number_lines, config.number_nonblank_lines);
            },
        }
    }
    Ok(())
}

fn print_lines_of_file(content: Box<dyn BufRead>, number_lines: bool, number_nonblank_lines: bool) {
    let mut line_count = 1;
    for (i, line) in content.lines().enumerate() {
        let mut result = "".to_string();
        let line_content = line.unwrap();
        if number_lines { 
            result = format!("\t{}\t", line_count);
        }
        if (number_nonblank_lines && !line_content.is_empty()) {
            result = format!("\t{}\t", line_count);
        } else if (number_nonblank_lines && line_content.is_empty()) {
            line_count -= 1;
        }
        line_count += 1;
        result += &line_content;
        println!("{}", result);

    }
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
            .conflicts_with("number_lines")
            .takes_value(false)
        )
        .arg(
            Arg::with_name("files")
            .value_name("FILES")
            .help("Input files")
            .required(false)
            .min_values(1)
        )
        .get_matches();

    let files = matches.values_of_lossy("files").unwrap_or(vec!["-".to_string()]);
    let number_lines = matches.is_present("number_lines");
    let number_nonblank_lines = matches.is_present("number_nonblank_lines");
    Ok(Config { files, number_lines, number_nonblank_lines })
}

fn open(filename : &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
