use clap::{arg, command};
use colored::Colorize;
use std::io::{self, Write};
use text_io::read;

pub mod input;
use input::Input;
pub mod node;

fn interactive_line(line: &str) {
    println!("{}  {}", "$".yellow().on_black(), line);
}

fn interactive_input() {
    print!("{}  ", "$".yellow().on_black());
    io::stdout().flush().unwrap();
}

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
        .arg(
            arg!(-i --interactive ... "Update initial facts and queries in the shell")
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
        let mut do_loop = true;
        while do_loop {
            for query in input.queries.clone().iter() {
                // Resolve the current state
                let mut path: Vec<String> = vec![];
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

            // Interactive mode to update rules, facts and queries
            if matches.is_present("interactive") {
                let mut search_command = true;
                while search_command {
                    interactive_line(&"[e]xec [r]ule [?]query [f]act [n]ext [h]elp [q]uit");
                    interactive_input();
                    let command: String = read!("{}");
                    // Resolve all of the current queries
                    if command == "e" || command == "exec" {
                        search_command = false;
                    }
                    // Show the current rules, initial facts and queries
                    else if command == "s" || command == "show" {
                        input.show_rules();
                        input.show_initial_facts();
                        input.show_queries();
                    }
                    // Add a rule
                    else if command == "r" || command == "rule" {
                        interactive_line(&"Add a rule, example: `A => B`");
                        interactive_input();
                        let rule: String = read!("{}");
                        let result = input.parse_rule(&rule.trim());
                        if result.is_err() {
                            interactive_line(&result.unwrap_err().to_string());
                        } else {
                            input.show_warnings();
                            input.show_rules();
                        }
                    }
                    // Set *all* of the initial facts
                    else if command == "f" || command == "facts" {
                        interactive_line(&"Set all initial facts, example: `ABC`");
                        interactive_input();
                        let facts: String = read!("{}");
                        let result = input.reparse_initial_facts(&format!("={}", facts.trim()));
                        if result.is_err() {
                            interactive_line(&result.unwrap_err().to_string());
                        } else {
                            input.show_warnings();
                            input.show_initial_facts();
                        }
                    }
                    // Set *all* of the queries
                    else if command == "?" || command == "queries" {
                        interactive_line(&"Set all queries, example: `ABC`");
                        interactive_input();
                        let queries: String = read!("{}\n");
                        let result = input.reparse_queries(&format!("?{}", queries.trim()));
                        if result.is_err() {
                            interactive_line(&result.unwrap_err().to_string());
                        } else {
                            input.show_warnings();
                            input.show_queries();
                        }
                    }
                    // Next file
                    else if command == "n" || command == "next" {
                        do_loop = false;
                        search_command = false;
                    }
                    // Quit the program
                    else if command == "q" || command == "quit" || command == "exit" {
                        return;
                    }
                    // Print the help, it's hard
                    else if command == "h" || command == "help" {
                        interactive_line(&"e, exec\tResolve the current queries");
                        interactive_line(
                            &"s, show\tShow the current rules, initial facts and queries",
                        );
                        interactive_line(&"r, rule\tAdd a rule");
                        interactive_line(&"f, facts\tSet the initial facts");
                        interactive_line(&"?, queries\tSet the queries to resolve");
                        interactive_line(&"n, next\tGo to the next file");
                        interactive_line(&"h, help\tPrint this help");
                        interactive_line(&"q, quit\tQuit the program");
                    }
                    if command.is_empty() {
                        println!();
                    }
                }
            } else {
                do_loop = false
            }
        }
    }
}
