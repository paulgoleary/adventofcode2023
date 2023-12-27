use std::cmp::{min, Ordering};
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

fn process_day5_input(lines : impl Iterator<Item = String>) -> Result<(Vec<u64>, Vec<Vec<RangeMap>>), Day5Error> {
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

    let maps: Vec<Vec<RangeMap>> = line_groups[1..].iter().map(|mg| {
        RangeMap::from_map_group(mg[1..].iter().map(|s| s.to_string()))
    }).collect();

    Ok((res_seeds, maps))
}
fn map_day5_part1_input(lines : impl Iterator<Item = String>) -> Result<Vec<u64>, Day5Error> {
    let (res_seeds, maps) = process_day5_input(lines)?;
    let ret: Vec<u64> = res_seeds.iter().map(|s| map_with_groups(*s, &maps)).collect();
    Ok(ret)
}

fn find_by_source(m: &Vec<RangeMap>, seek: u64) -> Option<usize> {
    match m.binary_search_by(|probe| {
        // find within range defined by source and width
        if seek >= probe.src && seek < (probe.src+probe.width) {
            Ordering::Equal
        } else if probe.src < seek {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }) {
        Ok(ord) => Some(ord),
        Err(_) => None
    }
}

fn map_by_source(m: &Vec<RangeMap>, seek: u64) -> u64 {
    match find_by_source(m, seek) {
        Some(idx) => {
            seek - m[idx].src + m[idx].dest
        },
        None => seek
    }
}

fn map_with_groups(map_val: u64, maps: &Vec<Vec<RangeMap>>) -> u64 {
    let mut ret = map_val;
    for m in maps {
        ret = map_by_source(&m, ret);
    }
    ret
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Range {
    start: u64,
    width: u64
}

impl Range {
    fn new(start: u64, width: u64) -> Range {
        Range{start, width}
    }
    fn begin(&self) -> u64 {
        self.start
    }
    fn end(&self) -> u64 {
        self.start + self.width
    }

    // return true if any part of self overlaps target
    fn overlaps(&self, target: &Range) -> bool {
        if self.begin() >= target.begin() && self.begin() < target.end() {
            return true
        } else if self.end() > target.begin() && self.end() <= target.end() {
            return true
        }
        false
    }

    // return true if check spans target - ie. target is completely within check
    fn spans(&self, target: &Range) -> bool {
        target.begin() >= self.begin() && target.end() < self.end()
    }

    // find maps that apply (overlap or spanned) to this range
    fn find_range_maps<'a>(&self, target: &'a Vec<RangeMap>) -> Vec<&'a RangeMap> {
        let find: Vec<&RangeMap> = target.iter().filter(|target| {
            self.overlaps(&target.src_range() ) || self.spans(&target.src_range())
        }).collect();
        find
    }

    // calculates map of range and returns 'unmapped' portion
    fn map_range(&self, rm: &RangeMap) -> (Range, Range) {
        if self.start >= rm.src {
            let off = self.start - rm.src;
            let map_width = min(rm.width - off, self.width);
            (Range::new(rm.dest+off, map_width), Range::new(rm.src+rm.width,self.width-map_width))
        } else if self.end() > rm.src {
            let map_width = self.end() - rm.src;
            (Range::new(rm.dest, map_width), Range::new(self.start,self.width-map_width))
        } else {
            (Range::new(0,0), Range::new(self.start,self.width))
        }
    }

    fn map_ranges(start_range: &Range, maps: &Vec<RangeMap>) -> Vec<Range> {
        let mut ret: Vec<Range> = Vec::new();
        let mut current_range = *start_range;
        for rm in maps {
            if current_range.overlaps(&rm.src_range()) || current_range.spans(&rm.src_range()) {
                let (res_mapped, res_unmapped) = current_range.map_range(rm);
                if res_mapped.width != 0 {
                    ret.push(res_mapped)
                }
                current_range = res_unmapped;
            }
        }
        if current_range.width > 0 {
            ret.push(current_range);
        }
        ret
    }

    fn map_multi_ranges(ranges: &Vec<Range>, maps: &Vec<RangeMap>) -> Vec<Range> {
        ranges.iter().flat_map(|r| Range::map_ranges(r, maps)).collect()
    }
}

