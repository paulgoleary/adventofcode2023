use std::fmt::Debug;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::str::FromStr;
use nom::IResult;
use nom::character::complete::digit1;

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