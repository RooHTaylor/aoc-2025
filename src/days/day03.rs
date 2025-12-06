
// Part 1
pub fn part1(debug: bool, input: &str) -> Result<String, String> {
    let mut sum: usize = 0;

    for line in input.lines() {
        let bank = line.trim();
        if bank.is_empty() {
            continue;
        }
        if debug { println!("Bank: {bank}"); }

        let mut first_battery: Option<u8> = None;
        let mut second_battery: Option<u8> = None;

        let batteries: Vec<char> = bank.chars().collect();
        for (i, b) in batteries.iter().enumerate() {

            // Check if we already have the largest possible joltage (99)
            if let Some( fb) = first_battery {
                if let Some(sb) = second_battery {
                    if fb == 9 && sb == 9 {
                        if debug { println!("We already have the largest possible joltage. Skipping battery"); }
                        break;
                    }
                }
            }
            let digit: u8 = match b.clone().to_digit(10) {
                Some(d) => d as u8,
                None => return Err(format!("Could not convert {} to digit", b))
            };

            if let Some(bat) = first_battery {
                // If the current battery is larger than the set first battery
                // and there is at least one more bettery to look at, then this
                // is the new first battery
                if digit > bat && i < batteries.len()-1 {
                    if debug { println!("first {digit} > {bat} ... setting"); }
                    first_battery = Some(digit);

                    // if we set the first didgit we have to reset the second digit
                    second_battery = None;

                    continue;
                }
            } else {
                if debug { println!("first not set. Setting to {digit}"); }
                first_battery = Some(digit);
                continue;
            }

            if let Some(bat) = second_battery {
                // If the current battery is larger than the set second battery
                // then this is the new second battery
                if digit > bat {
                    if debug { println!("second {digit} > {bat} ... setting"); }
                    second_battery = Some(digit);
                    continue;
                }
            } else {
                if debug { println!("second not set. Setting to {digit}"); }
                second_battery = Some(digit);
                continue;
            }
        }

        let mut joltage: u8 = 0;
        if let Some( fb) = first_battery {
            if let Some(sb) = second_battery {
                joltage += (10 * fb) + sb;
            } else {
                joltage += fb
            }
        }
        if debug { println!("Adding {joltage}"); }

        sum += joltage as usize;
    }

    Ok(sum.to_string())
}

// Part 2 solution
pub fn part2(_debug: bool, _input: &str) -> Result<String, String> {
    Ok(String::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = include_str!("../../example_inputs/day03-1.txt");
    const EXAMPLE2: &str = include_str!("../../example_inputs/day03-1.txt");

    #[test]
    fn example_part1() {
        assert_eq!(part1(false, EXAMPLE1), Ok("357".to_string()));
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(false, EXAMPLE2), Ok("".to_string()));
    }
}