impl From<(u64,u64)> for Range {
    fn from(value: (u64, u64)) -> Self {
        Range{start: value.0, width: value.1}
    }
}

#[derive(PartialEq, Debug)]
struct RangeMap {
    src: u64,
    dest: u64,
    width: u64
}

impl RangeMap {
    fn src_range(&self) -> Range {
        Range{start: self.src, width: self.width}
    }
    fn dest_range(&self) -> Range {
        Range{start: self.dest, width: self.width}
    }

    // input is "dest source width"; returns (source, dest, width)
// TODO: error handling?
    fn from_map_line(line: String) -> RangeMap {
        let (_, p) = separated_list1(multispace1, number::<u64>)(line.as_str()).unwrap_or_default();
        (p[1], p[0], p[2]).into()
    }

    fn from_map_group(lines : impl Iterator<Item = String>) -> Vec<RangeMap> {
        let mut ret: Vec<RangeMap> = lines.take_while(|l| !l.is_empty()).map(|l| {
            RangeMap::from_map_line(l).into()
        }).collect();
        ret.sort_by(|a, b| a.src.cmp(&b.src));
        ret
    }
}

impl From<(u64,u64,u64)> for RangeMap {
    fn from(value: (u64,u64,u64)) -> Self {
        RangeMap{src: value.0, dest: value.1, width: value.2}
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::{max, min};
    use nom::character::complete::multispace1;
    use nom::multi::separated_list1;
    use crate::common;
    use crate::common::number;
    use crate::day5::{find_by_source, map_by_source, map_day5_part1_input, process_day5_input, Range, RangeMap};

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

        let map_seed_2_soil = RangeMap::from_map_group(test_seed_2_soil.iter().map(|s| s.to_string()));
        assert_eq!(50, map_seed_2_soil[0].src);

        let test_soil_2_fertilizer: Vec<&str> = vec![
            "0 15 37",
            "37 52 2",
            "39 0 15",
            ""
        ];

        let map_soil_2_fertilizer = RangeMap::from_map_group(test_soil_2_fertilizer.iter().map(|s| s.to_string()));
        assert_eq!(0, map_soil_2_fertilizer[0].src);

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
        let expect_error = map_day5_part1_input(empty_input.iter().map(|s| s.to_string()));
        assert!(expect_error.is_err());
    }

    #[test]
    fn test_example() {
        if let Ok(lines) = common::read_lines("./data/day5example.txt") {
            let lines_iter = lines.map(|l| l.unwrap()).into_iter();
            let res = map_day5_part1_input(lines_iter);
            assert_eq!(vec![82, 43, 86, 35], res.unwrap());
        }
    }

    #[test]
    fn test_day5_mapping() {
        if let Ok(lines) = common::read_lines("./data/day5input.txt") {
            let lines_iter = lines.map(|l| l.unwrap()).into_iter();
            let res = map_day5_part1_input(lines_iter);
            assert!(res.is_ok());
            let check = res.unwrap();
            assert_eq!(check.iter().min(), Some(&403695602));
        }
    }

    #[test]
    fn test_part2_example() {
        let tests = vec![("./data/day5example.txt", 46)];
        for tt in tests {
            if let Ok(lines) = common::read_lines(tt.0) {
                let lines_iter = lines.map(|l| l.unwrap()).into_iter();
                let (res_seeds, res_maps) = process_day5_input(lines_iter).unwrap();

                // brute force the ranges ...
                let mut lowest = u64::MAX;
                for seed_range in res_seeds.chunks(2) {
                    for x in seed_range[0]..seed_range[0]+seed_range[1] {
                        let mut ret = x;
                        for m in &res_maps {
                            ret = map_by_source(&m, ret);
                        }
                        if ret < lowest {
                            lowest = ret
                        }
                    }
                }

                assert_eq!(lowest, tt.1);
            }
        }
    }

