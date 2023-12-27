use std::collections::HashMap;
use std::iter::Map;
use std::str::FromStr;
use crate::day7::HandType::{FiveOfAKind, FourOfAKind, FullHouse, HighHard, OnePair, ThreeOfAKind, TwoPair};
use crate::day7::Kind::*;

#[derive(Clone,Copy,Debug,Eq,Hash,PartialEq,PartialOrd)]
#[repr(u8)]
enum Kind {
    S2 = b'2',
    S3 = b'3',
    S4 = b'4',
    S5 = b'5',
    S6 = b'6',
    S7 = b'7',
    S8 = b'8',
    S9 = b'9',
    ST = b'T',
    SJ = b'J',
    SQ = b'Q',
    SK = b'K',
    SA = b'A'
}

impl Kind {
    fn from(c: char) -> Result<Self, ()> {
        match c {
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
            _ => Err(())
        }
    }
}

#[derive(Debug,PartialEq)]
enum HandType {
    HighHard, // 5 kinds
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

    fn new(cards: Vec<Kind>) -> Hand {
        let suits = cards.iter().fold(HashMap::new(), |mut acc, k| {
            let counter = acc.entry(*k).or_insert(0);
            *counter += 1;
            acc
        });
        let mut ret_cards: [Kind; 5] = [cards[0],cards[1],cards[2],cards[3],cards[4]]; // TODO: idiomatic way?
        let ret = Hand{cards: ret_cards, suits};
        ret
    }

    fn get_type(&self) -> HandType {
        match self.suits.len() {
            5 => HighHard,
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

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 5 {
            Err(())
        } else {
            Ok(Hand::new(s.chars().map(|c| Kind::from(c).unwrap()).collect()))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use crate::day7::Hand;
    use crate::day7::HandType::{OnePair, ThreeOfAKind, TwoPair};

    #[test]
    fn test_creating() {
        let test_vals = vec![
            ("32T3K 765", OnePair),
            ("T55J5 684", ThreeOfAKind),
            ("KK677 28", TwoPair),
            ("KTJJT 220", TwoPair),
            ("QQQJA 483", ThreeOfAKind)
        ];

        for (cards, hand_type) in test_vals {
            let s: Vec<&str> = cards.split_whitespace().collect();
            let hand = Hand::from_str(s[0]).unwrap();
            assert_eq!(hand_type, hand.get_type());
        }
    }
}