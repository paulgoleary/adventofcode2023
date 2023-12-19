use std::cmp::min;
use std::collections::HashSet;
use nom::bytes::complete::tag;
use nom::character::complete::{multispace1, space1};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use crate::common;
use crate::common::number;

fn card_pre_parser(input: &str) -> IResult<&str, u32> {
    tuple((tag("Card"), space1, crate::common::number, tag(":")))(input).map(|(remaining, res)| {
        (remaining, res.2)
    })
}

fn process_line_day4(line: &str) -> usize {
    let mut parser = tuple((
        card_pre_parser,
        multispace1,
        separated_list1(multispace1, number),
        multispace1,
        tag("|"),
        multispace1,
        separated_list1(multispace1, number))
    );
    let (_, wining_nums, card_nums) = parser(line)
        .map(|res| (res.1.0, res.1.2, res.1.6)).unwrap_or_default();
    let winning_set: HashSet<_> = wining_nums.iter().collect();
    let card_set: HashSet<_> = card_nums.iter().collect();
    let sect: HashSet<_> = winning_set.intersection(&card_set).collect();
    sect.len()
}

pub fn do_day4() {
    if let Ok(lines) = common::read_lines("./data/day4input.txt") {
        let card_matches: Vec<usize> = lines.map(|l| process_line_day4(&l.unwrap())).collect();

        let res_part1 = card_matches.iter().fold(0, |acc, x| acc + if *x == 0 {0} else {2u32.pow((x-1) as u32)});
        println!("final sum, part1: {}", res_part1);

        let res_part2 = process_part2(card_matches);
        println!("final count, part2: {}", res_part2);
    }
}

fn process_part2(results: Vec<usize>) -> i32 {
    let mut card_counts = vec![1 ;results.len()];
    for idx in 0..results.len()-1 {
        let end = min(results.len(), idx+1+results[idx]);
        for x in idx+1..end {
            card_counts[x] = card_counts[x] + card_counts[idx];
        }
    }
    card_counts.iter().fold(0, |acc, x| acc + x)
}

#[cfg(test)]
mod tests {
    use crate::day4::{process_line_day4, process_part2};

    #[test]
    fn test_example() {
        let ex = vec![
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
        "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
        "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
        "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
        "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
        "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"];

        let mut total = 0;
        let mut results: Vec<usize> = Vec::new();
        for line in ex.iter() {
            let res = process_line_day4(line);
            total += if res == 0 {0} else {2u32.pow((res-1) as u32)};
            results.push(res)
        };
        assert_eq!(13, total);

        let total_cards = process_part2(results);
        assert_eq!(30, total_cards);
    }
}
