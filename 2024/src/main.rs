use std::time::Instant;

mod day1;
mod day2;
mod day3;

fn main() {
    let input = include_str!("../inputs/day3");
    let iterations = 10000;
    let now = Instant::now();
    for _ in 0..=iterations {
        day3::part1::parse(input);
    }
    let part1_elapsed = now.elapsed().as_micros() / iterations;
    let now = Instant::now();
    for _ in 0..=iterations {
        day3::part2::parse(input);
    }
    let part2_elapsed = now.elapsed().as_micros() / iterations;
    println!(
        "Part 1: {part1_elapsed}µs (ran {iterations} times)\nPart 2: {part2_elapsed}µs (ran {iterations} times)"
    );
}
