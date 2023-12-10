use std::cmp::{max, min};
use std::collections::HashSet;

#[derive(Clone)]
#[derive(Default)]
struct LineProc {
    nums: Vec<u32>,
    num_ranges: Vec<(usize, usize)>,
    symbol_positions: HashSet<usize>
}

struct Section {
    width: usize,
    preceding: LineProc,
    current: LineProc,
    next: LineProc
}

impl Section {

    fn process_line(&self, line: &str) -> LineProc {
        let mut nums: Vec<u32> = Vec::new();
        let mut num_ranges: Vec<(usize, usize)> = Vec::new();
        let mut got_num = false;

        let mut symbol_positions: Vec<usize> = Vec::new();

        for (pos, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                let num: u32 = c.to_string().parse().unwrap();
                if got_num {
                    let idx = nums.len() - 1;
                    nums[idx] *= 10;
                    nums[idx] += num;
                } else {
                    got_num = true;
                    nums.push(num);
                    num_ranges.push((pos, 0));
                }
            } else {
                if got_num {
                    let idx = nums.len() - 1;
                    num_ranges[idx].1 = pos - 1;
                    got_num = false;
                }
            }
            if is_symbol(c) {
                symbol_positions.push(pos);
            }
        }

        LineProc{
            nums,
            num_ranges,
            symbol_positions: symbol_positions.iter().map(|x| *x).collect(),
        }
    }

    fn push(&self, line: &str) -> Section {
        Section{
            width: max(self.width, line.len()), // TODO: validate constant?
            preceding: self.current.clone(),
            current: self.next.clone(),
            next: self.process_line(line),
        }
    }

    fn find_adjacent_nums(&self) -> Vec<u32> {
        let zip = self.current.nums.iter().zip(self.current.num_ranges.iter());
        let ret = zip.enumerate().filter(|(idx, (num, (range_lo, range_hi)))|{
            let rl = if range_lo > &0 { range_lo - 1 } else { *range_lo };
            let rh = if range_hi < &(self.width - 1) { range_hi + 1 } else { *range_hi };
            for rx in rl..=rh {
                for sp in [&self.preceding.symbol_positions, &self.current.symbol_positions, &self.next.symbol_positions] {
                    if sp.contains(&rx) {
                        return true
                    }
                }
            }
            false
        });
        ret.map(|xx|*xx.1.0).collect()
    }
}

fn is_symbol(c: char) -> bool {
    if c.is_digit(10) || c == '.' {
        return false
    }
    true
}

#[cfg(test)]
mod tests {
    use nom::sequence::preceded;
    use crate::day3::{is_symbol, LineProc, Section};

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

        let mut sec = Section{
            width: 0,
            preceding: LineProc::default(),
            current: LineProc::default(),
            next: LineProc::default(),
        };

        let mut total = 0;
        for l in ex {
            sec = sec.push(l);
            let ret = sec.find_adjacent_nums();
            let sum = ret.iter().fold(0, |acc, num| acc + num);
            total += sum;
            println!("{:?}", ret)
        }
        sec = sec.push("");
        let ret = sec.find_adjacent_nums();
        let sum = ret.iter().fold(0, |acc, num| acc + num);
        total += sum;
        println!("{:?}", ret);

        assert_eq!(4361, total);
    }

    #[test]
    fn test_day3() {
        let line1 = "....411...............838......721.....44..............................................607..................................................";
        let line2 = "...&......519..................*..........#.97.........994..............404..............*...&43........440...882.......673.505.............";
        let line3 = ".....*......*...892.........971...%....131....*..........*.......515...$.......157.....412.............-.....*.............*............594.";
        let line4 = "..856.495....13...-...............602..........36...$.985....341*.........88.....*.921....................122..................806..508.....";

        let mut sec = Section{
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
