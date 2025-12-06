// Part 1
pub fn part1(debug: bool, input: &str) -> Result<String, String> {
    let grid = parse_grid(input);
    
    let mut movable_rolls: Vec<(usize, usize)> = vec![];
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if debug { println!("x: {x} y:{y}"); }
            // only check rolls of paper
            if grid[y][x] != '@' {
                if debug { println!("Not a roll. {}", grid[y][x]); }
                continue;
            }

            let mut adjacent_rolls: u8 = 0;
            // (x, y)
            let modifiers: Vec<(i8, i8)> = vec![(-1, -1), (0, -1), (1, -1), (-1, 0),
                (1, 0), (-1, 1), (0, 1), (1, 1)];
            for modifier in modifiers {
                if debug {
                    match modifier {
                        (-1, -1) => { println!("checking top left") },
                        (0, -1) => { println!("checking top middle") },
                        (1, -1) => { println!("checking top right") },
                        (-1, 0) => { println!("checking left") },
                        (1, 0) => { println!("checking right") },
                        (-1, 1) => { println!("checking bottom left") },
                        (0, 1) => { println!("checking bottom middle") },
                        (1, 1) => { println!("checking bottom right") },
                        _ => { }
                    }
                }
                let cx: usize = if modifier.0 < 0 {
                    match x.checked_sub(modifier.0.abs() as usize) {
                        Some(n) => {
                            // Out of bounds, therefore not occupied
                            if n >= grid[y].len() {
                                if debug { println!("cx >= {}, not occupied", grid[y].len()); }
                                continue;
                            }
                            n
                        },
                        // Out of bounds, therefore not occupied
                        None => {
                            if debug { println!("cx < 0, not occupied"); }
                            continue;
                        }
                    }
                } else {
                    match x.checked_add(modifier.0 as usize) {
                        Some(n) => {
                            // Out of bounds, therefore not occupied
                            if n >= grid[y].len() {
                                if debug { println!("cx >= {}, not occupied", grid[y].len()); }
                                continue;
                            }
                            n
                        },
                        // Out of bounds, therefore not occupied
                        None => { 
                            if debug { println!("cx >= {}, not occupied", usize::MAX); }
                            continue;
                        }
                    }
                };
                if debug { println!("cx: {cx} in bounds"); }
                let cy: usize = if modifier.1 < 0 {
                    match y.checked_sub(modifier.1.abs() as usize) {
                        Some(n) => {
                            if n >= grid.len() {
                                // Out of bounds, therefore not occupied
                                if debug { println!("cy >= {}, not occupied", grid.len()); }
                                continue;
                            }
                            n
                        },
                        // Out of bounds, therefore not occupied
                        None => { 
                            if debug { println!("cy < 0, not occupied"); }
                            continue;
                        }
                    }
                } else {
                    match y.checked_add(modifier.1 as usize) {
                        Some(n) => {
                            // Out of bounds, therefore not occupied
                            if n >= grid.len() {
                                if debug { println!("cy >= {}, not occupied", grid.len()); }
                                continue;
                            }
                            n
                        },
                        // Out of bounds, therefore not occupied
                        None => { 
                            if debug { println!("cy >= {}, not occupied", usize::MAX); }
                            continue;
                        }
                    }
                };
                if debug { println!("cy: {cy} in bounds"); }
                if grid[cy][cx] == '@' {
                    adjacent_rolls += 1;
                    if debug { println!("{cx},{cy} is an @ - found {adjacent_rolls} adacent rolls")}
                }
            }
            if adjacent_rolls < 4 {
                if debug { println!("Less than 4 rolls next to {x},{y} so it can be moved.")}
                movable_rolls.push((x, y));
            }
        } 
    }

    Ok(movable_rolls.len().to_string())
}

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = vec![];

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let row: Vec<char> = line.trim().chars().collect();
        grid.push(row);
    }
    grid
}

