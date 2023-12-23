use std::cmp::Ordering;
use std::error::Error;
use nom::character::complete::multispace1;
use nom::multi::separated_list1;
use crate::common::number;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Day5Error {
    #[error("input file had unexpected or invalid format")]
    InputFormatError(&'static str)
}


// input is "dest source width"; returns (source, dest, width)
// TODO: error handling?
fn parse_map_line(line: String) -> (u64, u64, u64) {
    let (_, p) = separated_list1(multispace1, number::<u64>)(line.as_str()).unwrap_or_default();
    (p[1], p[0], p[2])
}

fn process_map_group(lines : impl std::iter::Iterator<Item = String>) -> Vec<(u64, u64, u64)> {
    let mut ret: Vec<(u64, u64, u64)> = lines.take_while(|l| !l.is_empty()).map(|l| {
        parse_map_line(l)
    }).collect();
    ret.sort_by(|a, b| a.0.cmp(&b.0));
    ret
}

fn process_day5_input(lines : impl std::iter::Iterator<Item = String>) -> Result<Vec<u64>, Day5Error> {

    let line_groups = lines.fold(vec![Vec::new()], |mut acc: Vec<Vec<String>>, line| {
        if line.is_empty() {
            let empty_vec: Vec<String> = Vec::new();
            acc.push(empty_vec)
        } else {
            let last = acc.len()-1;
            acc[last].push(line);
        }
        acc
    });
    if line_groups.is_empty() {
        return Err(Day5Error::InputFormatError("found no input a/o format was invalid"));
    }

    // first group is special - seeds
    let res_seeds = match line_groups[0].last() {
        Some(sl) => {
            match sl.strip_prefix("seeds: ") {
                Some(seeds) => separated_list1(multispace1, number::<u64>)(seeds).unwrap_or_default().1,
                None => return Err(Day5Error::InputFormatError("expected 'seeds: ' prefix"))
            }
        }
        None => return Err(Day5Error::InputFormatError("empty initial line"))
    };
    if res_seeds.is_empty() {
        return Err(Day5Error::InputFormatError("found no seeds or seeds list parsed incorrectly"))
    }

    for mg in &line_groups[1..] {
        if mg.is_empty() || mg[0].find("map:").is_none() {
            return Err(Day5Error::InputFormatError("invalid map group - no header found"))
        }
    }

    let maps: Vec<Vec<(u64,u64,u64)>> = line_groups[1..].iter().map(|mg| {
        process_map_group(mg[1..].iter().map(|s| s.to_string()))
    }).collect();

    let ret: Vec<u64> = res_seeds.iter().map(|s| map_with_groups(*s, &maps)).collect();
    Ok(ret)
}

fn find_by_source(m: &Vec<(u64, u64, u64)>, seek: u64) -> Option<usize> {
    match m.binary_search_by(|probe| {
        // find within range defined by source and width
        if seek >= probe.0 && seek < (probe.0+probe.2) {
            Ordering::Equal
        } else if probe.0 < seek {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }) {
        Ok(ord) => Some(ord),
        Err(_) => None
    }
}

fn map_by_source(m: &Vec<(u64, u64, u64)>, seek: u64) -> u64 {
    match find_by_source(m, seek) {
        Some(idx) => {
            seek - m[idx].0 + m[idx].1
        },
        None => seek
    }
}

fn map_with_groups(map_val: u64, maps: &Vec<Vec<(u64,u64,u64)>>) -> u64 {
    let mut ret = map_val;
    for m in maps {
        ret = map_by_source(&m, ret);
    }
    ret
}

#[cfg(test)]
mod tests {
    use nom::character::complete::multispace1;
    use nom::multi::separated_list1;
    use crate::common;
    use crate::common::number;
    use crate::day5::{find_by_source, map_by_source, process_day5_input, process_map_group};

    #[test]
    fn test_tuple_searching() {
        let mut maps: Vec<(u32, u32, u32)> = Vec::new();
        // tuple will be: source, dest, width
        maps.push((98, 50, 2));
        maps.push((50, 52, 48));

        maps.sort_by(|a, b| a.0.cmp(&b.0));
        assert_eq!(50, maps[0].0);
    }

    #[test]
    fn test_map_parsing() {
        let test_parse = separated_list1(multispace1, number::<u32>)("50 98 2");
        let res = test_parse.unwrap_or_default();
        assert_eq!(3, res.1.len());
    }

    #[test]
    fn test_group_parsing() {
        let test_seed_2_soil: Vec<&str> = vec![
            "50 98 2",
            "52 50 48",
            ""
        ];

        let map_seed_2_soil = process_map_group(test_seed_2_soil.iter().map(|s| s.to_string()));
        assert_eq!(50, map_seed_2_soil[0].0);

        let test_soil_2_fertilizer: Vec<&str> = vec![
            "0 15 37",
            "37 52 2",
            "39 0 15",
            ""
        ];

        let map_soil_2_fertilizer = process_map_group(test_soil_2_fertilizer.iter().map(|s| s.to_string()));
        assert_eq!(0, map_soil_2_fertilizer[0].0);

        let res = find_by_source(&map_soil_2_fertilizer, 16);
        assert_eq!(1, res.unwrap(), "should find the second map - (15, 0, 37)");

        // Seed number 79 corresponds to soil number 81.
        // Seed number 14 corresponds to soil number 14.
        // Seed number 55 corresponds to soil number 57.
        // Seed number 13 corresponds to soil number 13.
        let res1 = map_by_source(&map_seed_2_soil, 79);
        assert_eq!(81, res1);

        let res2 = map_by_source(&map_seed_2_soil, 14);
        assert_eq!(14, res2);

        let res3 = map_by_source(&map_seed_2_soil, 55);
        assert_eq!(57, res3);

        let res4 = map_by_source(&map_seed_2_soil, 13);
        assert_eq!(13, res4);
    }

    #[test]
    fn test_error() {
        let empty_input: Vec<String> = vec![];
        let expect_error = process_day5_input(empty_input.iter().map(|s| s.to_string()));
        assert!(expect_error.is_err());
    }

    #[test]
    fn test_example() {
        if let Ok(lines) = common::read_lines("./data/day5example.txt") {
            let lines_iter = lines.map(|l| l.unwrap()).into_iter();
            let res = process_day5_input(lines_iter);
            assert_eq!(vec![82, 43, 86, 35], res.unwrap());
        }
    }

    #[test]
    fn test_day5_input() {
        if let Ok(lines) = common::read_lines("./data/day5input.txt") {
            let lines_iter = lines.map(|l| l.unwrap()).into_iter();
            let res = process_day5_input(lines_iter);
            assert!(res.is_ok());
            let check = res.unwrap();
            assert_eq!(check.iter().min(), Some(&403695602));
        }
    }

}