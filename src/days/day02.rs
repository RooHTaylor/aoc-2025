
// Part 1
pub fn part1(debug: bool, input: &str) -> Result<String, String> {
    let mut sum: usize = 0;

    let range_strings = input.trim().split(',');
    for range in range_strings {
        let (first_id, last_id) = match range.split_once('-') {
            Some(i) => i,
            None => {
                return Err(format!("Invalid range format! {range}"))
            }
        };
        let first_id = match first_id.parse::<usize>() {
            Ok(n) => n,
            Err(_) => {
                return Err(format!("Invalid first id! {range}"))
            }
        };
        let last_id = match last_id.parse::<usize>() {
            Ok(n) => n,
            Err(_) => {
                return Err(format!("Invalid last id! {range}"))
            }
        };
        if debug { println!("Checking range {} to {}", first_id, last_id); }

        for i in first_id..=last_id {
            let i_string = i.to_string();
            // If there's an even number of chars it can be split evenly
            if i_string.len() % 2 == 0 {
                let half: usize = i_string.len() / 2;
                let (first_half, second_half) = i_string.split_at(half);
                if first_half == second_half {
                    if debug { println!("INVALID! The first half ({}) of {} matches the second half ({})", first_half, i, second_half); }
                    sum += i;
                }
            }
        }
    }

    Ok(sum.to_string())
}

// Part 2 solution
pub fn part2(debug: bool, input: &str) -> Result<String, String> {
    let mut sum: usize = 0;

    let range_strings = input.trim().split(',');
    for range in range_strings {
        let (first_id, last_id) = match range.split_once('-') {
            Some(i) => i,
            None => {
                return Err(format!("Invalid range format! {range}"))
            }
        };
        let first_id = match first_id.parse::<usize>() {
            Ok(n) => n,
            Err(_) => {
                return Err(format!("Invalid first id! {range}"))
            }
        };
        let last_id = match last_id.parse::<usize>() {
            Ok(n) => n,
            Err(_) => {
                return Err(format!("Invalid last id! {range}"))
            }
        };
        if debug { println!("Checking range {} to {}", first_id, last_id); }

        'id: for i in first_id..=last_id {
            let i_string = i.to_string();
            let length = i_string.len();
            
            // Try to break the string into equal chunks of length n
            // Compare each chunk and skip if they aren't equal (valid id)
            let half_len = length / 2;
            'chunk_size: for n in 1..=half_len {
                if length % n != 0 {
                    if debug { println!("Can't split {i} into groups of {n}"); }
                    continue;
                }

                let mut chunks = i_string.as_bytes().chunks(n);
                let Some(first_chunk) = chunks.next() else {
                    return Err(format!("Failed to break {i} into chunks"))
                };

                for chunk in chunks {
                    if chunk != first_chunk {
                        continue 'chunk_size;
                    }
                }

                // If we get here, all chunks matched
                sum += i;
                continue 'id;
            }
        }
    }

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = include_str!("../../example_inputs/day02-1.txt");
    const EXAMPLE2: &str = include_str!("../../example_inputs/day02-1.txt");

    #[test]
    fn example_part1() {
        assert_eq!(part1(false, EXAMPLE1), Ok("1227775554".to_string()));
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(false, EXAMPLE2), Ok("4174379265".to_string()));
    }
}
