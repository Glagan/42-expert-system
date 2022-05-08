use crate::symbol::{Operator, Symbol};
use colored::Colorize;
use nom::{
    bytes::complete::{tag, take_until1, take_while, take_while1},
    character::complete::{anychar, multispace0},
    combinator::{opt, value},
    multi::many0,
    sequence::tuple,
    IResult,
};
use std::{cell::RefCell, collections::HashMap, fs, rc::Rc};

#[derive(Debug)]
pub struct Input {
    pub symbols: Vec<char>,
    pub rules: Vec<Symbol>,
    pub symbol_values: HashMap<char, bool>,
    pub initial_facts: HashMap<char, bool>,
    pub queries: Vec<char>,
    pub warnings: Vec<String>,
}

fn remove_whitespaces(string: &str) -> String {
    string.chars().filter(|c| !c.is_whitespace()).collect()
}

fn is_only_valid_characters(string: &str) -> Result<(), String> {
    if !string.chars().all(|c| {
        char::is_ascii_uppercase(&c)
            || c == '!'
            || c == '+'
            || c == '|'
            || c == '^'
            || c == '('
            || c == ')'
    }) {
        return Err("Invalid characters in Symbol, only uppercase letters and operators are allowed".to_string());
    }
    Ok(())
}

fn prepare_root_symbol(left: &str, right: &str) -> Result<(String, String), String> {
    let left = remove_whitespaces(left);
    let right = remove_whitespaces(right);
    is_only_valid_characters(&left)?;
    is_only_valid_characters(&right)?;
    Ok((left, right))
}

