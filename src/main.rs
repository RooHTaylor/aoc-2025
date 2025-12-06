mod days;

use aoc_2025::*;
use clap::Parser;
use std::{
    process,
    time::Instant,
};

/// Advent Of Code 2025
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Which day solution to run
    #[arg(short, long)]
    day: u8,

    /// Which part solution to run
    #[arg(short, long)]
    part: u8,

    /// Reuse the input file for part 1
    #[arg(short, long, default_value_t = false)]
    reuse: bool,

    /// Toggle using example input instead of use input
    #[arg(short, long, default_value_t = false)]
    test: bool,

    /// Toggle debug messages
    #[arg(long, default_value_t = false)]
    debug: bool,
}

fn main() {
    let start = Instant::now();
    // Parse args
    let args = Args::parse();
    if args.debug { println!("Args: {:?}", args); }
    if args.day < 1 || args.day > 25 {
        eprintln!("Invalid day! [1-25]");
        process::exit(1);
    }
    if args.part < 1 || args.part > 2 {
        eprintln!("Invalid part! [1|2]");
        process::exit(1);
    }

    // Safe to unwrap here, because we check for day>0 above
    let day_index = (args.day - 1) as usize;
    if day_index >= days::DAYS.len() {
        eprintln!("No solver for day {}", args.day);
        process::exit(1);
    }

    // Generate input filename
    let file_part = if args.reuse { 1 } else { args.part };
    let file_name = generate_filename(args.day, file_part, args.test);
    if args.debug { println!("Input filename: {}", file_name.to_string_lossy()); }
    // Load input file cotents into a String
    let input = load_input_file(&file_name);

    let day_entry = &days::DAYS[day_index];
    let result = match args.part {
        1 => (day_entry.part1)(args.debug, &input),
        2 => (day_entry.part2)(args.debug, &input),
        _ => {
            eprintln!("Part must be 1 or 2!");
            std::process::exit(1);
        }
    };
    let duration = start.elapsed();
    match result {
        Ok(r) => {
            println!("Answer: {}", r);
            println!("Solution runtime: {:.6}", duration.as_secs_f64());
        },
        Err(e) => {
            eprintln!("Error! {}", e);
            println!("Solution runtime: {:.6}", duration.as_secs_f64());
            process::exit(1)
        },
    }
}
