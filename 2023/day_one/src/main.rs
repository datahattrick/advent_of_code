use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::{Path, PathBuf};
use std::error::Error;
use std::fmt;

/// A simple program to take in a text file and produce an advent of code outcome
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The file to parse
    #[arg(short, long)]
    path: PathBuf,
}

fn main() {
    let args = Cli::parse();

    let digits: Vec<Vec<u32>>;

    if let Ok(lines) = read_lines(args.path) {
        digits = get_u32_from_string(lines);
    } else {
        panic!("Failed to read lines from file")
    }

    let mut combined_digits: Vec<u32> = Vec::new();

    for line in digits {
        let combined_digit = match combine(line) {
            Ok(combined) => combined,
            Err(e) => {
                println!("Error: {}", e);
                return;
            },
        };
        combined_digits.push(combined_digit)
    }

    let total: u32 = combined_digits.iter().sum();

    println!("The answer is {}", total)

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_u32_from_string(lines: Lines<BufReader<File>>) -> Vec<Vec<u32>> {
    let mut digit_lines: Vec<Vec<u32>> = Vec::new();
    for line in lines.flatten() {
        let mut digit_line: Vec<u32> = Vec::new();
        for c in line.chars() {
            if let Some(digit) = c.to_digit(10) {
                digit_line.push(digit);
            }
        }
        digit_lines.push(digit_line);
    }
    return digit_lines;
}

#[derive(Debug)]
struct CombineError;

impl fmt::Display for CombineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Input vector is empty")
    }
}

impl Error for CombineError {}

fn combine(digits: Vec<u32>) -> Result<u32, Box<dyn Error>> {
    if let Some(first) = digits.first() {
        if let Some(last) = digits.last() {
            return Ok(format!("{}{}", first, last).parse::<u32>().unwrap());
        }
    }
    Err(Box::new(CombineError))
}