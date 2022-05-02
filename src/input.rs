use crate::engine::{Operator, Symbol};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, anychar, char, multispace0},
    combinator::{opt, value},
    multi::{many0, many1, many1_count},
    sequence::{delimited, tuple},
    IResult,
};
use std::{collections::HashMap, fs};

#[derive(Debug)]
pub struct Input {
    pub symbols: Vec<String>,
    pub rules: Vec<Symbol>,
    pub symbol_values: HashMap<String, bool>,
    pub initial_facts: HashMap<String, bool>,
    pub queries: Vec<String>,
}

//               -symmetrical-
//                v         v
// symbol: (\s|!|\()*\w(\s|\()*
fn parse_symbol(i: &str) {
    tuple((
        many0(alt((multispace0, many0(char('!'))))),
        alt((
            delimited(
                alt((many1_count(char('(')))), // TODO: Symmetrical with close
                tuple((many0(char('!')), alpha1)),
                many1_count(char(')')),
            ),
            tuple((many0(char('!')), alpha1)),
        )),
        value((), multispace0),
    ));
}

//           ------symmetrical--------
//           |         operator      |
//           v         vvvvv         v
// block: !*\(*{symbol}[+|^]{symbol}\)*
fn parse_block(i: &str) {}

// regex: ^\s*({symbol}|{block})\s*(<=>|=>)\s*({symbol}|{block})\s*(?:#.+)?$
fn parse_rule(i: &str) -> IResult<&str, Symbol> {
    let mut final_block_parser = alt((tuple((tag("("), tag(")"))), tuple()));
    let (input, (_, left, _, op, _, right, _, _)) = tuple((
        value((), multispace0),
        many1(alpha1), // TODO Recursive block parser
        value((), multispace0),
        alt((tag("<=>"), tag("=>"))),
        value((), multispace0),
        many1(alpha1), // TODO Recursive block parser
        // Ignore comments
        value((), multispace0),
        opt(tuple((tag("#"), many0(anychar)))),
    ))(i)?;

    println!("{:?} {} {:?}", left, op, right);

    let symbol = Symbol {
        value: None,
        left: None,
        right: None,
        operator: Operator::And,
    };
    Ok((input, symbol))
}

// regex: ^=(\w)*\s*(?:#.+)?$
fn parse_initial_facts(i: &str) -> IResult<&str, Vec<String>> {
    let (input, (_, symbols, _, _)) = tuple((
        tag("="),
        many0(alpha1),
        // Ignore comments
        value((), multispace0),
        opt(tuple((tag("#"), many0(anychar)))),
    ))(i)?;

    let symbols = Vec::from_iter(symbols.iter().map(|item| String::from(*item)));
    Ok((input, symbols))
}

// regex: ^\?(\w)+\s*(?:#.+)?$
fn parse_queries(i: &str) -> IResult<&str, Vec<String>> {
    let (input, (_, symbols, _, _)) = tuple((
        tag("?"),
        many1(alpha1),
        // Ignore comments
        value((), multispace0),
        opt(tuple((tag("#"), many0(anychar)))),
    ))(i)?;

    let symbols = Vec::from_iter(symbols.iter().map(|item| String::from(*item)));
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
                if !input.symbol_values.contains_key(value) {
                    input.symbol_values.insert(value.to_string(), false);
                }
            }
            if let Some(left) = &symbol.left {
                add_symbol(input, left);
            }
            if let Some(right) = &symbol.right {
                add_symbol(input, right);
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
                let result = parse_queries(line);
                if let Ok((_, queries)) = result {
                    input.queries = queries;
                    parsed_queries = true
                } else {
                    let error = result.unwrap_err();
                    return Err(format!("{}\nLine {} `{}`", error, line_number, line));
                }
            }
            // Parse rule (and initial facts on error to switch context)
            else {
                let result = parse_rule(line);
                if let Ok((_, symbol)) = result {
                    add_symbol(&mut input, &symbol);
                    input.rules.push(symbol)
                }
                // Parse initial facts if the rule parser doesn't match
                else {
                    let original_error = result.unwrap_err();
                    let result = parse_initial_facts(line);
                    // If it's not the initial facts it's just an error
                    if result.is_err() {
                        return Err(format!(
                            "{}\nLine {} `{}`",
                            original_error, line_number, line
                        ));
                    }
                    // This block is always true
                    if let Ok((_, initial_facts)) = result {
                        for symbol in initial_facts {
                            input.initial_facts.insert(symbol, true);
                        }
                        parsed_initial_facts = true;
                    }
                }
            }
        }

        Ok(input)
    }

    fn check(&self) -> Result<(), String> {
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

        let content = content.unwrap();
        let input = Input::parse_content(&content);
        if let Ok(input) = &input {
            input.check()?;
        }
        input
    }
}
