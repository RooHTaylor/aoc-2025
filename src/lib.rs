use std::{
    fs, path::PathBuf, process, str::FromStr
};

// Check if the input file exists and is a file. Open it and return the contents
// as a String.
//
// # Exits - code 1
//
// This function will exit the process if the file does not exist or is not a file
pub fn load_input_file(filepath: &PathBuf) -> String {

    if !filepath.exists() || !filepath.is_file() {
        eprintln!("The input file either doesn't exist, or isn't a file.");
        eprintln!("Try running with -r|--reuse to reuse the input from part 1?");
        process::exit(1);
    }

    let file_contents = match fs::read_to_string(filepath) {
        Ok(s) => s,
        Err(err) => {
            eprintln!("There was a problem reading from the input file: {err}");
            process::exit(1);
        }
    };

    file_contents
}

// Generate the input file name from a day and part number. Use example_inputs if
// testing.
pub fn generate_filename(day: u8, part: u8, test: bool) -> PathBuf {
    let mut filename= if test {
        String::from_str("./example_inputs/day").unwrap()
    } else {
        String::from_str("./inputs/day").unwrap()
    };
    let day_string = if day < 10 {
        format!("0{day}")
    } else {
        format!("{day}")
    };
    filename.push_str(&day_string);
    filename.push_str("-");
    let part_string = format!("{part}");
    filename.push_str(&part_string);
    filename.push_str(".txt");

    PathBuf::from(&filename)
}