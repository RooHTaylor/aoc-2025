#[derive(Debug)]
struct Machine {
    target: usize,
    buttons: Vec<usize>,
    joltages: Vec<usize>,
}

use regex::Regex;

fn parse_input(input: &str) -> Result<Vec<Machine>, String> {
    let mut machines: Vec<Machine> = vec![];

    let re = Regex::new(r"\[(?P<target>[\.|#]+)\] (?P<buttons>(?:\([\d,]+\) )+)\{(?P<joltages>[\d,]+)\}").unwrap();

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let Some(caps) = re.captures(line) else {
            continue;
        };

        // Parse the target
        let mut target: usize = 0;
        let mut size: usize = 0;
        for char in caps["target"].chars() {
            match char {
                '.' => {
                    target = target << 1
                },
                '#' => {
                    target = (target << 1) + 1
                },
                _ => {
                    return Err(format!("Invalid target char encountered! {}", char))
                }
            }
            size += 1;
        }

        // Parse the buttons
        let mut buttons: Vec<usize> = vec![];
        let clean_buttons = caps["buttons"].replace("(", "").replace(")", "");
        let button_split = clean_buttons.split(" ");
        for b in button_split {
            let indicies: Vec<usize> = b.split(",").filter_map(|i| i.parse::<usize>().ok()).collect();
            let mut button: usize = 0;
            for i in indicies {
                if i >= size {
                    return Err("Button index greater than size of target!".to_string())
                }
                button += 1 << (size - i - 1);
            }
            buttons.push(button);
        }
        if buttons.len() < 1 {
            return Err("No buttons found!".to_string())
        }

        // Parse the joltages
        let clean_joltages = caps["joltages"].replace("{", "").replace("}", "");
        let joltages: Vec<usize> = clean_joltages.split(",").filter_map(|i| i.parse::<usize>().ok()).collect();
        if joltages.len() != size {
            return Err("Different number of bits in target and joltage!".to_string())
        }
        
        machines.push( Machine { target, buttons, joltages } );
    }

    if machines.len() < 1 {
        return Err("No machines found!".to_string())
    }

    Ok(machines)
}

fn recurse_buttons(debug: bool, state: usize, target: usize, buttons: &Vec<usize>, pressed_buttons: Vec<bool>, width: usize, smallest: &mut usize) {

    for (i, b) in buttons.iter().enumerate() {
        // Skip if we have already pressed this button (since buttons can only 
        // be pressed or not pressed, and any further pressing is identical to 
        // either pressing once or not pressing)
        if pressed_buttons[i] {
            if debug { println!("We've already pressed {} - skipping", i); }
            continue;
        }
        let mut pressed_buttons_clone = pressed_buttons.clone();
        pressed_buttons_clone[i] = true;

        let new_state = state ^ b;
        if debug { println!("XORing {:0width$b} with {:0width$b} to get {:0width$b}", state, b, new_state); }
        // If we're at the target state then return
        if new_state == target {
            if debug { println!("Found the target! New smallest."); }
            *smallest = pressed_buttons_clone.iter().filter(|&&b| b).count();
            return
        }
        let num_buttons = pressed_buttons_clone.iter().filter(|&&b| b).count();
        if debug { println!("We've pressed {} buttons", num_buttons); }
        // Don't keep checking if we're already past the number of button presses
        if num_buttons >= *smallest {
            if debug { println!("We've passed {} - returning", smallest); }
            return
        }
        if debug { println!("Not at the target yet! Recursing."); }
        recurse_buttons(debug, new_state, target, buttons, pressed_buttons_clone, width, smallest);
    }
}

// Part 1
pub fn part1(debug: bool, input: &str) -> Result<String, String> {

    let machines = match parse_input(input) {
        Ok(m) => m,
        Err(s) => {
            return Err(s)
        },
    };

    let mut total: usize = 0;
    for machine in &machines {
        if debug { println!("{:?}", machine); }

        let mut smallest: usize = machine.buttons.len() + 1;
        let pressed_buttons = vec![false; machine.buttons.len()];

        recurse_buttons(debug, 0, machine.target, &machine.buttons, pressed_buttons, machine.joltages.len(), &mut smallest);
        total = match total.checked_add(smallest) {
            Some(t) => t,
            None => {
                return Err("Total overflow occurred!".to_string())
            }
        };
    }

    Ok(total.to_string())
}

// Part 2
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
        assert_eq!(part1(false, EXAMPLE1), Ok("7".to_string()));
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(false, EXAMPLE2), Ok("".to_string()));
    }
}