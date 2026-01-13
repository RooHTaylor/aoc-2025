fn parse_input (input: &str) -> Result<Vec<(usize, usize)>, String> {
    let mut coordinates: Vec<(usize, usize)> = vec![];

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.trim().split(',').collect();
        if parts.len() != 2 {
            return Err(format!("Invalid input: {}", line))
        }
        let x = match parts[0].parse::<usize>() {
            Ok(n) => n,
            Err(_) => {
                return Err(format!("Could not convert {} to usize", parts[0]));
            }
        };
        let y = match parts[1].parse::<usize>() {
            Ok(n) => n,
            Err(_) => {
                return Err(format!("Could not convert {} to usize", parts[1]));
            }
        };
        coordinates.push((x, y));
    }

    Ok(coordinates)
}

// Part 1
pub fn part1(debug: bool, input: &str) -> Result<String, String> {

    let coordinates: Vec<(usize, usize)> = match parse_input(input) {
        Ok(v) => v,
        Err(s) => {
            return Err(s);
        }
    };
    if debug { println!("{:?}", coordinates); }

    let mut largest_area: usize = 0;
    for i in 0..coordinates.len() {
        let x = coordinates[i].0;
        let y = coordinates[i].1;

        for si in 0..coordinates.len() {
            // Don't compare against ourself
            if i == si { 
                continue;
            }

            let sx = coordinates[si].0;
            let sy = coordinates[si].1;

            if debug { println!("Comparing {},{} and {},{}", x, y, sx, sy); }

            // The absolute difference between the coords is the side lengths
            let length = x.abs_diff(sx) + 1;
            if debug { println!("Length: {}", length); }
            let width = y.abs_diff(sy) + 1;
            if debug { println!("Width: {}", width); }

            let area = length * width;
            if debug { println!("Area: {}", area); }
            if area > largest_area {
                if debug { println!("New largest area!"); }
                largest_area = area;
            }
        }
    }

    Ok(largest_area.to_string())
}

fn draw_lines(coordinates: &Vec<(usize, usize)>) -> Vec<((usize, usize), (usize, usize))> {
    let mut lines: Vec<((usize, usize), (usize, usize))> = vec![];

    let mut first: Option<(usize, usize)> = None;
    for i in 0..coordinates.len() {
        if let Some(f) = first {
            let s = coordinates[i].clone();
            lines.push((f,s));
            first = Some(s);
            continue;
        } else {
            first = Some(coordinates[i].clone());
            continue;
        }
    }
    if let Some(f) = first {
        let s = coordinates[0].clone();
        lines.push((f,s));
    }

    lines
}

use std::cmp::{min, max};
pub fn part2(debug: bool, input: &str) -> Result<String, String> {
    let coordinates = match parse_input(input) {
        Ok(c) => c,
        Err(s) => {
            return Err(s)
        }
    };
    if debug { println!("Coordinates: {:?}", coordinates); }

    let lines = draw_lines(&coordinates);
    if debug { println!("Lines: {:?}", lines); }

    let mut largest_area: usize = 0;
    for i in 0..coordinates.len() {
        let x = coordinates[i].0;
        let y = coordinates[i].1;

        'second: for si in 0..coordinates.len() {
            // Don't compare against ourself
            if i == si { 
                continue;
            }

            let sx = coordinates[si].0;
            let sy = coordinates[si].1;

            if debug { println!("Comparing {},{} and {},{}", x, y, sx, sy); }

            // The absolute difference between the coords is the side lengths
            let length = x.abs_diff(sx) + 1;
            if debug { println!("Length: {}", length); }
            let width = y.abs_diff(sy) + 1;
            if debug { println!("Width: {}", width); }

            let area = length * width;
            if debug { println!("Area: {}", area); }
            // Don't bother checking if the area isn't a winner.
            if area <= largest_area {
                if debug { println!("The area is smaller. Skipping..."); }
                continue;
            }

            // Create the box with the two points. Check to see if any of the
            // lines exist inside the rectangle.
            for line in &lines {
                if debug { println!("Checking line: {:?}", line); }
                if line.0.0 == line.1.0 &&
                    line.0.0 > min(x, sx) &&
                    line.0.0 < max(x, sx)
                {
                    if debug { println!("Vertical line exists within the x range"); }
                    // x is the same, line is vertical
                    if min(line.0.1, line.1.1) < min(y, sy) &&
                        max(line.0.1, line.1.1) > min(y, sy)
                    {
                        // starts before, ends within or after
                        if debug { println!("Line starts before and ends within or after"); }
                        continue 'second;
                    } else if min(line.0.1, line.1.1) >= min(y, sy) &&
                        min(line.0.1, line.1.1) < max(y, sy) &&
                        max(line.0.1, line.1.1) >= min(y, sy)
                    {
                        // starts within, ends within or after
                        if debug { println!("Line starts within and ends within or after"); }
                        continue 'second;
                    }

                } else if line.0.1 == line.1.1 &&
                    line.0.1 > min(y, sy) &&
                    line.0.1 < max(y, sy)
                {
                    if debug { println!("Horizontal line exists within the y range"); }
                    // y is the same, line is horizontal
                    if min(line.0.0, line.1.0) < min(x, sx) &&
                        max(line.0.0, line.1.0) > min(x, sx)
                    {
                        if debug { println!("Line starts before and ends within or after"); }
                        // starts before, ends within or after
                        continue 'second;
                    } else if min(line.0.0, line.1.0) >= min(x, sx) &&
                        min(line.0.0, line.1.0) < max(x, sx) &&
                        max(line.0.0, line.1.0) >= min(x, sx)
                    {
                        // starts within, ends within or after
                        if debug { println!("Line starts within and ends within or after"); }
                        continue 'second;
                    }
                }
                if debug { println!("No conflict!"); }
            }

            // If we've made it here then we've checked all the lines and none
            // failed. This is the new largest
            if debug { println!("New largest area!"); }
            largest_area = area;
        }
    }

    Ok(largest_area.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = include_str!("../../example_inputs/day09-1.txt");
    const EXAMPLE2: &str = include_str!("../../example_inputs/day09-1.txt");

    #[test]
    fn example_part1() {
        assert_eq!(part1(false, EXAMPLE1), Ok("50".to_string()));
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(false, EXAMPLE2), Ok("24".to_string()));
    }
}