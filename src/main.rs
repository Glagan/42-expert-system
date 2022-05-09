use std::process;

use clap::{arg, command};

use colored::Colorize;
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
    input.show_rules();
    input.show_initial_facts();

    // Create an inference engine for the Input and resolve all queries
    for query in input.queries.clone().iter() {
        // let mut path: Vec<String> = vec![];
        let result = input
            .facts
            .get_mut(query)
            .unwrap()
            .as_ref()
            .borrow_mut()
            .resolve();
        if let Ok(result) = result {
            println!(
                "{}{} {}",
                "?".normal().on_purple(),
                format!("{}", query).bright_cyan().on_purple(),
                if result {
                    format!("{}", "true".cyan())
                } else {
                    format!("{}", "false".yellow())
                }
            );
        } else {
            println!(
                "{}{} {}",
                "?".normal().on_purple(),
                format!("{}", query).bright_cyan().on_purple(),
                result.unwrap_err().to_string().red().on_yellow()
            );
        }
    }
}
