use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::value;
use nom::IResult;
use crate::common;

pub fn do_day1_part2() {
    if let Ok(lines) = common::read_lines("./data/day1input.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut sum = 0;
        for line in lines {
            if let Ok(token) = line {
                let ret = process_line_day1_part2(&token);
                println!("{}: {}, {}", token, ret.0, ret.1);
                sum += ret.0 * 10 + ret.1;
            }
        }
        println!("final sum: {}", sum);
    }
}

fn process_line_day1_simple(line: &String) -> (u32, u32) {
    let mut res = (0, 0);
    for c in line.chars() {
        if c.is_numeric() {
            res.0 = c.to_digit(10).unwrap();
            break;
        }
    }
    for c in line.chars().rev() {
        if c.is_numeric() {
            res.1 = c.to_digit(10).unwrap();
            break;
        }
    }
    res
}

fn parse_num(input: &str) -> IResult<&str, i32> {
    alt((
        value(0, tag("0")),
        value(0, tag("zero")),
        value(1, tag("1")),
        value(1, tag("one")),
        value(2, tag("2")),
        value(2, tag("two")),
        value(3, tag("3")),
        value(3, tag("three")),
        value(4, tag("4")),
        value(4, tag("four")),
        value(5, tag("5")),
        value(5, tag("five")),
        value(6, tag("6")),
        value(6, tag("six")),
        value(7, tag("7")),
        value(7, tag("seven")),
        value(8, tag("8")),
        value(8, tag("eight")),
        value(9, tag("9")),
        value(9, tag("nine")),
    ))(input)
}

// TODO: think of a better way to do this :/
fn parse_num_rev(input: &str) -> IResult<&str, i32> {
    alt((
        value(0, tag("0")),
        value(0, tag("orez")),
        value(1, tag("1")),
        value(1, tag("eno")),
        value(2, tag("2")),
        value(2, tag("owt")),
        value(3, tag("3")),
        value(3, tag("eerht")),
        value(4, tag("4")),
        value(4, tag("ruof")),
        value(5, tag("5")),
        value(5, tag("evif")),
        value(6, tag("6")),
        value(6, tag("xis")),
        value(7, tag("7")),
        value(7, tag("neves")),
        value(8, tag("8")),
        value(8, tag("thgie")),
        value(9, tag("9")),
        value(9, tag("enin")),
    ))(input)
}

fn proc_line(line: &str, f: &dyn Fn(&str) -> IResult<&str, i32>) -> i32 {
    for (idx, _) in line.char_indices() {
        let s = &line[idx..];
        match f(s) {
            Ok(x) => return x.1,
            Err(_) => continue
        }
    }
    0 // TODO: error handling???
}
fn process_line_day1_part2(line: &str) -> (i32, i32) {
    let rev_line = line.chars().rev().collect::<String>();
    (proc_line(line, &parse_num), proc_line(&rev_line, &parse_num_rev))
}

#[cfg(test)]
mod tests {
    use crate::day1::process_line_day1_part2;

    #[test]
    fn test_part1_day2() {
        let input = "1eighttwo8jfnhmfivefivezdsxqxqsjkone";
        let res = process_line_day1_part2(input);
        assert_eq!(1, res.0);
        assert_eq!(1, res.1);

        let input2 = "rtkrbtthree8sixfoureight6";
        let res2 = process_line_day1_part2(input2);
        assert_eq!(3, res2.0);
        assert_eq!(6, res2.1);

        let input3 = "six8threepvlxttc85two";
        let res3 = process_line_day1_part2(input3);
        assert_eq!(6, res3.0);
        assert_eq!(2, res3.1);

    }
}
