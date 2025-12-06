
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
pub fn part2(debug: bool, input: &str) -> Result<String, String> {

    let mut sum: usize = 0;

    for line in input.lines() {
        let bank = line.trim();
        if bank.is_empty() {
            continue;
        }
        if debug { println!("Bank: {bank}"); }

        let mut on_batteries: Vec<Option<u8>> = vec![None, None, None, None, None, 
            None, None, None, None, None, None, None];

        let batteries: Vec<char> = bank.chars().collect();
        for (i, b) in batteries.iter().enumerate() {

            // Check if we already have the largest possible joltage (99)
            if on_batteries.iter().all(|&b| b == Some(9)) {
                if debug { println!("We already have the largest possible joltage. Skipping battery"); }
                break;
            }

            let digit: u8 = match b.clone().to_digit(10) {
                Some(d) => d as u8,
                None => return Err(format!("Could not convert {} to digit", b))
            };

            let mut set = false;
            for bi in 0..on_batteries.len() {
                // If we set a battery on the last iteration, all remaining batteries
                // are reset
                if set {
                    if debug { println!("We set a battery before, resetting {bi}"); }
                    on_batteries[bi] = None;
                    continue;
                }

                if let Some(b) = on_batteries[bi] {
                    // if the digit is bigger, and we have enough batteries left
                    // to fill the remaining on battery spots, then update it
                    if digit > b && (on_batteries.len() - (bi + 1)) <= (batteries.len() - (i + 1)) {
                        if debug { println!("{bi} {digit} > {b} ... updating")}
                        on_batteries[bi] = Some(digit);
                        // Must reset all following on batteries.
                        set = true;
                    }
                } else {
                    if debug { println!("{bi} is unset, setting to {digit}"); }
                    // No need to reset folloing on batteries, because they should be off
                    on_batteries[bi] = Some(digit);
                    break;
                }
            }
        }

        let mut joltage: usize = 0;
        for (i, b) in on_batteries.iter().enumerate() {
            let battery = if let Some(bat) = b {
                *bat as usize
            } else {
                return Err(format!("A battery position wasn't filled!"))
            };
            let base: usize = 10;
            let exp = on_batteries.len() as u32 - i as u32 - 1;
            let multiplier = match base.checked_pow(exp) {
                Some(m) => m,
                None => return Err(format!("Multiplier overflow!"))
            };
            let newjoltage = battery * multiplier;
            joltage = match joltage.checked_add(newjoltage) {
                Some(n) => n,
                None => return Err(format!("joltage size overflow!"))
            };
        }
        if debug { println!("Adding {joltage}"); }

        sum = match sum.checked_add(joltage) {
            Some(n) => n,
            None => return Err(format!("Sum overflow!"))
        };
    }

    Ok(sum.to_string())
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
        assert_eq!(part2(false, EXAMPLE2), Ok("3121910778619".to_string()));
    }
}
