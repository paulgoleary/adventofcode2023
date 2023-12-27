use std::cmp::max;
use std::collections::HashMap;
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::space1;
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

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
enum CubeColor {
    #[default]
    None,
    Red,
    Green,
    Blue
}

#[derive(Default, PartialEq)]
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

    fn merge_max(&self, cs: &CubeSet) -> CubeSet {
        let r_cnt = max(self.cubes.get(&Red).unwrap_or(&0), cs.cubes.get(&Red).unwrap_or(&0));
        let g_cnt = max(self.cubes.get(&Green).unwrap_or(&0), cs.cubes.get(&Green).unwrap_or(&0));
        let b_cnt = max(self.cubes.get(&Blue).unwrap_or(&0), cs.cubes.get(&Blue).unwrap_or(&0));
        CubeSet::make(*r_cnt, *g_cnt, *b_cnt)
    }

    fn power(&self) -> u32 {
        self.cubes.get(&Red).unwrap_or(&0)
            * self.cubes.get(&Green).unwrap_or(&0)
            * self.cubes.get(&Blue).unwrap_or(&0)
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

fn color(input: &str) -> IResult<&str, CubeColor> {
    alt((
        value(CubeColor::Red, tag("red")),
        value(CubeColor::Green,tag("green")),
        value(CubeColor::Blue,tag("blue"))
    ))(input)
}

fn cube_parser(input: &str) -> IResult<&str, Cube> {
    tuple((space1, common::number, space1, color))(input).map(|(remaining, res)| {
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
    tuple((tag("Game"), space1, common::number, tag(":")))(input).map(|(remaining, res)| {
        (remaining, res.2)
    })
}

fn process_line_day2(line: &str, check_set: &CubeSet) -> u32 {
    let mut parser = tuple((game_parser, groups_parser));
    let (out, (game_num, groups)) = parser(line).unwrap_or_default();

    for grp_str in groups {
        let set = cube_set_parser(grp_str).unwrap_or_default();
        if !check_set.possible(&set.1) {
            return 0
        }
    }
    return game_num
}

fn process_line_day2_part2(line: &str) -> CubeSet {
    let mut parser = tuple((game_parser, groups_parser));
    let (out, (game_num, groups)) = parser(line).unwrap_or_default();

    let mut max_set = CubeSet::make(0,0,0);
    for grp_str in groups {
        let (_, set) = cube_set_parser(grp_str).unwrap_or_default();
        max_set = max_set.merge_max(&set);
    }
    return max_set
}

pub fn do_day2() {
    if let Ok(lines) = common::read_lines("./data/day2input.txt") {
        let mut sum = 0;
        for line in lines {
            if let Ok(token) = line {
                let ret = process_line_day2_part2(&token);
                sum += ret.power()
            }
        }
        println!("final sum: {}", sum);
    }
}

#[cfg(test)]
mod tests {
    use nom::sequence::tuple;
    use crate::day2::{cube_parser, CubeColor, CubeSet, do_day2, game_parser, groups_parser, process_line_day2, process_line_day2_part2};

    #[test]
    fn test_do_day2() {
        do_day2()
    }
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
    fn test_line_parsing() {
        let input = "Game 1: 12 red, 2 green, 5 blue; 9 red, 6 green, 4 blue; 10 red, 2 green, 5 blue; 8 blue, 9 red";

        let mut parser = tuple((game_parser, groups_parser));

        let (out, (game_id, groups)) = parser(input).unwrap_or_default();
        assert_eq!("", out);
        assert_eq!(1, game_id);
        assert_eq!(4, groups.len());
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

    #[test]
    fn test_day2_part2() {
        let input = "Game 1: 12 red, 2 green, 5 blue; 9 red, 6 green, 4 blue; 10 red, 2 green, 5 blue; 8 blue, 9 red";
        let check_set = process_line_day2_part2(input);
        assert_eq!(576, check_set.power());
    }

    #[test]
    fn test_merge_max() {

        let c1 = CubeSet::make(10, 10, 10);
        let c2 = CubeSet::make(5, 15, 5);

        let c_max = c1.merge_max(&c2);

        assert_eq!(10, c_max.cubes[&CubeColor::Red]);
        assert_eq!(15, c_max.cubes[&CubeColor::Green]);
        assert_eq!(10, c_max.cubes[&CubeColor::Blue]);

        let c3 = CubeSet::make(10, 0, 0);

        let c_max2 = c3.merge_max(&c2) ;

        assert_eq!(10, c_max2.cubes[&CubeColor::Red]);
        assert_eq!(15, c_max2.cubes[&CubeColor::Green]);
        assert_eq!(5, c_max2.cubes[&CubeColor::Blue]);

    }
}