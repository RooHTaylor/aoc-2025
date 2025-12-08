// Part 1
pub fn part1(debug: bool, input: &str) -> Result<String, String> {
    
    let problems = parse_input(input);
    if debug { println!("{:?}", problems); }

    let mut sum: usize = 0;
    for problem in problems {
        sum += match problem.operand {
            Operand::ADD => {
                problem.group.iter().sum::<usize>()
            },
            Operand::MUL => {
                problem.group.iter().product::<usize>()
            }
        };
    }

    Ok(sum.to_string())
}

#[derive(Debug, Clone)]
enum Operand {
    ADD,
    MUL,
}

#[derive(Debug, Clone)]
struct Problem {
    group: Vec<usize>,
    operand: Operand,
}

fn parse_input(input: &str) -> Vec<Problem> {

    let mut lines: Vec<Vec<&str>> = vec![];
    for line in input.lines() {
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        lines.push(parts);
    }

    let mut problems: Vec<Problem> = vec![];

    for gi in 0..lines[0].len() {
        let mut group: Vec<usize> = vec![];
        let mut operand: Operand = Operand::ADD;
        for i in 0..lines.len() {
            if i == lines.len()-1 {
                // last element is sign
                operand = match lines[i][gi] {
                    "*" => Operand::MUL,
                    _ => {
                        continue;
                    }
                };
                continue;
            }
            let num: usize = lines[i][gi].parse().unwrap();
            group.push(num);
        }
        problems.push(Problem{group, operand})
    }

    problems
}

fn parse_input_part2(input: &str) -> Vec<Problem> {

    // Parse the chart by char, so we preserve the whitespace positions
    let mut lines: Vec<Vec<char>> = vec![];
    for line in input.lines() {
        let parts: Vec<char> = line.chars().collect();
        lines.push(parts);
    }

    // Rotate the chart to the right to make it easier to parse
    let mut rotated_lines: Vec<Vec<char>> = vec![];
    for col in 0..lines[0].len() {
        let mut column: Vec<char> = vec![];
        for line in (0..lines.len()).rev() {
            column.push(lines[line][col]);
        }
        rotated_lines.push(column)
    }

    let mut problems: Vec<Problem> = vec![];

    let mut group: Vec<usize> = vec![];
    let mut operand: Option<Operand> = None;
    for l in 0..rotated_lines.len() {
        // If the first char of the line is a * or + that means we're starting a
        // new problem. If we have a current group stored, store it with the operand
        // 
        if rotated_lines[l][0] == '*' || rotated_lines[l][0] == '+' {
            if !group.is_empty() {
                problems.push(Problem{
                    group: group.clone(),
                    operand: operand.unwrap().clone()
                });
                group.clear();
            }
            operand = match rotated_lines[l][0] {
                '*' => Some(Operand::MUL),
                '+' => Some(Operand::ADD),
                _ => None,
            };
        }

        let mut buffer: Vec<char> = vec![];
        for c in 0..rotated_lines[l].len() {
            if rotated_lines[l][c].is_digit(10) {
                buffer.push(rotated_lines[l][c]);
            }
        }
        if buffer.is_empty() {
            // Skip if we didn't find a single number
            continue;
        }
        // Since we rotated the chart, top to bottom becomes right to left, and
        // we are parsing the lines left to right, so we have to reverse the numbers.
        buffer.reverse();
        // We can use unwrap here, because we guarantee that the buffer only contains
        // base-10 digits
        let num: usize = buffer.iter().collect::<String>().parse().unwrap();
        group.push(num);
    }

    // Push the final problem into the Vec
    if !group.is_empty() {
        problems.push(Problem{
            group: group.clone(),
            operand: operand.unwrap().clone()
        });
    }

    problems
}

// Part 2
pub fn part2(debug: bool, input: &str) -> Result<String, String> {
    let problems = parse_input_part2(input);
    if debug { println!("{:?}", problems); }

    let mut sum: usize = 0;
    for problem in problems {
        sum += match problem.operand {
            Operand::ADD => {
                problem.group.iter().sum::<usize>()
            },
            Operand::MUL => {
                problem.group.iter().product::<usize>()
            }
        };
    }

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = include_str!("../../example_inputs/day06-1.txt");
    const EXAMPLE2: &str = include_str!("../../example_inputs/day06-1.txt");

    #[test]
    fn example_part1() {
        assert_eq!(part1(false, EXAMPLE1), Ok("4277556".to_string()));
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(false, EXAMPLE2), Ok("3263827".to_string()));
    }
}