// Part 1
pub fn part1(debug: bool, input: &str) -> Result<String, String> {

    let (fresh_ranges, stock) = match parse_input(input) {
        Ok(i) => i,
        Err(e) => {
            return Err(e)
        }
    };
    if debug { println!("Ranges: {:?}\nIds: {:?}", fresh_ranges, stock); }

    let mut fresh_count: usize = 0;
    'id: for id in &stock {
        'range: for range in &fresh_ranges {
            if *id < range.0 || *id > range.1 {
                continue 'range;
            }
            if debug { println!("{} is fresh for being between {} and {}", id, range.0, range.1)}
            fresh_count += 1;
            continue 'id;
        }
    }
    
    Ok(fresh_count.to_string())
}

fn parse_input(input: &str) -> Result<(Vec<(usize, usize)>, Vec<usize>), String> {
    let mut fresh_ranges: Vec<(usize, usize)> = vec![];
    let mut stock: Vec<usize> = vec![];

    let mut done_ranges = false;
    for line in input.lines() {
        if !done_ranges {
            if line.trim().is_empty() {
                // The first empty line we encounter will finish range parsing
                done_ranges = true;
                continue;
            }
            let parts: Vec<&str> = line.trim().split('-').collect();
            if parts.len() != 2 {
                return Err(format!("Invalid range format! {line}"))
            }
            let min = match parts[0].parse::<usize>() {
                Ok(n) => n,
                Err(_) => {
                    return Err(format!("Invalid range format! {line}"))
                }
            };
            let max = match parts[1].parse::<usize>() {
                Ok(n) => n,
                Err(_) => {
                    return Err(format!("Invalid range format! {line}"))
                }
            };

            fresh_ranges.push((min, max));
        } else {
            if line.trim().is_empty() {
                continue;
            }
            let id = match line.parse::<usize>() {
                Ok(n) => n,
                Err(_) => {
                    return Err(format!("Invalid id format! {line}"))
                }
            };

            stock.push(id);
        }
    }

    Ok((fresh_ranges, stock))
}

// Part 2
pub fn part2(debug: bool, input: &str) -> Result<String, String> {
    
    let (mut fresh_ranges, _) = match parse_input(input) {
        Ok(i) => i,
        Err(e) => {
            return Err(e)
        }
    };
    if debug { println!("Ranges: {:?}", fresh_ranges); }

    // Guarantee that ranges are ordered by the min value
    //fresh_ranges.sort_by_key(|k| k.0);
    fresh_ranges.sort();
    if debug { println!("Sorted Ranges: {} {:?}", fresh_ranges.len(), fresh_ranges); }

    let mut i: usize = 0;
    let mut found_overlaps = false;
    loop {
        let ni = i + 1;
        if i >= fresh_ranges.len() || ni >= fresh_ranges.len() {
            
            if debug { println!("{} is too high! Len {}", i, fresh_ranges.len()); }
            if found_overlaps {
                // If we found overlaps, we need to re-examine list to see if any 
                // new ranges also overlap.
                i = 0;
                found_overlaps = false;
                continue;
            } else {
                // Break the loop if we're out of bounds or if there is no next element
                // to combine with
                break;
            }
        }

        if fresh_ranges[ni].0 <= (fresh_ranges[i].1 + 1) {
            found_overlaps = true;
            if debug { println!("[{}] {}, {} and {}, {} overlap", i, fresh_ranges[i].0, fresh_ranges[i].1, fresh_ranges[ni].0, fresh_ranges[ni].1); }
            
            if fresh_ranges[ni].1 >= fresh_ranges[i].1 {
                if debug { println!("Next range end is larger, using it {}", fresh_ranges[ni].1); }
                fresh_ranges[i].1 = fresh_ranges[ni].1
            }
            
            fresh_ranges.remove(ni);
            if debug { println!("{} ranges now", fresh_ranges.len()); }
        } else {
            if debug { println!("[{}] {}, {} doesn't overlap {}, {}", i, fresh_ranges[i].0, fresh_ranges[i].1, fresh_ranges[ni].0, fresh_ranges[ni].1); }
            i += 1;
        }
    }
    if debug { println!("Consolidated Ranges: {:?}", fresh_ranges); }

    let mut fresh_count: usize = 0;
    for range in &fresh_ranges {
        let diff: usize = match range.1.checked_sub(range.0) {
            Some(d) => d,
            None => {
                return Err(format!("Could not subtract range! {}-{}", range.1, range.0))
            }
        };

        // Account for off-by-1
        fresh_count += diff + 1;
    }
    
    Ok(fresh_count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = include_str!("../../example_inputs/day05-1.txt");
    const EXAMPLE2: &str = include_str!("../../example_inputs/day05-1.txt");

    #[test]
    fn example_part1() {
        assert_eq!(part1(false, EXAMPLE1), Ok("3".to_string()));
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(false, EXAMPLE2), Ok("14".to_string()));
    }
}