use std::cmp::min;
use std::collections::HashSet;

#[derive(Debug, Copy, Clone)]
struct JunctionBox {
    x: isize,
    y: isize,
    z: isize,
    circuit: usize,
}

fn parse_input(input: &str) -> Result<Vec<JunctionBox>, String> {
    let mut boxes: Vec<JunctionBox> = vec![];

    for line in input.lines().enumerate() {
        if line.1.trim().is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.1.split(',').collect();
        if parts.len() != 3 {
            return Err(format!("Invalid input! {}", line.1))
        }

        let x: isize = match parts[0].parse() {
            Ok(n) => n,
            Err(_) => {
                return Err(format!("Invalid x input! {}", line.1))
            },
        };
        let y: isize = match parts[1].parse() {
            Ok(n) => n,
            Err(_) => {
                return Err(format!("Invalid y input! {}", line.1))
            },
        };
        let z: isize = match parts[2].parse() {
            Ok(n) => n,
            Err(_) => {
                return Err(format!("Invalid z input! {}", line.1))
            },
        };

        boxes.push(JunctionBox { x, y, z, circuit: line.0 });
    }

    Ok(boxes)
}

// Part 1
pub fn part1(debug: bool, input: &str) -> Result<String, String> {

    let mut boxes = match parse_input(input) {
        Ok(b) => b,
        Err(e) => {
            return Err(e);
        },
    };
    if debug { println!("Junction boxes: {:?}", boxes); }

    let limit = if boxes.len() == 20 {
        // exmaple input!
        10
    } else {
        1000
    };

    // Fill circuits map
    let mut circuits: Vec<Vec<usize>> = vec![];
    for i in 0..boxes.len() {
        let circuit: Vec<usize> = vec![i];
        circuits.push(circuit);
    }
    if debug { println!("Circuits: {:?}", circuits); }

    let mut joined: usize = 0;
    let mut direct_connections: HashSet<(usize, usize)> = HashSet::new();
    loop {
        // Break out of the loop if we've joined all the boxes, or 1000 of them
        if joined == min(boxes.len(), limit) {
            break;
        }

        let mut min_distance: (f64, usize, usize) = (f64::MAX, 0, 0);

        // Loop over each box
        for i in 0..boxes.len() {
            for ci in 0..boxes.len() {
                // Do not compare against ourself.
                if ci == i {
                    continue;
                }

                // Do not consider if they are already connected
                if direct_connections.contains(&(i, ci)) {
                    continue;
                }

                if debug { println!("Checking {:?} and {:?}", boxes[i], boxes[ci]); }

                // Calculate the euclidean distance between the two and save if
                // it's the smallest
                let distance = (
                    (boxes[i].x - boxes[ci].x).pow(2) as f64 + 
                    (boxes[i].y - boxes[ci].y).pow(2) as f64 + 
                    (boxes[i].z - boxes[ci].z).pow(2) as f64
                ).sqrt();
                if debug { println!("Distance: {}", distance); }
                if distance < min_distance.0 {
                    if debug { println!("Found new shortest distance {} between boxes {} and {}", distance, i, ci); }
                    min_distance.0 = distance;
                    min_distance.1 = i;
                    min_distance.2 = ci;
                }
            }
        }

        // Check that we found a min
        if min_distance.0 != f64::MAX {
            if debug { println!("Connecting {:?} and {:?}", boxes[min_distance.1], boxes[min_distance.2]); }
            direct_connections.insert((min_distance.1, min_distance.2));
            direct_connections.insert((min_distance.2, min_distance.1));
            let c1 = boxes[min_distance.1].circuit;
            let c2 = boxes[min_distance.2].circuit;
            if c1 == c2 {
                if debug { println!("Connected. Nothing happened!"); }
                // BUT a connection was made.
                joined += 1;
                if debug { println!("{:?}", circuits); }
                if debug { println!("Joined: {}", joined); }
                continue;
            }
            if debug {println!("Merging circuit {} and {}", c1, c2); }
            // Loop over all members of c2
            for icb in 0..circuits[c2].len() {
                // For each box in c2
                let bx = circuits[c2][icb];
                if debug { println!("Found {:?} in circuit {}", boxes[bx], c2); }
                // Set the new circuit to c1
                boxes[bx].circuit = c1;
                // Push the box id into the new circuit list
                circuits[c1].push(bx);
            }
            // Clear the now empty c2 list, as they're all in c1 now
            circuits[c2] = vec![];
            joined += 1;
        }

        if debug { println!("{:?}", circuits); }
        if debug { println!("Joined: {}", joined); }
        if debug { println!("Boxes: {:?}", boxes); }
    }

    circuits.sort_by(|c1, c2| c2.len().cmp(&c1.len()));
    if debug { println!("{:?}", circuits); }
    let mut totals: Vec<usize> = vec![];
    for i in 0..3 {
        totals.push(
            circuits[i].len()
        );
    }
    if debug { println!("{:?}", totals); }

    Ok(totals.iter().product::<usize>().to_string())
}

