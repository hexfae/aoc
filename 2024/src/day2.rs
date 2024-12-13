pub mod part1 {
    pub fn parse(input: &str) -> usize {
        input
            .lines()
            .map(|line| {
                line.split(' ')
                    .filter_map(|str| str.parse::<u8>().ok())
                    .collect::<Vec<u8>>()
            })
            .filter_map(|mut vec| {
                if vec.is_sorted() {
                    return Some(vec);
                }
                vec.reverse();
                if vec.is_sorted() {
                    return Some(vec);
                }
                None
            })
            .filter(|numbers| {
                numbers
                    .windows(2)
                    .all(|pair| pair[0] != pair[1] && pair[1] - pair[0] <= 3)
            })
            .count()
    }

    #[test]
    fn example() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
        assert_eq!(parse(input), 2);
    }

    #[test]
    fn actual() {
        let input = include_str!("../inputs/day2");
        assert_eq!(parse(input), 564);
    }
}