fn parse_symbol_block(string: &str, is_conclusion: bool) -> Result<Rc<RefCell<Symbol>>, String> {
    // Initial state
    let mut opened_context = 0;
    let mut upper_symbols: Vec<Rc<RefCell<Symbol>>> = vec![];
    let mut current_symbol: Rc<RefCell<Symbol>> = Rc::new(RefCell::new(Symbol::new()));

    // Parse
    for (i, c) in string.chars().enumerate() {
        if c == '(' {
            // Open context on available symbol side
            opened_context += 1;
            if !RefCell::borrow(&current_symbol).has_left() {
                let new_symbol = Rc::new(RefCell::new(Symbol::new()));
                current_symbol.borrow_mut().left = Some(Rc::clone(&new_symbol));
                upper_symbols.push(Rc::clone(&current_symbol));
                current_symbol = new_symbol;
            } else if !RefCell::borrow(&current_symbol).has_right() {
                if !RefCell::borrow(&current_symbol).has_operator() {
                    return Err(format!(
                        "Opening context on a incomplete symbol in block `{}` column {}",
                        string,
                        i + 1
                    ));
                }
                let new_symbol = Rc::new(RefCell::new(Symbol::new()));
                current_symbol.borrow_mut().right = Some(Rc::clone(&new_symbol));
                upper_symbols.push(Rc::clone(&current_symbol));
                current_symbol = new_symbol;
            } else {
                return Err(format!(
                    "Invalid opening context on empty symbol in block `{}` column {}",
                    string,
                    i + 1
                ));
            }
        } else if c == ')' {
            // Close context by poping the last upper symbol
            if upper_symbols.is_empty() {
                return Err(format!(
                    "Closing context on root symbol in block `{}` column {}",
                    string,
                    i + 1
                ));
            }
            if RefCell::borrow(&current_symbol).has_left()
                && !RefCell::borrow(&current_symbol).has_right()
                && RefCell::borrow(&current_symbol).has_operator()
            {
                return Err(format!(
                    "Closing context on incomplete symbol in block `{}` column {}",
                    string,
                    i + 1
                ));
            }
            if !RefCell::borrow(&current_symbol).has_left()
                && !RefCell::borrow(&current_symbol).has_right()
                && !RefCell::borrow(&current_symbol).has_value()
            {
                return Err(format!(
                    "Unused context in block `{}` column {}",
                    string,
                    i + 1
                ));
            }
            opened_context -= 1;
            current_symbol = upper_symbols.pop().unwrap();
        } else if c == '!' {
            // Open context on an available symbol side
            if !RefCell::borrow(&current_symbol).has_left() {
                let new_symbol = Rc::new(RefCell::new(Symbol::operator(Operator::Not)));
                current_symbol.borrow_mut().left = Some(Rc::clone(&new_symbol));
                upper_symbols.push(Rc::clone(&current_symbol));
                current_symbol = new_symbol;
            } else if !RefCell::borrow(&current_symbol).has_right() {
                if !RefCell::borrow(&current_symbol).has_operator() {
                    return Err(format!(
                        "Invalid NOT operator on incomplete symbol in block `{}` column {}",
                        string,
                        i + 1
                    ));
                }
                let new_symbol = Rc::new(RefCell::new(Symbol::operator(Operator::Not)));
                current_symbol.borrow_mut().right = Some(Rc::clone(&new_symbol));
                upper_symbols.push(Rc::clone(&current_symbol));
                current_symbol = new_symbol;
            } else {
                return Err(format!(
                    "Invalid NOT operator on empty symbol in block `{}` column {}",
                    string,
                    i + 1
                ));
            }
        } else if c == '+' || c == '|' || c == '^' {
            if is_conclusion && (c == '|' || c == '^') {
                return Err(format!("Unallowed Operator in conclusion `{}` column {}", string, i + 1));
            }
            // Set the operator of the current symbol or create a new one
            if RefCell::borrow(&current_symbol).has_operator() {
                if RefCell::borrow(&current_symbol).has_left() && RefCell::borrow(&current_symbol).has_right() { 
                    // If the current symbol is full create new symbol with the nested previous
                    // -- Check for operator priority
                    let new_operator = Symbol::match_operator(c).unwrap();
                    // -- If an operator has "more" priority
                    // -- the right side of the current symbol is inserted on a new symbol with the new operator
                    // -- and the new nested symbol is set as the current symbol
                    if new_operator < RefCell::borrow(&current_symbol).operator.unwrap() { 
                        let new_symbol = Rc::new(RefCell::new(Symbol::new()));
                        new_symbol.borrow_mut().left = Some(Rc::clone(&RefCell::borrow(&current_symbol).right.as_ref().unwrap()));
                        new_symbol.borrow_mut().operator = Some(new_operator);
                        current_symbol.borrow_mut().right = Some(Rc::clone(&new_symbol));
                        upper_symbols.push(Rc::clone(&current_symbol));
                        current_symbol = new_symbol;
                    }
                    // -- Else a new symbol is created and the previous one is added on the left side of the new one
                    else {
                        let new_symbol = Rc::new(RefCell::new(Symbol::new()));
                        new_symbol.borrow_mut().left = Some(Rc::clone(&current_symbol));
                        new_symbol.borrow_mut().operator = Some(new_operator);
                        current_symbol = new_symbol;
                        // Update last upper symbol left or right which was for the current symbol
                        if !upper_symbols.is_empty() {
                            let last = upper_symbols.last().unwrap();
                            if RefCell::borrow(last).has_right() {
                                last.borrow_mut().right = Some(Rc::clone(&current_symbol));
                            } else if RefCell::borrow(last).has_left() {
                                last.borrow_mut().left = Some(Rc::clone(&current_symbol));
                            } else {
                                return Err(format!("Opening a new nested symbol on a full operator with an empty context in block `{}` column {}", string, i + 1));
                            }
                        }
                    }
                } else {
                    return Err(format!(
                        "Operator on already set symbol in block `{}` column {}",
                        string,
                        i + 1
                    ));
                }
            } else {
                if !RefCell::borrow(&current_symbol).has_left() {
                    if RefCell::borrow(&current_symbol).has_value() {
                        let value =RefCell::borrow(&current_symbol).value.unwrap();
                        current_symbol.borrow_mut().left = Some(Rc::new(RefCell::new(Symbol::unit(value))));
                        current_symbol.borrow_mut().value= None;            
                    } else {
                        return Err(format!(
                            "Adding operator to empty symbol in block `{}` column {}",
                            string,
                            i + 1
                        ));
                    }
                }
                current_symbol.borrow_mut().operator = Symbol::match_operator(c);
            }
        } else if !RefCell::borrow(&current_symbol).has_left() {
            // If the current symbol has a Operator::Not
            // -- set the value of the symbol
            // Else create a symbol with a value on an opened side
            if RefCell::borrow(&current_symbol).operator_eq(&Operator::Not) {
                current_symbol.borrow_mut().value = Some(c);
                if !upper_symbols.is_empty() {
                    let last = upper_symbols.pop().unwrap();
                    current_symbol = Rc::clone(&last);
                }
            } else if !RefCell::borrow(&current_symbol).has_value() &&
                    !RefCell::borrow(&current_symbol).has_left() &&
                    !RefCell::borrow(&current_symbol).has_right() &&
                    !RefCell::borrow(&current_symbol).has_operator() {
                current_symbol.borrow_mut().value = Some(c);
            } else {
                current_symbol.borrow_mut().left = Some(Rc::new(RefCell::new(Symbol::unit(c))));
            }
        } else if !RefCell::borrow(&current_symbol).has_right() {
            if !RefCell::borrow(&current_symbol).has_operator() {
                return Err(format!(
                    "Missing operator between symbols in block `{}` column {}",
                    string,
                    i + 1
                ));
            }
            current_symbol.borrow_mut().right = Some(Rc::new(RefCell::new(Symbol::unit(c))));
        } else {
            return Err(format!(
                "Extraneous symbol with no operators or block in block `{}` column {}",
                string,
                i + 1
            ));
        }
    }

    // Check incomplete current_symbol
    let has_upper_symbols = upper_symbols.is_empty();
    if 
        // Missing right side on root symbol with operator
        (!has_upper_symbols
            && RefCell::borrow(&current_symbol).has_left()
            && !RefCell::borrow(&current_symbol).has_operator())
        || 
        // Empty symbol
        (!has_upper_symbols
            && !RefCell::borrow(&current_symbol).has_value()
            && !RefCell::borrow(&current_symbol).has_left()
            && !RefCell::borrow(&current_symbol).has_right())
        || 
        // Nested symbol has missing right side -- missing right side on root is allowed
        (has_upper_symbols
            && RefCell::borrow(&current_symbol).has_left()
            && !RefCell::borrow(&current_symbol).has_right()
            && RefCell::borrow(&current_symbol).has_operator())
    {
        return Err(format!("Incomplete symbol in block `{}`", string));
    }

    // Check contexts
    if opened_context != 0 {
        return Err(format!("Unclosed context in block `{}`", string));
    }

    // Unshift the root symbol
    if !upper_symbols.is_empty() {
        current_symbol = upper_symbols.swap_remove(0);
    }

    Ok(current_symbol)
}

