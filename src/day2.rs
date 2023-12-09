use std::collections::HashMap;
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{alphanumeric0, digit1, multispace0, space1};
use nom::combinator::value;
use nom::IResult;
use nom::multi::separated_list0;
use nom::sequence::tuple;
use crate::common;
use crate::day2::CubeColor::{Blue, Green, Red};

fn groups_parser(s: &str) -> IResult<&str, Vec<&str>> {
    separated_list0(tag(";"), is_not(";"))(s)
}

fn group_parser(s: &str) -> IResult<&str, Vec<&str>> {
    separated_list0(tag(","), is_not(","))(s)
}

#[derive(Clone)]
#[derive(Default)]
#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(Hash)]
enum CubeColor {
    #[default]
    None,
    Red,
    Green,
    Blue
}

#[derive(Default)]
#[derive(PartialEq)]
struct Cube {
    cnt: u32,
    color: CubeColor,
}

#[derive(Default)]
struct CubeSet {
    cubes: HashMap<CubeColor, u32>
}

impl CubeSet {
    fn possible(&self, g: &CubeSet) -> bool {
        for (k, v) in g.cubes.iter() {
            let cnt = self.cubes.get(k).unwrap_or(&0);
            if v > cnt {
                return false
            }
        }
        true
    }

    fn make(r: u32, g: u32, b: u32) -> CubeSet {
        let cubes: HashMap<CubeColor, u32> = [
            (Red, r),
            (Green, g),
            (Blue, b)
        ].iter().cloned().collect();
        CubeSet {cubes}
    }
}

fn number(input: &str) -> IResult<&str, u32> {
    digit1(input).map(|(remaining, number)| {
        // it can panic if the string represents a number
        // that does not fit into u32
        let n = number.parse().unwrap();
        (remaining, n)
    })
}

fn color(input: &str) -> IResult<&str, CubeColor> {
    alt((
        value(CubeColor::Red, tag("red")),
        value(CubeColor::Green,tag("green")),
        value(CubeColor::Blue,tag("blue"))
    ))(input)
}

fn cube_parser(input: &str) -> IResult<&str, Cube> {
    tuple((space1, number, space1, color))(input).map(|(remaining, res)| {
        (remaining, Cube {cnt: res.1, color: res.3})
    })
}

fn cube_set_parser(input: &str) -> IResult<&str, CubeSet> {
    group_parser(input).map(|(remaining, res)| {
        let cs: HashMap<CubeColor, u32> = res.iter().map(|cs| {
            match cube_parser(cs) {
                Ok(c) => (c.1.color, c.1.cnt),
                Err(_) => (CubeColor::None, 0)
            }
        }).collect();
        (remaining, CubeSet{cubes: cs})
    })
}

fn game_parser(input: &str) -> IResult<&str, u32> {
    tuple((tag("Game"), space1, number, tag(":")))(input).map(|(remaining, res)| {
        (remaining, res.2)
    })
}

fn process_line_day2(line: &str, check_set: &CubeSet) -> u32 {
    let (out, game_num) = match game_parser(line) {
        Ok(x) => x,
        Err(_) => ("", 0)
    };

    let groups = groups_parser(out).unwrap_or_default();

    for grp_str in groups.1 {
        let set = cube_set_parser(grp_str).unwrap_or_default();
        if !check_set.possible(&set.1) {
            return 0
        }
    }
    return game_num
}

pub fn do_day2() {
    let check_set = CubeSet::make(12, 13, 14);
    if let Ok(lines) = common::read_lines("./data/day2input.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut sum = 0;
        for line in lines {
            if let Ok(token) = line {
                let ret = process_line_day2(&token, &check_set);
                sum += ret
            }
        }
        println!("final sum: {}", sum);
    }
}


#[cfg(test)]
mod tests {
    use nom::bytes::complete::take_until;
    use nom::error::Error;
    use crate::day2::{cube_parser, cube_set_parser, CubeColor, CubeSet, game_parser, group_parser, groups_parser, process_line_day2};

    #[test]
    fn test_games_possible() {
        let bag = CubeSet::make(12, 13, 14);

        let turn1 = CubeSet::make(20, 8, 6);
        assert!(!bag.possible(&turn1));

        let turn2 = CubeSet::make(1, 2, 6);
        assert!(bag.possible(&turn2))
    }

    #[test]
    fn test_parse_cube() {
        let input = " 12 red";
        let res = cube_parser(input).unwrap_or_default();
        assert_eq!("", res.0);
        assert_eq!(CubeColor::Red, res.1.color)
    }

    #[test]
    fn test_parse_game() {
        let input = "Game 15:";
        let res = game_parser(input).unwrap_or_default();
        assert_eq!(15, res.1)
    }

    #[test]
    fn test_day2() {
        let input = "Game 1: 12 red, 2 green, 5 blue; 9 red, 6 green, 4 blue; 10 red, 2 green, 5 blue; 8 blue, 9 red";

        let check_set1 = CubeSet::make(10, 10, 10);
        let game1 = process_line_day2(input, &check_set1);
        assert_eq!(0, game1);

        let check_set2 = CubeSet::make(12, 12, 12);
        let game1 = process_line_day2(input, &check_set2);
        assert_eq!(1, game1);

    }
}