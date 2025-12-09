struct JunctionBox {
    x: usize,
    y: usize,
    z: usize,
    circuit: Option<usize>,
}

// Part 1
pub fn part1(_debug: bool, _input: &str) -> Result<String, String> {

    Ok("".to_string())
}

pub fn part2(_debug: bool, _input: &str) -> Result<String, String> {

    Ok("".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = include_str!("../../example_inputs/day08-1.txt");
    const EXAMPLE2: &str = include_str!("../../example_inputs/day08-1.txt");

    #[test]
    fn example_part1() {
        assert_eq!(part1(false, EXAMPLE1), Ok("".to_string()));
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(false, EXAMPLE2), Ok("".to_string()));
    }
}