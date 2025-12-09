// Part 1
pub fn part1(debug: bool, input: &str) -> Result<String, String> {
    let (grid, start_position) = parse_input(input);
    let start_position = if let Some(sp) = start_position {
        sp
    } else {
        return Err("Couldn't find starting position!".to_string());
    };

    let mut beams: Vec<bool> = vec![false; grid[0].len()];
    beams[start_position] = true;
    let rows: usize = grid.len();
    let cols: usize = grid[0].len();
    let mut num_splits: usize = 0;

    if debug {println!("{}", grid[0].iter().collect::<String>()); }
    for l in 1..rows {
        for c in 0..cols {
            match grid[l][c] {
                '^' => {
                    if beams[c] {
                        num_splits += 1;
                        beams[c] = false;
                        if c >= 1 {
                            beams[c-1] = true;
                        }
                        if c+1 < cols {
                            beams[c+1] = true;
                        }
                    }
                },
                _ => { },
            }
        }
        
        if debug {
            for c in 0..cols {
                if beams[c] {
                    print!("|");
                } else {
                    print!("{}", grid[l][c]);
                }
            }
            print!("\n");
        }
    }

    Ok(num_splits.to_string())
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Option<usize>) {
    let mut grid: Vec<Vec<char>> = vec![];
    let mut start_position: Option<usize> = None;

    for line in input.lines() {
        let line_chars: Vec<char> = line.trim().chars().collect();
        if start_position == None {
            start_position = line_chars.iter().position(|&c| c == 'S')
        }
        grid.push(line_chars);
    }

    (grid, start_position)
}

pub fn part2(debug: bool, input: &str) -> Result<String, String> {
    let (grid, start_position) = parse_input(input);
    let start_position = if let Some(sp) = start_position {
        sp
    } else {
        return Err("Couldn't find starting position!".to_string());
    };

    let rows: usize = grid.len();
    let cols: usize = grid[0].len();
    let mut beams: Vec<usize> = vec![0; grid[0].len()];
    beams[start_position] = 1;

    for l in 1..rows {
        for c in 0..cols {
            match grid[l][c] {
                '^' => {
                    if beams[c] > 0 {
                        if c >= 1 {
                            beams[c-1] += beams[c];
                        }
                        if c+1 < cols {
                            beams[c+1] += beams[c];
                        }
                        beams[c] = 0;
                    }
                },
                _ => { },
            }
        }
        
        if debug { println!("{} beams: {:?}", l, beams); }
    }

    Ok(beams.iter().sum::<usize>().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = include_str!("../../example_inputs/day07-1.txt");
    const EXAMPLE2: &str = include_str!("../../example_inputs/day07-1.txt");

    #[test]
    fn example_part1() {
        assert_eq!(part1(false, EXAMPLE1), Ok("21".to_string()));
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(false, EXAMPLE2), Ok("40".to_string()));
    }
}