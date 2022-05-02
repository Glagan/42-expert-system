use std::process;

use clap::{arg, command};

mod input;
use input::Input;
mod engine;

fn main() {
    let matches = command!()
        .arg(
            arg!(<input_file> "Path to the input file")
                .takes_value(true)
                .multiple_values(false)
                .forbid_empty_values(true),
        )
        .get_matches();

    let input_file_path = matches.value_of("input_file").unwrap();
    println!("Input file: {:#?}", input_file_path);
    let input = Input::new(input_file_path).unwrap_or_else(|error| {
        eprintln!("Failed to parse input file: {}", error);
        process::exit(1);
    });
    println!("{:#?}", input);
}
