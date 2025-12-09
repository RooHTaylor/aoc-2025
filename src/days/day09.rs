fn parse_input (input: &str) -> Result<Vec<(usize, usize)>, String> {
    let mut coordinates: Vec<(usize, usize)> = vec![];

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.trim().split(',').collect();
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

            // The absolute difference between the coords is the side lenths
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

pub fn part2(_debug: bool, _input: &str) -> Result<String, String> {

    Ok("".to_string())
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
        assert_eq!(part2(false, EXAMPLE2), Ok("".to_string()));
    }
}