    #[test]
    fn test_part2_example_mapping() {
        let tests = vec![
            ("./data/day5example.txt", 46u64),
            ("./data/day5input.txt", 219529182u64),
            ("./data/day5input-jhrcook.txt", 1240035u64)
        ];
        for tt in tests {
            if let Ok(lines) = common::read_lines(tt.0) {
                let lines_iter = lines.map(|l| l.unwrap()).into_iter();
                let (res_seeds, res_maps) = process_day5_input(lines_iter).unwrap();

                let mut lowest = u64::MAX;
                for seed_range in res_seeds.chunks(2) {
                    let mut ranges = vec![Range::new(seed_range[0], seed_range[1])];
                    for rm in &res_maps {
                        ranges = Range::map_multi_ranges(&ranges, &rm);
                    }
                    for r in ranges {
                        if r.start < lowest {
                            lowest = r.start
                        }
                    }
                }
                assert_eq!(lowest, tt.1);
            }
        }
    }

    #[test]
    fn test_filter_range_maps() {
        // (0, 10), (20, 30)
        let target1: Vec<RangeMap> = vec![(0,100,10).into(), (20,200,10).into()];

        // (15, 20) - result should be unmapped value
        let check1 = Range::map_ranges(&Range::new(15,5), &target1);
        assert_eq!(check1.len(), 1);
        assert_eq!(check1[0], (15,5).into());

        // (5, 25) - overlaps both. should return 3 maps
        let mut check2 = Range::map_ranges(&Range::new(5,20), &target1);
        check2.sort_by((|a, b| a.start.cmp(&b.start)));
        assert_eq!(check2.len(), 3);
        assert_eq!(check2[0], (10,10).into(), "unmapped piece");
        assert_eq!(check2[1], (105,5).into(), "mapped piece - first map");
        assert_eq!(check2[2], (200,5).into(), "mapped piece - second map");

        // (20, 25) - completely within second map
        let check3 = Range::map_ranges(&Range::new(20,5), &target1);
        assert_eq!(check3.len(), 1);
        assert_eq!(check3[0], (200,5).into(), "fully mapped");

        // (15, 35) - overlaps just second
        let mut check4 = Range::map_ranges(&Range::new(15,20), &target1);
        check4.sort_by((|a, b| a.start.cmp(&b.start)));
        assert_eq!(check4.len(), 2);
        assert_eq!(check4[0], (15,5).into(), "unmapped piece");
        assert_eq!(check4[1], (200,15).into(), "mapped piece");
    }

    #[test]
    fn test_filter_ranges() {
        // tuples are (src, dest, width)
        // (0, 10), (20, 30)
        let target1: Vec<RangeMap> = vec![(0u64,100u64,10u64).into(), (20u64,200u64,10u64).into()];

        // (15, 20) - no overlap. note that tuple end is not inclusive
        let check1 = Range::new(15u64,5u64).find_range_maps(&target1);
        assert_eq!(check1.len(), 0);

        // (5, 25) - overlaps both
        let check2 = Range::new(5u64,20u64).find_range_maps(&target1);
        assert_eq!(check2.len(), 2);

        // (20, 25) - completely within
        let check3 = Range::new(20u64,5u64).find_range_maps(&target1);
        assert_eq!(check3.len(), 1);
        assert_eq!(*check3[0], (20, 200, 10).into());

        // (15, 35) - overlaps second
        let check4 = Range::new(15u64,20u64).find_range_maps(&target1);
        assert_eq!(check4.len(), 1);
        assert_eq!(*check4[0], (20, 200, 10).into());

        let check5 = Range::new(0u64,15u64).find_range_maps(&target1);
        assert_eq!(check5.len(), 1);
        assert_eq!(*check5[0], (0, 100, 10).into());
    }
}