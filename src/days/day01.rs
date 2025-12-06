
// Part 1
// A dial from 0 to 99, starting at 50 (0 wraps to 99 and 99 to 0)
// Instructions to turn dial: L<n> or R<n>.
// The actual password is the number of times the dial is left pointing at 0 
// after any rotation in the sequence
pub fn part1(debug: bool, input: &str) -> Result<String, String> {
    #[derive(Debug)]
    enum Direction {
        Left,
        Right,
    }
    #[derive(Debug)]
    struct Instruction {
        direction: Direction,
        amount: usize,
    }

    // Parse the input
    let mut instructions: Vec<Instruction> = Vec::new();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if debug { println!("Parsing {line}"); }

        let (direction, amount) = line.split_at(1);
        let direction = match direction {
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => return Err(format!("Invalid instruction! {line}"))
        };
        let amount = match amount.parse::<usize>() {
            Ok(n) => n,
            Err(_) => return Err(format!("Invalid instruction! {line}"))
        };
        instructions.push(Instruction { direction, amount });
    }
    if debug { println!("Final instruction list: {:?}", instructions) }
    
    // Process the instructions
    let mut position: u8 = 50;
    let mut num_zeros: usize = 0;
    for instruction in instructions {
        if debug { println!("Rotating {:?} by {}", instruction.direction, instruction.amount); }
        for _ in 0..instruction.amount {
            match instruction.direction {
                Direction::Left => {
                    position = match position.checked_sub(1) {
                        Some(n) => n,
                        None => 99,
                    };
                },
                Direction::Right => {
                    position = if position + 1 > 99 {
                        0
                    } else {
                        position + 1
                    };
                },
            }
        }
        if debug { println!("Ending on {}", position); }
        if position == 0 {
            num_zeros += 1;
            if debug { println!("Thats {} zeros", num_zeros);}
        }
    }

    Ok(num_zeros.to_string())
}

// Part 2 solution
pub fn part2(debug: bool, input: &str) -> Result<String, String> {
    #[derive(Debug)]
    enum Direction {
        Left,
        Right,
    }
    #[derive(Debug)]
    struct Instruction {
        direction: Direction,
        amount: usize,
    }

    // Parse the input
    let mut instructions: Vec<Instruction> = Vec::new();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if debug { println!("Parsing {line}"); }

        let (direction, amount) = line.split_at(1);
        let direction = match direction {
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => return Err(format!("Invalid instruction! {line}"))
        };
        let amount = match amount.parse::<usize>() {
            Ok(n) => n,
            Err(_) => return Err(format!("Invalid instruction! {line}"))
        };
        instructions.push(Instruction { direction, amount });
    }
    if debug { println!("Final instruction list: {:?}", instructions) }
    
    // Process the instructions
    let mut position: u8 = 50;
    let mut num_zeros: usize = 0;
    for instruction in instructions {
        if debug { println!("Rotating {:?} by {}", instruction.direction, instruction.amount); }
        for _ in 0..instruction.amount {
            match instruction.direction {
                Direction::Left => {
                    position = match position.checked_sub(1) {
                        Some(n) => n,
                        None => 99,
                    };
                },
                Direction::Right => {
                    position = if position + 1 > 99 {
                        0
                    } else {
                        position + 1
                    };
                },
            }
            if position == 0 {
                num_zeros += 1;
                if debug { println!("Thats {} zeros", num_zeros);}
            }
        }
        if debug { println!("Ending on {}", position); }
    }

    Ok(num_zeros.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = include_str!("../../example_inputs/day01-1.txt");
    const EXAMPLE2: &str = include_str!("../../example_inputs/day01-1.txt");

    #[test]
    fn example_part1() {
        assert_eq!(part1(false, EXAMPLE1), Ok("3".to_string()));
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(false, EXAMPLE2), Ok("6".to_string()));
    }
}
