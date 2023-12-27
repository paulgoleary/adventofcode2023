use std::cmp::Ordering;
use std::collections::HashMap;
use std::str::FromStr;
use crate::common::AoCError;
use crate::day7::HandType::{FiveOfAKind, FourOfAKind, FullHouse, HighCard, OnePair, ThreeOfAKind, TwoPair};
use crate::day7::Kind::*;

#[derive(Clone,Copy,Debug,Eq,Hash,Ord,PartialEq,PartialOrd)]
enum Kind {
    SX,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    S8,
    S9,
    ST,
    SJ,
    SQ,
    SK,
    SA
}

impl Kind {
    fn from(c: char) -> Result<Self, AoCError> {
        match c {
            'X' => Ok(SX),
            '2' => Ok(S2),
            '3' => Ok(S3),
            '4' => Ok(S4),
            '5' => Ok(S5),
            '6' => Ok(S6),
            '7' => Ok(S7),
            '8' => Ok(S8),
            '9' => Ok(S9),
            'T' => Ok(ST),
            'J' => Ok(SJ),
            'Q' => Ok(SQ),
            'K' => Ok(SK),
            'A' => Ok(SA),
            _ => Err(AoCError::InputValueError(format!("invalid kind token: {}", c)))
        }
    }
}

#[derive(Debug,Eq,Ord,PartialEq,PartialOrd)]
enum HandType {
    HighCard, // 5 kinds
    OnePair, // 4 kinds
    TwoPair, // 3 kinds
    ThreeOfAKind, // 3 kinds
    FullHouse, // 2 kinds
    FourOfAKind, // 2 kinds
    FiveOfAKind // 1 kind
}

struct Hand {
    cards: [Kind; 5],
    suits: HashMap<Kind, i32>
}

impl Hand {

    fn handle_jokers(mut suits: HashMap<Kind, i32>) -> HashMap<Kind, i32> {
        match suits.remove(&SX) {
            Some(5) => {
                suits.insert(SX, 5);
            }
            Some(cnt) => {
                let (max_suit, max_cnt) = suits.iter().max_by(|(_, x), (_, y)| x.cmp(y)).unwrap();
                suits.insert(*max_suit, max_cnt+cnt);
            },
            _ => {}
        }
        suits
    }

    fn new(cards: Vec<Kind>) -> Hand {
        let suits = cards.iter().fold(HashMap::new(), |mut acc, k| {
            let counter = acc.entry(*k).or_insert(0);
            *counter += 1;
            acc
        });
        let mut ret_cards: [Kind; 5] = [cards[0],cards[1],cards[2],cards[3],cards[4]]; // TODO: idiomatic way?
        let ret = Hand{cards: ret_cards, suits: Hand::handle_jokers(suits)};
        ret
    }

    fn get_type(&self) -> HandType {
        match self.suits.len() {
            5 => HighCard,
            4 => OnePair,
            3 => {
                if self.suits.iter().any(|(_, v)|*v == 3) {ThreeOfAKind} else {TwoPair}
            },
            2 => {
                if self.suits.iter().any(|(_, v)|*v == 4) {FourOfAKind} else {FullHouse}
            },
            1 => FiveOfAKind,
            _ => panic!("cannot happen")
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.get_type().cmp(&other.get_type()) {
            Ordering::Equal => {self.cards.cmp(&other.cards)}
            ord => ord
        }
    }
}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.get_type() == other.get_type() && self.cards == other.cards
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other)) // TODO: can this be derived?
    }
}

impl Eq for Hand { }

impl FromStr for Hand {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 5 {
            Err(AoCError::InputFormatError("invalid length for hand string - expected exactly 5"))
        } else {
            let hand: Result<Vec<Kind>, _> = s.chars().map(|c| Kind::from(c)).collect();
            Ok(Hand::new(hand?))
        }
    }
}

struct HandBid {
    hand: Hand,
    bid: u64
}
impl FromStr for HandBid {
    type Err = AoCError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let s: Vec<_> = input.split_whitespace().collect();
        match s[..] {
            [hand_str, bid_str] => Ok(
                HandBid {
                    hand: Hand::from_str(hand_str)?,
                    bid: u64::from_str(bid_str)?
                }
            ),
            _ => Err(AoCError::InputValueError(format!("invalid hand bid value: {}", input.to_string())))
        }
    }
}

