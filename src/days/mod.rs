pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;

pub struct Day {
    pub part1: fn(bool, &str) -> Result<String, String>,
    pub part2: fn(bool, &str) -> Result<String, String>,
}

pub const DAYS: &[Day] = &[
    Day { part1: day01::part1, part2: day01::part2 }, // day 1
    Day { part1: day02::part1, part2: day02::part2 }, // day 2
    Day { part1: day03::part1, part2: day03::part2 }, // day 3
    Day { part1: day04::part1, part2: day04::part2 }, // day 4
    Day { part1: day05::part1, part2: day05::part2 }, // day 5
    Day { part1: day06::part1, part2: day06::part2 }, // day 6
    Day { part1: day07::part1, part2: day07::part2 }, // day 7
    Day { part1: day08::part1, part2: day08::part2 }, // day 8
    Day { part1: day09::part1, part2: day09::part2 }, // day 9
];