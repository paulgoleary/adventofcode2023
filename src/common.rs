use std::fmt::Debug;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::num::ParseIntError;
use std::path::Path;
use std::str::FromStr;
use nom::IResult;
use nom::character::complete::digit1;
use thiserror::Error;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn number<T: FromStr + Default>(input: &str) -> IResult<&str, T> {
    Ok(digit1(input).map(|(remaining, number)| {
        match number.parse() {
            Ok(res) => (remaining, res),
            Err(e) => panic!(), // TODO: better error handling?
        }
    })?)
}

#[derive(Error, Debug)]
pub enum AoCError {
    #[error("input file had unexpected or invalid format")]
    InputFormatError(&'static str),
    #[error("input value had unexpected or invalid format")]
    InputValueError(String)
}

impl From<ParseIntError> for AoCError {
    fn from(value: ParseIntError) -> Self {
        AoCError::InputValueError(format!("input value is invalid or unexpected: {}", value))
    }
}
