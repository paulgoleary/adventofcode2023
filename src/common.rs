use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use nom::IResult;
use nom::character::complete::digit1;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn number(input: &str) -> IResult<&str, u32> {
    digit1(input).map(|(remaining, number)| {
        // it can panic if the string represents a number
        // that does not fit into u32
        let n = number.parse().unwrap();
        (remaining, n)
    })
}
