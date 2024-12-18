pub mod part1 {
    use regex::Regex;

    pub fn parse(input: &str) -> u32 {
        let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("invalid regex");
        regex
            .captures_iter(input)
            .map(|capture| capture.extract())
            .filter_map(|(_, [first, second])| {
                Some((first.parse::<u32>().ok()?, second.parse::<u32>().ok()?))
            })
            .map(|(first, second)| first * second)
            .sum::<u32>()
    }

    #[test]
    fn example() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(parse(input), 161);
    }

    #[test]
    fn actual() {
        let input = include_str!("../inputs/day3");
        assert_eq!(parse(input), 164_730_528);
    }
}

pub mod part2 {
    use regex::Regex;

    enum Instruction {
        Mul(u32, u32),
        Do,
        DoNot,
    }

    pub fn parse(input: &str) -> u32 {
        let regex =
            Regex::new(r"mul\((\d{1,3},\d{1,3})\)|(do\(\))|(don't\(\))").expect("invalid regex");
        let mut mul_instructions_enabled = true;
        regex
            .captures_iter(input)
            .map(|capture| capture.extract::<1>().1[0])
            .filter_map(Instruction::parse_from)
            .map(|instruction| match instruction {
                Instruction::Mul(first, second) => {
                    if mul_instructions_enabled {
                        first * second
                    } else {
                        0
                    }
                }
                Instruction::Do => {
                    mul_instructions_enabled = true;
                    0
                }
                Instruction::DoNot => {
                    mul_instructions_enabled = false;
                    0
                }
            })
            .sum()
    }

    #[test]
    fn example() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(parse(input), 48);
    }

    #[test]
    fn actual() {
        let input = include_str!("../inputs/day3");
        assert_eq!(parse(input), 70_478_672);
    }

    impl Instruction {
        fn parse_from(input: &str) -> Option<Self> {
            match input {
                "do()" => Some(Self::Do),
                "don't()" => Some(Self::DoNot),
                other => {
                    let (left, right) = other.split_once(',')?;
                    Some(Self::Mul(left.parse().ok()?, right.parse().ok()?))
                }
            }
        }
    }
}
