use crate::engine::{Operator, Symbol};
use nom::{
    branch::alt,
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
        return Err("Invalid characters in Symbol".to_string());
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

fn parse_symbol_block(string: &str) -> Result<Rc<RefCell<Symbol>>, String> {
    // Initial state
    let mut current_symbol = Rc::new(RefCell::new(Symbol::new()));
    println!("parsing <{}>", string);

    // Parse
    for c in string.chars() {
        if c == '(' {
            // Open context on available symbol side
            if !current_symbol.borrow().has_left() {
                let new_symbol = Rc::new(RefCell::new(Symbol::new()));
                current_symbol.borrow_mut().left = Some(Rc::clone(&new_symbol));
                new_symbol.borrow_mut().parent = Some(Rc::clone(&current_symbol));
                current_symbol = new_symbol;
            } else if !current_symbol.borrow().has_right() {
                let new_symbol = Rc::new(RefCell::new(Symbol::new()));
                current_symbol.borrow_mut().right = Some(Rc::clone(&new_symbol));
                new_symbol.borrow_mut().parent = Some(Rc::clone(&current_symbol));
                current_symbol = new_symbol;
            } else {
                return Err("Invalid opening context on empty symbol".to_string());
            }
        } else if c == ')' {
            // Close context by poping the last upper symbol
            if !current_symbol.borrow().has_parent() {
                return Err("Closing context on root symbol".to_string());
            }
            if current_symbol.borrow().has_left()
                && !current_symbol.borrow().has_right()
                && current_symbol.borrow().has_operator()
            {
                return Err("Closing context on incomplete symbol".to_string());
            }
            let parent = Rc::clone(current_symbol.borrow().parent.as_ref().unwrap());
            current_symbol = parent;
        } else if c == '!' {
            // Open context on an available symbol side
            if !current_symbol.borrow().has_left() {
                let new_symbol = Rc::new(RefCell::new(Symbol::operator(Operator::Not)));
                current_symbol.borrow_mut().left = Some(Rc::clone(&new_symbol));
                new_symbol.borrow_mut().parent = Some(Rc::clone(&current_symbol));
                current_symbol = new_symbol;
            } else if !current_symbol.borrow().has_right() {
                if !current_symbol.borrow().has_operator() {
                    return Err("Invalid NOT operator on incomplete symbol".to_string());
                }
                let new_symbol = Rc::new(RefCell::new(Symbol::operator(Operator::Not)));
                current_symbol.borrow_mut().right = Some(Rc::clone(&new_symbol));
                new_symbol.borrow_mut().parent = Some(Rc::clone(&current_symbol));
                current_symbol = new_symbol;
            } else {
                return Err("Invalid NOT operator on empty symbol".to_string());
            }
        } else if c == '+' || c == '|' || c == '^' {
            // Set the operator of the current symbol or create a new one
            if current_symbol.borrow().has_operator() {
                if current_symbol.borrow().has_left() && current_symbol.borrow().has_right() {
                    // If the current symbol is full create new symbol with the nested previous
                    let new_symbol = Rc::new(RefCell::new(Symbol::new()));
                    new_symbol.borrow_mut().left = Some(Rc::clone(&current_symbol));
                    new_symbol.borrow_mut().operator = Symbol::match_operator(c);
                    new_symbol.borrow_mut().parent =
                        Some(Rc::clone(current_symbol.borrow().parent.as_ref().unwrap()));
                    current_symbol.borrow_mut().parent = Some(Rc::clone(&new_symbol));
                    current_symbol = new_symbol;
                    // Update last upper symbol left or right which was for the current symbol
                    if current_symbol.borrow().has_parent() {
                        let parent = Rc::clone(current_symbol.borrow().parent.as_ref().unwrap());
                        if parent.borrow().has_right() {
                            parent.borrow_mut().right = Some(Rc::clone(&current_symbol));
                        } else if parent.borrow().has_left() {
                            parent.borrow_mut().left = Some(Rc::clone(&current_symbol));
                        } else {
                            return Err("Error opening a new nested symbol on a full operator with an empty context".to_string());
                        }
                    }
                } else {
                    return Err("Operator on already set symbol".to_string());
                }
            }
            current_symbol.borrow_mut().operator = Symbol::match_operator(c);
        } else if !current_symbol.borrow().has_left() {
            // If the current symbol has a Operator::Not
            // -- set the value of the symbol
            // Else create a symbol with a value on an opened side
            if current_symbol.borrow().has_operator()
                && current_symbol.borrow().operator_eq(&Operator::Not)
            {
                println!("Adding symbol of left side {}", string);
                current_symbol.borrow_mut().value = Some(c);
                if current_symbol.borrow().has_parent() {
                    let parent = Rc::clone(current_symbol.borrow().parent.as_ref().unwrap());
                    current_symbol = parent;
                }
            } else {
                current_symbol.borrow_mut().left = Some(Rc::new(RefCell::new(Symbol::unit(c))));
            }
        } else if !current_symbol.borrow().has_right() {
            if !current_symbol.borrow().has_operator() {
                return Err("Missing operator between symbols".to_string());
            }
            current_symbol.borrow_mut().right = Some(Rc::new(RefCell::new(Symbol::unit(c))));
        } else {
            return Err("Extraneous symbol with no operators or block".to_string());
        }
    }

    // Return root symbol
    while current_symbol.borrow().has_parent() {
        let parent = Rc::clone(current_symbol.borrow().parent.as_ref().unwrap());
        current_symbol = parent;
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
        alt((take_until1("<=>"), take_until1("=>"))),
        value((), multispace0),
        alt((tag("<=>"), tag("=>"))),
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
        };
        let mut parsed_initial_facts = false;
        let mut parsed_queries = false;

        fn add_symbol(input: &mut Input, symbol: &Symbol) {
            if let Some(value) = &symbol.value {
                input.symbol_values.entry(*value).or_insert(false);
            }
            if let Some(left) = &symbol.left {
                add_symbol(input, &left.borrow());
            }
            if let Some(right) = &symbol.right {
                add_symbol(input, &right.borrow());
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
                    // TODO warning for duplicate queries ?
                    input.queries = queries;
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
                        parent: None,
                        value: None,
                        left: Some(parse_symbol_block(&left)?),
                        right: Some(parse_symbol_block(&right)?),
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
                        // TODO warning for duplicate initial facts ?
                        input.initial_facts.entry(symbol).or_insert(true);
                        *input.symbol_values.entry(symbol).or_insert(true) = true;
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

    fn check(&self) -> Result<(), String> {
        if self.queries.is_empty() {
            return Err("Queries can't be empty".to_string());
        }
        if !self.queries.iter().all(char::is_ascii_uppercase) {
            return Err("Queries can only be uppercase letters".to_string());
        }
        if !self.initial_facts.keys().all(char::is_ascii_uppercase) {
            return Err("Initial facts can only be uppercase letters".to_string());
        }
        // Check initial fact symbols exist in list of all symbols => warning ?
        // Check the query symbols exist in list of all symbols => warning ?
        // Check for ambiguous symbols ?
        Ok(())
    }

    pub fn new(file_path: &str) -> Result<Input, String> {
        let content = fs::read_to_string(file_path);
        if let Err(e) = content {
            return Err(e.to_string());
        }

        let input = Input::parse_content(&content.unwrap())?;
        input.check()?;
        Ok(input)
    }
}
