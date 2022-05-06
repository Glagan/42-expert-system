use crate::{input::Input, symbol::Symbol};
use colored::Colorize;
use std::fmt;

#[derive(Debug)]
#[allow(dead_code)]
pub enum ResultStatus {
    Success,
    Ambiguous,
    Failure,
}

#[derive(Debug)]
pub struct QueryResult {
    pub status: ResultStatus,
    pub value: bool,
    pub ambiguous_symbols: Vec<char>,
}

impl fmt::Display for QueryResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.status {
            ResultStatus::Success => {
                if self.value {
                    write!(f, "{}", "true".normal().on_green())
                } else {
                    write!(f, "{}", "false".normal().on_red())
                }
            }
            ResultStatus::Ambiguous => write!(
                f,
                "{} because of symbols {}",
                "ambiguous".normal().on_yellow(),
                self.ambiguous_symbols
                    .iter()
                    .map(|c| format!("{}", c))
                    .collect::<Vec<String>>()
                    .join(&", ")
            ),
            ResultStatus::Failure => write!(f, "{} to resolve value", "failed".normal().on_red(),),
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
                status: ResultStatus::Success,
                value: true,
                ambiguous_symbols: vec![],
            });
        }

        // Get the list of rules that can imply the query
        let rules = self.get_rules(query);
        println!("Rules {:#?}", rules);

        Ok(QueryResult {
            status: ResultStatus::Success,
            value: false,
            ambiguous_symbols: vec![],
        })
    }
}
