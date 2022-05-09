use std::process;

use clap::{arg, command};

mod input;
use input::Input;
mod node;

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
    let mut input = Input::new();
    input.load_file(input_file_path).unwrap_or_else(|error| {
        eprintln!("Failed to parse input file: {}", error);
        process::exit(1);
    });
    input.show_warnings();
    input.show_initial_facts();

    // Create an inference engine for the Input and resolve all queries
    /*let engine = Engine::new(input);
    for query in &engine.input.queries {
        let mut path: Vec<String> = vec![];
        let result = engine.resolve_query(query, &mut path);
        if let Ok(result) = result {
            println!(
                "{}{} {}",
                "?".normal().on_purple(),
                format!("{}", query).bright_cyan().on_purple(),
                result
            );
        } else {
            println!(
                "{}{} {}",
                "?".normal().on_purple(),
                format!("{}", query).bright_cyan().on_purple(),
                format!("{}", result.unwrap_err()).red().on_yellow()
            );
        }
    }*/
}
