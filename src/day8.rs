

#[cfg(test)]
mod tests {
    use core::num;
    use std::collections::HashMap;
    use std::iter::Take;
    use std::str::FromStr;
    use nom::bytes::complete::{tag, take};
    use nom::character::complete::space1;
    use nom::error::Error;
    use nom::IResult;
    use nom::sequence::tuple;
    use num_bigint::ToBigUint;
    use num_integer::Integer;
    use crate::common;
    use crate::common::{AoCError, number};

    fn node_parser(input: &str) -> IResult<&str, (&str, &str, &str)> {
        match tuple((
            take::<_, _, Error<_>>(3usize),
            tag(" = ("),
            take(3usize),
            tag(", "),
            take(3usize),
            tag(")")))(input) {
            Ok((rem, res)) => Ok((rem, (res.0, res.2, res.4))),
            Err(e) => Err(e)
        }
    }

    #[derive(Default)]
    struct Node {
        key: [u8; 3],
        left: [u8; 3],
        right: [u8; 3]
    }

    fn to_utf3(s: &str) -> [u8; 3] {
        let mut ret: [u8; 3] = [0, 0, 0];
        s.bytes()
            .zip(ret.iter_mut())
            .for_each(|(c, ptr)| *ptr = c);
        ret
    }

    impl Node {
        fn new(key: &str, left: &str, right: &str) -> Node {
            Node{key: to_utf3(key), left: to_utf3(left), right: to_utf3(right)}
        }
    }

    impl FromStr for Node {
        type Err = AoCError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match node_parser(s) {
                Ok((_, (key, left, right))) => Ok(Node::new(key,left,right)),
                Err(e) => Err(AoCError::InputValueError(format!("invalid node format: {}", s)))
            }
        }
    }

    fn process_day8_node_input(lines : impl Iterator<Item = String>) -> Result<Vec<Node>, AoCError> {
        lines.map(|node_str| {
            Node::from_str(node_str.as_str())
        }).collect::<Result<_, _>>()
    }

    #[test]
    fn test_parse_line() {
        let line = "AAA = (BBB, CCC)";
        let (_, (key, left, right)) = node_parser(line).unwrap_or_default();
        assert_eq!(key, "AAA");
        assert_eq!(left, "BBB");
        assert_eq!(right, "CCC");

        let n = Node::from_str(line).unwrap_or_default();
        assert_eq!(n.key, [b'A', b'A', b'A']);
        assert_eq!(n.left, [b'B', b'B', b'B']);
        assert_eq!(n.right, [b'C', b'C', b'C']);
    }

    #[test]
    fn test_day8_part1_example() {
        if let Ok(lines) = common::read_lines("./data/day8example.txt") {
            let mut lines_iter = lines.map(|l| l.unwrap()).into_iter();
            let inst = lines_iter.next();
            assert_eq!(inst, Some("RL".to_string()));
            lines_iter.next();
            let ret = process_day8_node_input(lines_iter).unwrap_or_default();
            assert_eq!(ret.len(), 7);

            let ret_map: HashMap<_, _> = ret.iter().map(|n| (&n.key, n)).collect();
            assert_eq!(ret_map.len(), 7);

            // go RL
            let kr = ret_map.get(&to_utf3("AAA")).unwrap().right;
            let kl = ret_map.get(&kr).unwrap().left;
            assert_eq!(kl, to_utf3("ZZZ"));
        }
    }

    struct NodeStream<'a> {
        m: &'a HashMap<[u8;3], &'a Node>,
        next_keys: Vec<[u8;3]>,
        ix_bytes: Vec<u8>,
        idx: usize
    }

    impl NodeStream<'_> {
        fn new<'a>(node_map: &'a HashMap<[u8;3], &'a Node>, start_keys: Vec<[u8;3]>, ix_str: String) -> NodeStream<'a> {
            NodeStream{m: node_map, next_keys: start_keys, ix_bytes: ix_str.bytes().collect(), idx: 0}
        }
    }
    impl<'a> Iterator for NodeStream<'a> {
        type Item = Vec<[u8;3]>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.next_keys.contains(&[0, 0, 0]) {
                return None
            }

            let next_keys = self.next_keys.iter().map(|k| {
                match self.m.get(k) {
                    Some(n) => {
                        match self.ix_bytes[self.idx] {
                            b'L' => Some(n.left),
                            b'R' => Some(n.right),
                            _ => panic!("should not happen")
                        }
                    }
                    None => None
            }}).collect::<Vec<_>>();

            self.idx += 1;
            if self.idx == self.ix_bytes.len() {
                self.idx = 0;
            }
            let ret_key = self.next_keys.clone();
            self.next_keys = next_keys.iter().map(|k| k.unwrap_or_default()).collect();
            Some(ret_key)
        }
    }

    #[test]
    fn test_day8_part1() {
        if let Ok(lines) = common::read_lines("./data/day8input.txt") {
            let mut lines_iter = lines.map(|l| l.unwrap()).into_iter();
            let inst = lines_iter.next().unwrap();
            lines_iter.next();

            let ret = process_day8_node_input(lines_iter).unwrap_or_default();
            assert_eq!(ret.len(), 786);

            let ret_map: HashMap<_, _> = ret.iter().map(|n| (n.key, n)).collect();
            assert_eq!(ret_map.len(), 786);

            let ns = NodeStream::new(&ret_map, vec![to_utf3("AAA")], inst.clone());
            let ns_cnt = ns.take_while(|n| n[0] != [b'Z', b'Z', b'Z']).count();
            assert_eq!(ns_cnt, 19631);

            let mut key = to_utf3("AAA");
            let stop_key = to_utf3("ZZZ");
            let mut cnt = 0;
            for ix in inst.chars().cycle() {
                let n = ret_map[&key];
                cnt += 1;

                key = match ix {
                    'L' => n.left,
                    'R' => n.right,
                    _ => panic!("should not happen")
                };
                if key == stop_key {
                    break;
                }
            }
            assert_eq!(cnt, 19631);
        }
    }

    #[test]
    fn test_day8_part2() {
        if let Ok(lines) = common::read_lines("./data/day8input.txt") {
            let mut lines_iter = lines.map(|l| l.unwrap()).into_iter();
            let inst = lines_iter.next().unwrap();
            lines_iter.next();

            let ret = process_day8_node_input(lines_iter).unwrap_or_default();
            assert_eq!(ret.len(), 786);

            let ret_map: HashMap<_, _> = ret.iter().map(|n| (n.key, n)).collect();
            assert_eq!(ret_map.len(), 786);

            let part2_start_keys: Vec<[u8; 3]> = ret_map.keys().filter(|k| k[2] == b'A').map(|k| *k).collect();

            let key_counts: Vec<_> = part2_start_keys.iter().map(|k| {
                let ns = NodeStream::new(&ret_map, vec![*k], inst.clone());
                ns.take_while(|n| {
                    !n.iter().all(|k| k[2] == b'Z')
                }).count()
            }).collect();

            let keys_lcm = key_counts.iter().fold(1.to_biguint().unwrap(), |acc, kc| {
                acc.lcm(&kc.to_biguint().unwrap())
            });
            assert_eq!(keys_lcm, 21003205388413u64.to_biguint().unwrap())
        }
    }
}