fn process_day7_input(lines : impl Iterator<Item = String>) -> Result<Vec<HandBid>, AoCError> {
    lines.map(|hand_str| {
        HandBid::from_str(hand_str.as_str())
    }).collect::<Result<Vec<_>, _>>()
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use std::str::FromStr;
    use crate::common;
    use crate::day7::{Hand, HandBid, process_day7_input};
    use crate::day7::HandType::{FourOfAKind, OnePair, ThreeOfAKind, TwoPair};

    fn get_day7_result(mut res: Vec<HandBid>) -> u64 {
        res.sort_by(|v1, v2| v1.hand.cmp(&v2.hand));
        res.iter().enumerate().fold(0, |acc, (idx, hb)| {
            acc + (idx as u64+1) * hb.bid
        })
    }

    #[test]
    fn test_day7_part1() {
        if let Ok(lines) = common::read_lines("./data/day7input.txt") {
            let lines_iter = lines.map(|l| l.unwrap()).into_iter();
            let total_winnings = match process_day7_input(lines_iter) {
                Ok(res) => get_day7_result(res),
                Err(_) => 0
            };
            assert_eq!(total_winnings, 246424613);
        }
    }

    #[test]
    fn test_day7_part2() {
        if let Ok(lines) = common::read_lines("./data/day7input.txt") {
            let lines_iter = lines.map(|l| l.unwrap().replace("J", "X")).into_iter();
            let total_winnings = match process_day7_input(lines_iter) {
                Ok(res) => get_day7_result(res),
                Err(_) => 0
            };
            assert_eq!(total_winnings, 248256639);
        }
    }

    #[test]
    fn test_example() {
        let test_vals = vec![
            ("32T3K 765", OnePair),
            ("T55J5 684", ThreeOfAKind),
            ("KK677 28", TwoPair),
            ("KTJJT 220", TwoPair),
            ("QQQJA 483", ThreeOfAKind)
        ];

        let res = process_day7_input(test_vals.iter().map(|v|v.0.to_string()));
        let mut ordered_hands = res.unwrap();
        ordered_hands.sort_by(|v1, v2| v1.hand.cmp(&v2.hand));

        assert_eq!(ordered_hands[0].hand.get_type(), OnePair);
        assert_eq!(ordered_hands[4].hand.get_type(), ThreeOfAKind);

        let total_winnings = ordered_hands.iter().enumerate().fold(0, |acc, (idx, hb)| {
            acc + (idx as u64+1) * hb.bid
        });
        assert_eq!(total_winnings, 6440);
    }

    #[test]
    fn test_example_part2() {
        let test_vals = vec![
            ("32T3K 765", OnePair),
            ("T55J5 684", FourOfAKind),
            ("KK677 28", TwoPair),
            ("KTJJT 220", FourOfAKind),
            ("QQQJA 483", FourOfAKind)
        ];

        let res = process_day7_input(test_vals.iter().map(|v| v.0.replace("J", "X")));
        let mut ordered_hands = res.unwrap();
        ordered_hands.sort_by(|v1, v2| v1.hand.cmp(&v2.hand));

        assert_eq!(ordered_hands[0].bid, 765);
        assert_eq!(ordered_hands[4].bid, 220);

        let total_winnings = ordered_hands.iter().enumerate().fold(0, |acc, (idx, hb)| {
            acc + (idx as u64+1) * hb.bid
        });
        assert_eq!(total_winnings, 5905);
    }

    #[test]
    fn test_hand_ordering() {
        let h1 = Hand::from_str("KK677").unwrap();
        let h2 = Hand::from_str("KTJJT").unwrap();
        assert_eq!(h1.cmp(&h2), Ordering::Greater);
    }

    #[test]
    fn test_collect_result() {
        let strings = vec!["7", "42", "one"];
        let numbers: Result<Vec<_>, _> = strings
            .into_iter()
            .map(|s| s.parse::<i32>())
            .collect();
        assert!(numbers.is_err())
    }
}