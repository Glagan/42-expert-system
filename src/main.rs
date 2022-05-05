use std::process;

use clap::{arg, command};

mod engine;
mod input;
use engine::Engine;
use input::Input;
mod symbol;

fn main() {
    let matches = command!()
        .arg(
            arg!(<input_file> "Path to the input file")
                .takes_value(true)
                .multiple_values(false)
                .forbid_empty_values(true),
        )
        .get_matches();

    // Parse input and convert the rules to a tree
    let input_file_path = matches.value_of("input_file").unwrap();
    let input = Input::new(input_file_path).unwrap_or_else(|error| {
        eprintln!("Failed to parse input file: {}", error);
        process::exit(1);
    });

    // Create an inference engine for the Input and resolve all queries
    let engine = Engine { input: input };
    for query in &engine.input.queries {
        let result = engine.resolve_query(query);
        println!("Query [{}] result: {:#?}", query, result);
    }
}
