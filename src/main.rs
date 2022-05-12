use clap::{arg, command};
use colored::Colorize;

pub mod input;
use input::Input;
pub mod node;

fn main() {
    let matches = command!()
        .arg(
            arg!(<file_paths> "Path to the input file(s)")
                .takes_value(true)
                .multiple_values(true)
                .forbid_empty_values(true),
        )
        .arg(
            arg!(-v --visualize ... "Visualize the path to resolve a query")
                .required(false)
                .takes_value(false)
                .multiple_values(false),
        )
        .get_matches();

    // Parse input and convert the rules to a tree
    let file_paths: Vec<_> = matches.values_of("file_paths").unwrap().collect();
    for file_path in file_paths {
        println!("{}", format!("#  {}", file_path).black().on_white());
        let mut input = Input::new();
        let load_result = input.load_file(file_path);
        if load_result.is_err() {
            eprintln!("Failed to parse input file: {}", load_result.unwrap_err());
            continue;
        }
        input.show_warnings();
        input.show_rules();
        input.show_initial_facts();

        // Create an inference engine for the Input and resolve all queries
        for query in input.queries.clone().iter() {
            let mut path: Vec<String> = vec![];
            // let mut path: Vec<String> = vec![];
            let result = input
                .facts
                .get(query)
                .unwrap()
                .as_ref()
                .borrow()
                .resolve(&mut path);
            if matches.is_present("visualize") {
                path.iter()
                    .map(|path| println!("{}  {}", "?".purple().on_black(), path))
                    .for_each(drop);
            }
            if let Ok(result) = result {
                println!(
                    "{}{} {}",
                    "?".normal().on_purple(),
                    format!("{}", query).bright_cyan().on_purple(),
                    if result.is_true() {
                        format!("{}", "true".cyan())
                    } else if result.is_ambiguous() {
                        format!("{}", "ambiguous".purple())
                    } else {
                        format!("{}", "false".yellow())
                    }
                );
            } else {
                println!(
                    "{}{} {}",
                    "?".normal().on_purple(),
                    format!("{}", query).bright_cyan().on_purple(),
                    result.unwrap_err().to_string().red()
                );
            }
        }
    }
}
