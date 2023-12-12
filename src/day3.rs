use std::cmp::{max, min};
use std::collections::HashSet;
use crate::common;

#[derive(Clone, Default)]
struct LineProc {
    nums: Vec<(u32, usize, usize)>,
    symbol_positions: HashSet<usize>
}

pub trait SelectSymbol {
    fn select(&self, c: char) -> bool;
}

struct Section {
    select_func: fn(c: char) -> bool,
    width: usize,
    preceding: LineProc,
    current: LineProc,
    next: LineProc
}

fn is_symbol(c: char) -> bool {
    if c.is_digit(10) || c == '.' {
        return false
    }
    true
}

fn is_gear(c: char) -> bool {
    if c == '*' {
        return true
    }
    false
}

impl SelectSymbol for Section {
    fn select(&self, c: char) -> bool {
        if c.is_digit(10) || c == '.' {
            return false
        }
        true
    }
}

impl Section {

    fn new(sf: fn(c: char) -> bool) -> Self {
        Section{
            select_func: sf,
            width: 0,
            preceding: LineProc::default(),
            current: LineProc::default(),
            next: LineProc::default(),
        }
    }

    fn process_line(&self, line: &str) -> LineProc {
        let mut nums: Vec<(u32, usize, usize)> = Vec::new();
        let mut got_num = false;

        let mut symbol_positions: Vec<usize> = Vec::new();

        for (pos, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                let num: u32 = c.to_string().parse().unwrap();
                if got_num {
                    let idx = nums.len() - 1;
                    nums[idx].0 *= 10;
                    nums[idx].0 += num;
                } else {
                    got_num = true;
                    nums.push((num, pos, 0));
                }
            } else {
                if got_num {
                    let idx = nums.len() - 1;
                    nums[idx].2 = pos - 1;
                    got_num = false;
                }
            }
            // if self.select(c) {
            if (self.select_func)(c) {
                symbol_positions.push(pos);
            }
        }
        if got_num {
            let idx = nums.len() - 1;
            nums[idx].2 = line.len() - 1;
        }

        nums.sort();
        LineProc{
            nums,
            symbol_positions: symbol_positions.iter().map(|x| *x).collect(),
        }
    }

    fn push(&self, line: &str) -> Self {
        Section{
            select_func: self.select_func,
            width: max(self.width, line.len()), // TODO: validate constant?
            preceding: self.current.clone(),
            current: self.next.clone(),
            next: self.process_line(line),
        }
    }

    fn find_adjacent_nums(&self) -> Vec<u32> {
        let ret = self.current.nums.iter().enumerate().filter(|(idx, (num, range_lo, range_hi))|{
            let rl = if *range_lo > 0 { range_lo - 1 } else { *range_lo };
            let rh = if *range_hi < (self.width - 1) { range_hi + 1 } else { *range_hi };
            for rx in rl..=rh {
                for sp in [&self.preceding.symbol_positions, &self.current.symbol_positions, &self.next.symbol_positions] {
                    if sp.contains(&rx) {
                        return true
                    }
                }
            }
            false
        });
        ret.map(|(_, (num, _, _))| *num ).collect()
    }
}

pub fn do_day3() {
    if let Ok(lines) = common::read_lines("./data/day3input.txt") {
        let lines_iter = lines.map(|l| l.unwrap()).into_iter();
        let total = process_lines_day3(lines_iter);
        println!("final sum: {}", total);
    }
}

fn process_lines_day3(lines : impl std::iter::Iterator<Item = String>) -> u32 {
    let mut total = 0;
    let mut sec = Section::new(is_symbol);
    for line in lines {
        sec = sec.push(line.as_str());
        let ret = sec.find_adjacent_nums();
        let sum = ret.iter().fold(0, |acc, num| acc + num);
        total += sum;
    }
    sec = sec.push("");
    let ret = sec.find_adjacent_nums();
    let sum = ret.iter().fold(0, |acc, num| acc + num);
    total += sum;
    total
}

#[cfg(test)]
mod tests {
    use crate::day3::{is_symbol, LineProc, process_lines_day3, Section};

    #[test]
    fn test_edge() {
        let test = vec![
            "....758..........................*......=.............@................................273......911...#....@666...+193......................",
            ".............604....483..&144.859......807...-.........995..-218.770............37.512.*.........*.........................215...........117",
            "......354..........*...............$........849.*.................................*.....242....469.&764.........................959*128.$..."];

        let test_iter = test.iter().map(|s| s.to_string()).into_iter();
        let total = process_lines_day3(test_iter);

        assert_eq!(9626, total);
    }

    #[test]
    fn test_example() {
        let ex = vec![
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598.."];

        let ex_iter = ex.iter().map(|s| s.to_string()).into_iter();
        let total = process_lines_day3(ex_iter);
        assert_eq!(4361, total);
    }

    #[test]
    fn test_day3() {
        let line1 = "....411...............838......721.....44..............................................607..................................................";
        let line2 = "...&......519..................*..........#.97.........994..............404..............*...&43........440...882.......673.505.............";
        let line3 = ".....*......*...892.........971...%....131....*..........*.......515...$.......157.....412.............-.....*.............*............594.";
        let line4 = "..856.495....13...-...............602..........36...$.985....341*.........88.....*.921....................122..................806..508.....";

        let mut sec = Section{
            select_func: is_symbol,
            width: 0,
            preceding: LineProc::default(),
            current: LineProc::default(),
            next: LineProc::default(),
        };

        sec = sec.push(line1);
        sec = sec.push(line2);
        assert_eq!(0, sec.preceding.nums.len());

        let expect: Vec<u32> = vec![411, 721,607];
        let check = sec.find_adjacent_nums();
        assert_eq!(expect, check);

        sec = sec.push(line3);
        assert_ne!(0, sec.preceding.nums.len());

        let expect1: Vec<u32> = vec![519, 97, 994, 404, 43, 440, 882, 673, 505];
        let check1 = sec.find_adjacent_nums();
        assert_eq!(expect1, check1);

        sec = sec.push(line4);

        let expect2: Vec<u32> = vec![892, 971, 131, 515, 157, 412];
        let check2 = sec.find_adjacent_nums();
        assert_eq!(expect2, check2);

        assert_eq!(7, sec.current.nums.len());
        assert_eq!(9, sec.current.symbol_positions.len());

        sec = sec.push("");
        assert_eq!(0, sec.next.nums.len());

    }
}
