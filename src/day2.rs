use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{alphanumeric0, digit1, multispace0, space1};
use nom::combinator::value;
use nom::IResult;
use nom::multi::separated_list0;
use nom::sequence::tuple;

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
enum CubeColor {
    #[default]
    None,
    Red,
    Green,
    Blue
}

#[derive(Default)]
#[derive(PartialEq)]
struct Cubes {
    cnt: u32,
    color: CubeColor,
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

fn cube_parser(input: &str) -> IResult<&str, Cubes> {
    tuple((space1, number, space1, color))(input).map(|(remaining, res)| {
        (remaining, Cubes{cnt: res.1, color: res.3})
    })
}

fn game_parser(input: &str) -> IResult<&str, u32> {
    tuple((tag("Game"), space1, number))(input).map(|(remaining, res)| {
        (remaining, res.2)
    })
}


#[cfg(test)]
mod tests {
    use nom::bytes::complete::take_until;
    use nom::error::Error;
    use crate::day2::{cube_parser, CubeColor, game_parser, group_parser, groups_parser};

    #[test]
    fn test_parse_cube() {
        let input = " 12 red";
        let res = cube_parser(input).unwrap_or_default();
        assert_eq!("", res.0);
        assert_eq!(CubeColor::Red, res.1.color)
    }

    #[test]
    fn test_parse_game() {
        let input = "Game 15";
        let res = game_parser(input).unwrap_or_default();
        assert_eq!(15, res.1)
    }

    #[test]
    fn test_day2() {
        let input = "Game 1: 12 red, 2 green, 5 blue; 9 red, 6 green, 4 blue; 10 red, 2 green, 5 blue; 8 blue, 9 red";

        let res = take_until::<_, _, Error<_>>(":")(input);
        let out = match res {
            Ok(x) => x,
            Err(_) => ("", "")
        };
        assert_eq!(out.1, "Game 1");

        let input2 = &out.0[1..]; // need to skip ':'
        let groups = groups_parser(input2).unwrap_or_default();

        for grp_str in groups.1 {
            let grp = group_parser(grp_str).unwrap_or_default();
            assert!(grp.1.len() > 0)
        }
    }
}