// Separate rule in two blocks and parse the two blocks individually later
//               -symmetrical-
//                v         v
// symbol: (\s|!|\()*\w(\s|\()*
//           ------symmetrical--------
//           |         operator      |
//           v         vvvvv         v
// block: !*\(*{symbol}[+|^]{symbol}\)*
// regex: ^\s*({symbol}|{block})\s*(<=>|=>)\s*({symbol}|{block})\s*(?:#.+)?$
fn rule(i: &str) -> IResult<&str, (&str, &str, &str)> {
    let (input, (_, left, _, op, _, right, _, _)) = tuple((
        value((), multispace0),
        // alt((take_until1("<=>"), take_until1("=>"))),
        take_until1("=>"),
        value((), multispace0),
        // alt((tag("<=>"), tag("=>"))),
        tag("=>"),
        value((), multispace0),
        take_while1(|c| c != '#'),
        // Ignore comments
        value((), multispace0),
        opt(tuple((tag("#"), many0(anychar)))),
    ))(i)?;
    Ok((input, (left, op, right)))
}

// regex: ^=(\w)*\s*(?:#.+)?$
fn initial_facts(i: &str) -> IResult<&str, Vec<char>> {
    let (input, (_, symbols, _, _)) = tuple((
        tag("="),
        take_while(|c| c != '#'),
        // Ignore comments
        value((), multispace0),
        opt(tuple((tag("#"), many0(anychar)))),
    ))(i)?;

    let symbols = Vec::from_iter(symbols.chars().filter(|c| c != &' '));
    Ok((input, symbols))
}

