#[cfg(test)]
mod tests {
    #[test]
    fn test_part1_example() {
        // let races = vec![(7,9), (15,40), (30,200)];
        // let races = vec![(59,597), (79,1234), (65,1032), (75,1328)];
        let races = vec![(59796575u64,597123410321328u64)];
        let mut prod = 1;
        for (race_t, race_d) in races {
            let cnt = (0..=race_t).filter(|idx| (race_t - idx) * idx > race_d).count();
            prod *= cnt;
        }
        // assert_eq!(288, prod);
        assert_eq!(34454850, prod);
    }
}