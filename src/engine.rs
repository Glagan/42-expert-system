use crate::{
    input::Input,
    symbol::{Operator, Symbol},
};
use colored::Colorize;
use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc, vec};

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
    memoized_queries: HashMap<char, QueryResult>,
    _resolving_rules: Vec<usize>,
}

impl Engine {
    pub fn new(input: Input) -> Engine {
        Engine {
            input,
            memoized_queries: HashMap::new(),
            _resolving_rules: vec![],
        }
    }

    fn get_rules(&self, query: &char) -> Vec<Symbol> {
        let mut rules: Vec<Symbol> = vec![];
        for rule in self.input.rules.iter() {
            if rule.imply_symbol(query) {
                rules.push(rule.clone());
            }
        }
        rules
    }

    fn resolve_unit(&self, unit: &char) -> Result<QueryResult, String> {
        if self.input.initial_facts.contains_key(unit) {
            return Ok(QueryResult {
                value: true,
                ambiguous: false,
                ambiguous_symbols: vec![],
            });
        }
        if self.memoized_queries.contains_key(unit) {
            let memoized_result = self.memoized_queries.get(unit).unwrap();
            return Ok(QueryResult {
                value: memoized_result.value,
                ambiguous: memoized_result.ambiguous,
                ambiguous_symbols: memoized_result.ambiguous_symbols.clone(),
            });
        }
        return self.resolve_query(unit);
    }

    fn resolve_symbol(&self, symbol: Rc<RefCell<Symbol>>) -> Result<QueryResult, String> {
        // Resolve simple rules with a value
        if let Some(value) = &RefCell::borrow(&symbol).value {
            let mut result = self.resolve_unit(value)?;
            if RefCell::borrow(&symbol).operator_eq(&Operator::Not) {
                result.value = !result.value
            }
            return Ok(result);
        }
        // Resolve Symbol with operator
        if let Some(op) = &RefCell::borrow(&symbol).operator {
            if !RefCell::borrow(&symbol).has_left() {
                return Err("Missing left Symbol in operation".to_string());
            }
            if !RefCell::borrow(&symbol).has_right() {
                return Err("Missing right Symbol in operation".to_string());
            }
            let symbol = symbol.borrow();
            let left = symbol.left.as_ref().unwrap();
            let left_result = self.resolve_symbol(Rc::clone(&left))?;
            let right = symbol.right.as_ref().unwrap();
            let right_result = self.resolve_symbol(Rc::clone(&right))?;
            return match op {
                Operator::And => {
                    if left_result.value && right_result.value {
                        return Ok(QueryResult {
                            value: true,
                            ambiguous: false, // Can't be ambiguous
                            ambiguous_symbols: vec![],
                        });
                    }
                    return Ok(QueryResult {
                        value: false,
                        ambiguous: left_result.ambiguous || right_result.ambiguous,
                        ambiguous_symbols: [
                            left_result.ambiguous_symbols,
                            right_result.ambiguous_symbols,
                        ]
                        .concat(),
                    });
                }
                Operator::Or => {
                    if left_result.value || right_result.value {
                        return Ok(QueryResult {
                            value: true,
                            ambiguous: false, // Can't be ambiguous
                            ambiguous_symbols: vec![],
                        });
                    }
                    return Ok(QueryResult {
                        value: false,
                        ambiguous: left_result.ambiguous || right_result.ambiguous,
                        ambiguous_symbols: [
                            left_result.ambiguous_symbols,
                            right_result.ambiguous_symbols,
                        ]
                        .concat(),
                    });
                }
                Operator::Xor => {
                    if (left_result.value && !right_result.value)
                        || (!left_result.value && right_result.value)
                    {
                        return Ok(QueryResult {
                            value: true,
                            ambiguous: false, // Can't be ambiguous
                            ambiguous_symbols: vec![],
                        });
                    }
                    return Ok(QueryResult {
                        value: false,
                        ambiguous: left_result.ambiguous || right_result.ambiguous,
                        ambiguous_symbols: [
                            left_result.ambiguous_symbols,
                            right_result.ambiguous_symbols,
                        ]
                        .concat(),
                    });
                }
                _ => Err(format!("Invalid operator {:?} in Symbol", op)),
            };
        }
        // Resolve nested block
        if let Some(left) = &RefCell::borrow(&symbol).left {
            return self.resolve_symbol(Rc::clone(left));
        }
        Err(format!("Invalid Symbol {:?} in rule", symbol))
    }

    fn resolve_rule(&self, rule: &Symbol) -> Result<QueryResult, String> {
        // A rule should have a left symbol
        // -- except if it's only a value (with an optionnal Operator::Not)
        if let Some(symbol) = &rule.left {
            if let Some(value) = &RefCell::borrow(symbol).value {
                // Simple rule
                let mut result = self.resolve_unit(value)?;
                if rule.operator_eq(&Operator::Not) {
                    result.value = !result.value;
                }
                return Ok(result);
            } else {
                // Nested rule
                // TODO memoize rule by it's value of it's right side
                return self.resolve_symbol(Rc::clone(symbol));
            }
        }
        Err("Empty rule condition".to_string())
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
        let mut best_query_result: Option<QueryResult> = None;
        for rule in rules.iter() {
            // TODO check if rule is currently being resolved to avoid infinite loops ?
            let rule_result = self.resolve_rule(rule)?;
            if rule_result.value {
                return Ok(rule_result);
            }
            // Set best_query_result or update it to the less ambiguous one
            if let Some(best_query_result) = &mut best_query_result {
                if !rule_result.ambiguous && best_query_result.ambiguous {
                    best_query_result.value = rule_result.value;
                    best_query_result.ambiguous = rule_result.ambiguous;
                    best_query_result.ambiguous_symbols = rule_result.ambiguous_symbols;
                }
            } else {
                best_query_result = Some(rule_result);
            }
        }

        // Return the best rule (either ambiguous or false)
        if best_query_result.is_none() {
            return Ok(QueryResult {
                value: false,
                ambiguous: false,
                ambiguous_symbols: vec![],
            });
        }
        Ok(best_query_result.unwrap())
    }
}