// regex: ^\?(\w)+\s*(?:#.+)?$
fn queries(i: &str) -> IResult<&str, Vec<char>> {
    let (input, (_, symbols, _, _)) = tuple((
        tag("?"),
        take_while1(|c| c != '#'),
        // Ignore comments
        value((), multispace0),
        opt(tuple((tag("#"), many0(anychar)))),
    ))(i)?;

    let symbols = Vec::from_iter(symbols.chars().filter(|c| c != &' '));
    Ok((input, symbols))
}

impl Input {
    fn parse_content(content: &str) -> Result<Input, String> {
        let mut input = Input {
            symbols: vec![],
            rules: vec![],
            symbol_values: HashMap::new(),
            initial_facts: HashMap::new(),
            queries: vec![],
            warnings: vec![],
        };
        let mut parsed_initial_facts = false;
        let mut parsed_queries = false;

        fn add_symbol(input: &mut Input, symbol: &Symbol) {
            if let Some(value) = &symbol.value {
                if !input.symbols.contains(value) {
                    input.symbols.push(*value);
                }
                input.symbol_values.entry(*value).or_insert(false);
            }
            if let Some(left) = &symbol.left {
                add_symbol(input, &RefCell::borrow(left));
            }
            if let Some(right) = &symbol.right {
                add_symbol(input, &RefCell::borrow(right));
            }
        }

        for (line_number, line) in content.lines().enumerate() {
            let line = line.trim();
            // Ignore empty lines and lines with only a comment
            if line.starts_with('#') || line.is_empty() {
                continue;
            }
            if parsed_queries {
                return Err(format!(
                    "queries should be the last line of a configuration file\nLine {} `{}`",
                    line_number, line
                ));
            }

            // Parse queries
            if parsed_initial_facts {
                let result = queries(line);
                if let Ok((_, queries)) = result {
                    // Check if each queries are not duplicate and exist in rules or initial facts
                    for query in queries.iter() {
                        if input.queries.contains(query) {
                            input.warnings.push(format!("Duplicate query for symbol {}", query));
                        } else {
                            input.queries.push(*query);
                        }
                        if !input.symbols.contains(query) {
                            input.warnings.push(format!("Query for missing symbol {}", query));
                        }
                    }
                    parsed_queries = true
                } else {
                    let error = result.unwrap_err();
                    return Err(format!("{}\nLine {} `{}`", error, line_number, line));
                }
            }
            // Parse rule (and initial facts on error to switch context)
            else {
                let result = rule(line);
                if let Ok((_, (left, op, right))) = result {
                    let (left, right) = prepare_root_symbol(left, right)?;
                    let rule = Symbol {
                        value: None,
                        left: Some(parse_symbol_block(&left, false)?),
                        right: Some(parse_symbol_block(&right, true)?),
                        operator: if op == "=>" {
                            Some(Operator::Implies)
                        } else {
                            Some(Operator::IfAndOnlyIf)
                        },
                    };
                    add_symbol(&mut input, &rule);
                    input.rules.push(rule)
                }
                // Parse initial facts if the rule parser doesn't match
                else {
                    let original_error = result.unwrap_err();
                    let result = initial_facts(line);
                    // If it's not the initial facts it's just an error
                    if result.is_err() {
                        return Err(format!(
                            "{}\nLine {} `{}`",
                            original_error, line_number, line
                        ));
                    }
                    // Else add them to the Input
                    let (_, initial_facts) = result.unwrap();
                    for symbol in initial_facts {
                    // Check if each initial facts are not duplicated
                        if input.initial_facts.contains_key(&symbol) {
                            input.warnings.push(format!("Duplicate initial fact for symbol {}", symbol));
                        } else {
                            input.initial_facts.entry(symbol).or_insert(true);
                            *input.symbol_values.entry(symbol).or_insert(true) = true;
                        }
                    }
                    parsed_initial_facts = true;
                }
            }
        }

        if !parsed_initial_facts {
            return Err("Missing initial facts".to_string());
        }
        if !parsed_queries {
            return Err("Missing queries".to_string());
        }
        Ok(input)
    }

