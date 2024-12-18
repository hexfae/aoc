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

pub mod part2 {
    pub fn parse(input: &str) -> usize {
        input
            .lines()
            .map(|line| {
                line.split(' ')
                    .filter_map(|str| str.parse::<u8>().ok())
                    .collect::<Vec<u8>>()
            })
            .map(|mut vec| {
                if vec.first().unwrap_or(&0) > vec.last().unwrap_or(&0) {
                    vec.reverse();
                }
                vec
            })
            .filter(|numbers| {
                let safe_without_dampening = numbers
                    .windows(2)
                    // wrapping_sub will overflow to ~255 meaning the
                    // second check will fail, which is what we want
                    .all(|pair| pair[0] != pair[1] && pair[1].wrapping_sub(pair[0]) <= 3);
                if safe_without_dampening {
                    return true;
                }
                for i in 0..numbers.len() {
                    let mut numbers = numbers.clone();
                    numbers.remove(i);
                    if numbers
                        .windows(2)
                        // wrapping_sub will overflow to ~255 meaning the
                        // second check will fail, which is what we want
                        .all(|pair| pair[0] != pair[1] && pair[1].wrapping_sub(pair[0]) <= 3)
                    {
                        return true;
                    }
                }
                false
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
        1 3 6 7 9";
        assert_eq!(parse(input), 4);
    }

    #[test]
    fn actual() {
        let input = include_str!("../inputs/day2");
        assert_eq!(parse(input), 604);
    }
}
