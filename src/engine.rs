use crate::{input::Input, symbol::Symbol};
use colored::Colorize;
use std::{cell::RefCell, fmt};

#[derive(Debug)]
pub struct QueryResult {
    pub value: bool,
    pub ambiguous: bool,
    pub ambiguous_symbols: Vec<char>,
}

impl fmt::Display for QueryResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.ambiguous {
            if !self.ambiguous_symbols.is_empty() {
                write!(
                    f,
                    "{} because of symbols {}",
                    "ambiguous".normal().on_yellow(),
                    self.ambiguous_symbols
                        .iter()
                        .map(|c| format!("{}", c))
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            } else {
                write!(f, "{}", "ambiguous".normal().on_yellow())
            }
        } else if self.value {
            write!(f, "{}", "true".normal().on_green())
        } else {
            write!(f, "{}", "false".normal().on_red())
        }
    }
}

#[derive(Debug)]
pub struct Engine {
    pub input: Input,
}

impl Engine {
    fn get_rules(&self, query: &char) -> Vec<Symbol> {
        let mut rules: Vec<Symbol> = vec![];
        for rule in self.input.rules.iter() {
            if rule.imply_symbol(query) {
                rules.push(rule.clone());
            }
        }
        rules
    }

    pub fn resolve_query(&self, query: &char) -> Result<QueryResult, String> {
        // Resolve initial facts queries
        if self.input.initial_facts.contains_key(query)
            && *self.input.initial_facts.get(query).unwrap()
        {
            return Ok(QueryResult {
                value: true,
                ambiguous: false,
                ambiguous_symbols: vec![],
            });
        }

        // Get the list of rules that can imply the query
        let rules = self.get_rules(query);
        let mut best_query_result = QueryResult {
            value: false,
            ambiguous: false,
            ambiguous_symbols: vec![],
        };
        for rule in rules.iter() {
            if let Some(symbol) = &rule.left {
                if RefCell::borrow(symbol).has_value() {
                    // Simple rule
                    best_query_result.value = self
                        .input
                        .initial_facts
                        .contains_key(&RefCell::borrow(symbol).value.unwrap());
                    if best_query_result.value {
                        return Ok(best_query_result);
                    } else {
                        // TODO Resolve symbol value with resolve_query, see below vvv
                    }
                }
            } else {
                // TODO Travel to a leaf, set the initial state of QueryResult by calculating it
                // TODO > A symbol value is true if inside the initial facts
                // TODO > Else call resolve_query recursively for the symbol value (and memoize it' value ?)
                // TODO Add a `visited` flag to avoid infinite rules (return an error)
                // TODO On operator, apply a function on the "leaf" of both sides
            }
            if rule.is_ambiguous() {
                best_query_result.ambiguous = true;
            }
        }

        // Return the best rule (either ambiguous or false)
        Ok(best_query_result)
    }
}