    fn check(&mut self) -> Result<(), String> {
        if self.queries.is_empty() {
            return Err("Queries can't be empty".to_string());
        }
        if !self.queries.iter().all(char::is_ascii_uppercase) {
            return Err("Queries can only be uppercase letters".to_string());
        }
        if !self.initial_facts.keys().all(char::is_ascii_uppercase) {
            return Err("Initial facts can only be uppercase letters".to_string());
        }
        for rule in self.rules.iter() {
            if rule.is_infinite() {
                return Err(format!("Rule {} is infinite", rule));
            }
            if rule.is_ambiguous() {
                self.warnings.push(format!("Rule {} is ambiguous", rule));
            }
        }
        Ok(())
    }

    pub fn new(file_path: &str) -> Result<Input, String> {
        let content = fs::read_to_string(file_path);
        if let Err(e) = content {
            return Err(e.to_string());
        }

        let mut input = Input::parse_content(&content.unwrap())?;
        input.check()?;
        Ok(input)
    }

    pub fn show_warnings(&self) {
        for warning in self.warnings.iter() {
            println!("{} {}", "!".red().on_yellow(), warning.yellow());
        }
    }

    pub fn show_initial_facts(&self) {
        print!("{}  ", "=".normal().on_purple());
        if !self.initial_facts.is_empty() {
            for (symbol, _) in self.initial_facts.iter() {
                print!("{}", format!("{}", symbol).normal().on_green());
            }
        } else {
            print!("No initial facts");
        }
        println!();
    }
}

#[test]
fn handle_parsing_error_1() {
    let result = parse_symbol_block("(", false);
    assert!(result.is_err())
}

#[test]
fn handle_parsing_error_2() {
    let result = parse_symbol_block(")", false);
    assert!(result.is_err())
}

#[test]
fn handle_parsing_error_3() {
    let result = parse_symbol_block("!", false);
    assert!(result.is_err())
}

#[test]
fn handle_parsing_error_4() {
    let result = parse_symbol_block("+", false);
    assert!(result.is_err())
}

#[test]
fn handle_parsing_error_5() {
    let result = parse_symbol_block("A+", false);
    assert!(result.is_err())
}

#[test]
fn handle_parsing_error_6() {
    let result = parse_symbol_block("+A", false);
    assert!(result.is_err())
}

#[test]
fn handle_parsing_error_7() {
    let result = parse_symbol_block("()", false);
    assert!(result.is_err())
}

#[test]
fn handle_parsing_error_8() {
    let result = parse_symbol_block("(A+)", false);
    assert!(result.is_err())
}

#[test]
fn handle_parsing_error_9() {
    let result = parse_symbol_block("!()", false);
    assert!(result.is_err())
}

#[test]
fn handle_parsing_error_10() {
    // Any other characters than operators or symbols should already be removed when calling this function
    let result = parse_symbol_block("A | B", false);
    assert!(result.is_err())
}

#[test]
fn handle_parsing_error_11() {
    let result = parse_symbol_block("!(A+)", false);
    assert!(result.is_err())
}

#[test]
fn handle_parsing_success_1() {
    let result = parse_symbol_block("A", false);
    assert!(result.is_ok())
}

#[test]
fn handle_parsing_success_2() {
    let result = parse_symbol_block("A+B", false);
    assert!(result.is_ok())
}

#[test]
fn handle_parsing_success_3() {
    let result = parse_symbol_block("(A+B)^C", false);
    assert!(result.is_ok())
}

#[test]
fn handle_parsing_success_4() {
    let result = parse_symbol_block("A+(B+C)+D", false);
    assert!(result.is_ok())
}

#[test]
fn handle_parsing_success_5() {
    let result = parse_symbol_block("!A", false);
    assert!(result.is_ok())
}

#[test]
fn handle_parsing_success_6() {
    let result = parse_symbol_block("!(A)", false);
    assert!(result.is_ok())
}

#[test]
fn handle_parsing_success_7() {
    let result = parse_symbol_block("!(A+B)", false);
    assert!(result.is_ok())
}

#[test]
fn handle_parsing_success_8() {
    let result = parse_symbol_block("(F^G)|(T+I)", false);
    assert!(result.is_ok())
}
