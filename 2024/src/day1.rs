pub mod part1 {
    pub fn parse(input: &str) -> u32 {
        let (left, right): (Vec<&str>, Vec<&str>) = input
            .lines()
            .filter_map(|line| line.split_once("   "))
            .unzip();
        let mut left: Vec<u32> = left.iter().filter_map(|str| str.parse().ok()).collect();
        let mut right: Vec<u32> = right.iter().filter_map(|str| str.parse().ok()).collect();
        left.sort_unstable();
        right.sort_unstable();
        let total_distance: u32 = left
            .into_iter()
            .zip(right)
            .map(|(left, right)| {
                if left > right {
                    left - right
                } else {
                    right - left
                }
            })
            .sum();
        total_distance
    }

    #[test]
    fn example() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(parse(input), 11);
    }

    #[test]
    fn actual() {
        let input = include_str!("../inputs/day1");
        assert_eq!(parse(input), 2_580_760);
    }
}

pub mod part2 {
    pub fn parse(input: &str) -> u32 {
        let (left, right): (Vec<&str>, Vec<&str>) = input
            .lines()
            .filter_map(|line| line.split_once("   "))
            .unzip();

        let mut right: Vec<u32> = right.iter().filter_map(|str| str.parse().ok()).collect();
        right.sort_unstable();
        left.iter()
            .filter_map(|str| str.parse().ok())
            .map(|number| {
                let mut times_appeared = 0;
                if let Ok(index) = right.binary_search(&number) {
                    times_appeared += 1;
                    let mut left_offset = 1;
                    while index >= left_offset {
                        if let Some(previous) = right.get(index - left_offset) {
                            if previous == &number {
                                left_offset += 1;
                                times_appeared += 1;
                            } else {
                                break;
                            }
                        }
                    }
                    let mut right_offset = 1;
                    while let Some(next) = right.get(index + right_offset) {
                        if next == &number {
                            right_offset += 1;
                            times_appeared += 1;
                        } else {
                            break;
                        }
                    }
                }
                // dbg!(number, times_appeared, number * times_appeared);
                number * times_appeared
            })
            .sum()
    }

    #[test]
    fn example() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(parse(input), 31);
    }

    #[test]
    fn actual() {
        let input = include_str!("../inputs/day1");
        assert_eq!(parse(input), 25_358_365);
    }
}
