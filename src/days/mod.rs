pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;

pub struct Day {
    pub part1: fn(bool, &str) -> Result<String, String>,
    pub part2: fn(bool, &str) -> Result<String, String>,
}

pub const DAYS: &[Day] = &[
    Day { part1: day01::part1, part2: day01::part2 }, // day 1
    Day { part1: day02::part1, part2: day02::part2 }, // day 2
    Day { part1: day03::part1, part2: day03::part2 }, // day 3
    Day { part1: day04::part1, part2: day04::part2 }, // day 4
];