// Part 2
pub fn part2(debug: bool, input: &str) -> Result<String, String> {
    let mut grid = parse_grid(input);
    let mut sum: usize = 0;

    loop {
        let mut movable_rolls: Vec<(usize, usize)> = vec![];
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if debug { println!("x: {x} y:{y}"); }
                // only check rolls of paper
                if grid[y][x] != '@' {
                    if debug { println!("Not a roll. {}", grid[y][x]); }
                    continue;
                }

                let mut adjacent_rolls: u8 = 0;
                // (x, y)
                let modifiers: Vec<(i8, i8)> = vec![(-1, -1), (0, -1), (1, -1), (-1, 0),
                    (1, 0), (-1, 1), (0, 1), (1, 1)];
                for modifier in modifiers {
                    if debug {
                        match modifier {
                            (-1, -1) => { println!("checking top left") },
                            (0, -1) => { println!("checking top middle") },
                            (1, -1) => { println!("checking top right") },
                            (-1, 0) => { println!("checking left") },
                            (1, 0) => { println!("checking right") },
                            (-1, 1) => { println!("checking bottom left") },
                            (0, 1) => { println!("checking bottom middle") },
                            (1, 1) => { println!("checking bottom right") },
                            _ => { }
                        }
                    }
                    let cx: usize = if modifier.0 < 0 {
                        match x.checked_sub(modifier.0.abs() as usize) {
                            Some(n) => {
                                // Out of bounds, therefore not occupied
                                if n >= grid[y].len() {
                                    if debug { println!("cx >= {}, not occupied", grid[y].len()); }
                                    continue;
                                }
                                n
                            },
                            // Out of bounds, therefore not occupied
                            None => {
                                if debug { println!("cx < 0, not occupied"); }
                                continue;
                            }
                        }
                    } else {
                        match x.checked_add(modifier.0 as usize) {
                            Some(n) => {
                                // Out of bounds, therefore not occupied
                                if n >= grid[y].len() {
                                    if debug { println!("cx >= {}, not occupied", grid[y].len()); }
                                    continue;
                                }
                                n
                            },
                            // Out of bounds, therefore not occupied
                            None => { 
                                if debug { println!("cx >= {}, not occupied", usize::MAX); }
                                continue;
                            }
                        }
                    };
                    if debug { println!("cx: {cx} in bounds"); }
                    let cy: usize = if modifier.1 < 0 {
                        match y.checked_sub(modifier.1.abs() as usize) {
                            Some(n) => {
                                if n >= grid.len() {
                                    // Out of bounds, therefore not occupied
                                    if debug { println!("cy >= {}, not occupied", grid.len()); }
                                    continue;
                                }
                                n
                            },
                            // Out of bounds, therefore not occupied
                            None => { 
                                if debug { println!("cy < 0, not occupied"); }
                                continue;
                            }
                        }
                    } else {
                        match y.checked_add(modifier.1 as usize) {
                            Some(n) => {
                                // Out of bounds, therefore not occupied
                                if n >= grid.len() {
                                    if debug { println!("cy >= {}, not occupied", grid.len()); }
                                    continue;
                                }
                                n
                            },
                            // Out of bounds, therefore not occupied
                            None => { 
                                if debug { println!("cy >= {}, not occupied", usize::MAX); }
                                continue;
                            }
                        }
                    };
                    if debug { println!("cy: {cy} in bounds"); }
                    if grid[cy][cx] == '@' {
                        adjacent_rolls += 1;
                        if debug { println!("{cx},{cy} is an @ - found {adjacent_rolls} adacent rolls")}
                    }
                }
                if adjacent_rolls < 4 {
                    if debug { println!("Less than 4 rolls next to {x},{y} so it can be moved.")}
                    movable_rolls.push((x, y));
                }
            } 
        }

        if movable_rolls.len() == 0 {
            break;
        }
        if debug { println!("Moving {} rolls", movable_rolls.len()); }
        sum += movable_rolls.len();
        for roll in movable_rolls {
            grid[roll.1][roll.0] = '.';
        }
    }

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = include_str!("../../example_inputs/day04-1.txt");
    const EXAMPLE2: &str = include_str!("../../example_inputs/day04-1.txt");

    #[test]
    fn example_part1() {
        assert_eq!(part1(false, EXAMPLE1), Ok("13".to_string()));
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(false, EXAMPLE2), Ok("43".to_string()));
    }
}