pub fn part2(debug: bool, input: &str) -> Result<String, String> {
    let mut boxes = match parse_input(input) {
        Ok(b) => b,
        Err(e) => {
            return Err(e);
        },
    };
    if debug { println!("Junction boxes: {:?}", boxes); }

    // Fill circuits map
    let mut circuits: Vec<Vec<usize>> = vec![];
    for i in 0..boxes.len() {
        let circuit: Vec<usize> = vec![i];
        circuits.push(circuit);
    }
    if debug { println!("Circuits: {:?}", circuits); }

    let mut direct_connections: HashSet<(usize, usize)> = HashSet::new();
    let mut last_two: (usize, usize) = (0, 0);
    loop {
        // Break out of the loop if we've joined all the boxes to the same circuit
        if circuits.iter().map(|v| v.len()).max().unwrap_or(0) == boxes.len() {
            break;
        }

        let mut min_distance: (f64, usize, usize) = (f64::MAX, 0, 0);

        // Loop over each box
        for i in 0..boxes.len() {
            for ci in 0..boxes.len() {
                // Do not compare against ourself.
                if ci == i {
                    continue;
                }

                // Do not consider if they are already connected
                if direct_connections.contains(&(i, ci)) {
                    continue;
                }

                //if debug { println!("Checking {:?} and {:?}", boxes[i], boxes[ci]); }

                // Calculate the euclidean distance between the two and save if
                // it's the smallest
                let distance = (
                    (boxes[i].x - boxes[ci].x).pow(2) as f64 + 
                    (boxes[i].y - boxes[ci].y).pow(2) as f64 + 
                    (boxes[i].z - boxes[ci].z).pow(2) as f64
                ).sqrt();
                //if debug { println!("Distance: {}", distance); }
                if distance < min_distance.0 {
                    if debug { println!("Found new shortest distance {} between boxes {} and {}", distance, i, ci); }
                    min_distance.0 = distance;
                    min_distance.1 = i;
                    min_distance.2 = ci;
                }
            }
        }

        // Check that we found a min
        if min_distance.0 != f64::MAX {
            if debug { println!("Connecting {:?} and {:?}", boxes[min_distance.1], boxes[min_distance.2]); }
            direct_connections.insert((min_distance.1, min_distance.2));
            direct_connections.insert((min_distance.2, min_distance.1));
            last_two = (min_distance.1, min_distance.2);
            let c1 = boxes[min_distance.1].circuit;
            let c2 = boxes[min_distance.2].circuit;
            if c1 == c2 {
                if debug { println!("Connected. Nothing happened!"); }
                // BUT a connection was made.
                if debug { println!("{:?}", circuits); }
                continue;
            }
            if debug {println!("Merging circuit {} and {}", c1, c2); }
            // Loop over all members of c2
            for icb in 0..circuits[c2].len() {
                // For each box in c2
                let bxi = circuits[c2][icb];
                if debug { println!("Found {:?} in circuit {}", boxes[bxi], c2); }
                // Set the new circuit to c1
                if debug { println!("Setting circuit of box {} to {}", bxi, c1); }
                boxes[bxi].circuit = c1;
                // Push the box id into the new circuit list
                circuits[c1].push(bxi);
            }
            // Clear the now empty c2 list, as they're all in c1 now
            circuits[c2] = vec![];
        }

        if debug { println!("{:?}", circuits); }
        if debug { println!("Boxes: {:?}", boxes); }
    }

    let answer: isize = boxes[last_two.0].x * boxes[last_two.1].x;

    Ok(answer.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = include_str!("../../example_inputs/day08-1.txt");
    const EXAMPLE2: &str = include_str!("../../example_inputs/day08-1.txt");

    #[test]
    fn example_part1() {
        assert_eq!(part1(false, EXAMPLE1), Ok("40".to_string()));
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(false, EXAMPLE2), Ok("25272".to_string()));
    }
}