use colored::Colorize;
use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::{self, Debug},
    rc::Rc,
};

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum Operator {
    Not,
    And,
    Or,
    Xor,
    Implies,
    IfAndOnlyIf,
}

#[derive(Clone, Debug)]
pub struct Fact {
    pub repr: char,
    pub value: bool,
    pub resolved: bool,
    pub rules: Vec<Rc<RefCell<Node>>>,
}

impl Fact {
    pub fn set(&mut self, value: bool) {
        self.value = value;
        self.resolved = true;
    }

    pub fn resolve(
        &self,
        facts: &mut HashMap<char, bool>,
        visited: &mut Vec<i64>,
    ) -> Result<bool, String> {
        if self.resolved {
            return Ok(self.value);
        }
        if facts.contains_key(&self.repr) {
            return Ok(*facts.get(&self.repr).unwrap());
        }
        // self.resolved = true;
        if !self.rules.is_empty() {
            for rule in self.rules.iter() {
                let result = RefCell::borrow(rule).resolve(facts, visited)?;
                if result {
                    facts.insert(self.repr, result);
                    // self.value = result;
                    return Ok(result);
                }
            }
        }
        facts.insert(self.repr, self.value);
        Ok(self.value)
    }
}

#[derive(Clone, Debug)]
pub struct Node {
    pub id: i64,
    pub fact: Option<Rc<RefCell<Fact>>>,
    pub left: Option<Rc<RefCell<Node>>>,
    pub right: Option<Rc<RefCell<Node>>>,
    pub operator: Option<Operator>,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.has_fact() {
            if self.operator_eq(&Operator::Not) {
                write!(
                    f,
                    "not {}",
                    RefCell::borrow(self.fact.as_ref().unwrap()).repr
                )?;
            } else {
                write!(f, "{}", RefCell::borrow(self.fact.as_ref().unwrap()).repr)?;
            }
        } else if self.has_operator() {
            std::fmt::Display::fmt(&RefCell::borrow(self.left.as_ref().unwrap()), f)?;
            write!(f, " ")?;
            match self.operator.unwrap() {
                Operator::And => write!(f, "and"),
                Operator::Or => write!(f, "or"),
                Operator::Xor => write!(f, "xor"),
                Operator::Not => write!(f, "not"),
                Operator::Implies => write!(f, "implies"),
                Operator::IfAndOnlyIf => write!(f, "if and only if"),
            }?;
            if self.has_right() {
                write!(f, " ")?;
                std::fmt::Display::fmt(&RefCell::borrow(self.right.as_ref().unwrap()), f)?;
            }
        } else {
            if self.has_left() {
                std::fmt::Display::fmt(&RefCell::borrow(self.left.as_ref().unwrap()), f)?;
            }
            if self.has_right() {
                write!(f, " ")?;
                std::fmt::Display::fmt(&RefCell::borrow(self.right.as_ref().unwrap()), f)?;
            }
        }
        Ok(())
    }
}

impl Node {
    pub fn new() -> Node {
        Node {
            id: 0,
            fact: None,
            left: None,
            right: None,
            operator: None,
        }
    }

    pub fn operator(operator: Operator) -> Node {
        Node {
            id: 0,
            fact: None,
            left: None,
            right: None,
            operator: Some(operator),
        }
    }

    pub fn match_operator(op: char) -> Option<Operator> {
        if op == '+' {
            return Some(Operator::And);
        } else if op == '|' {
            return Some(Operator::Or);
        } else if op == '^' {
            return Some(Operator::Xor);
        }
        None
    }

    pub fn has_fact(&self) -> bool {
        self.fact.is_some()
    }

    pub fn has_left(&self) -> bool {
        self.left.is_some()
    }

    pub fn has_right(&self) -> bool {
        self.right.is_some()
    }

    pub fn has_operator(&self) -> bool {
        self.operator.is_some()
    }

    pub fn operator_eq(&self, op: &Operator) -> bool {
        if let Some(current_op) = &self.operator {
            return current_op == op;
        }
        false
    }

    pub fn all_facts(&self) -> Vec<Rc<RefCell<Fact>>> {
        let mut facts = vec![];
        if let Some(value) = &self.fact {
            facts.push(Rc::clone(value));
        }
        if let Some(left) = &self.left {
            let left_facts = RefCell::borrow(left).all_facts();
            facts = [facts, left_facts].concat();
        }
        if let Some(right) = &self.right {
            let right_facts = RefCell::borrow(right).all_facts();
            facts = [facts, right_facts].concat();
        }
        facts
    }

    pub fn print_short(&self) {
        if self.has_fact() {
            let repr = if RefCell::borrow(self.fact.as_ref().unwrap()).resolved {
                format!(
                    "{}",
                    format!("{}", RefCell::borrow(self.fact.as_ref().unwrap()).repr).green()
                )
            } else {
                format!("{}", RefCell::borrow(self.fact.as_ref().unwrap()).repr)
            };
            if self.operator_eq(&Operator::Not) {
                print!("!{}", repr);
            } else {
                print!("{}", repr);
            }
        } else if self.has_operator() {
            if !self.operator_eq(&Operator::Implies) && !self.operator_eq(&Operator::IfAndOnlyIf) {
                print!("(");
            }
            RefCell::borrow(self.left.as_ref().unwrap()).print_short();
            print!(" ");
            match self.operator.unwrap() {
                Operator::And => print!("+"),
                Operator::Or => print!("|"),
                Operator::Xor => print!("^"),
                Operator::Not => print!("!"),
                Operator::Implies => print!("=>"),
                Operator::IfAndOnlyIf => print!("<=>"),
            };
            if self.has_right() {
                print!(" ");
                RefCell::borrow(self.right.as_ref().unwrap()).print_short();
            }
            if !self.operator_eq(&Operator::Implies) && !self.operator_eq(&Operator::IfAndOnlyIf) {
                print!(")");
            }
        } else {
            if self.has_left() {
                RefCell::borrow(self.left.as_ref().unwrap()).print_short();
            }
            if self.has_right() {
                print!(" ");
                RefCell::borrow(self.right.as_ref().unwrap()).print_short();
            }
        }
    }

    pub fn resolve(
        &self,
        facts: &mut HashMap<char, bool>,
        visited: &mut Vec<i64>,
    ) -> Result<bool, String> {
        if visited.contains(&self.id) {
            return Err("Infinite rule".to_string());
        }
        visited.push(self.id);
        if self.fact.is_some() {
            let result = RefCell::borrow(self.fact.as_ref().unwrap()).resolve(facts, visited)?;
            if self.operator_eq(&Operator::Not) {
                visited.swap_remove(visited.iter().position(|id| *id == self.id).unwrap());
                return Ok(!result);
            }
            visited.swap_remove(visited.iter().position(|id| *id == self.id).unwrap());
            return Ok(result);
        } else if let Some(op) = &self.operator {
            let result = match op {
                Operator::Implies => {
                    let result =
                        RefCell::borrow(self.left.as_ref().unwrap()).resolve(facts, visited)?;
                    RefCell::borrow(self.right.as_ref().unwrap()).resolve_conclusion(result, facts)
                }
                Operator::IfAndOnlyIf => {
                    let left =
                        RefCell::borrow(self.left.as_ref().unwrap()).resolve(facts, visited)?;
                    let right =
                        RefCell::borrow(self.right.as_ref().unwrap()).resolve(facts, visited)?;
                    // TODO Resolve conclusion on both sides ? Always set to true if true
                    // if left && right {
                    //     RefCell::borrow(self.right.as_ref().unwrap())
                    //         .resolve_conclusion(result, facts, visited)
                    // }
                    Ok(left && right)
                }
                Operator::And => {
                    let left =
                        RefCell::borrow(self.left.as_ref().unwrap()).resolve(facts, visited)?;
                    let right =
                        RefCell::borrow(self.right.as_ref().unwrap()).resolve(facts, visited)?;
                    Ok(left && right)
                }
                Operator::Or => {
                    let left =
                        RefCell::borrow(self.left.as_ref().unwrap()).resolve(facts, visited)?;
                    let right =
                        RefCell::borrow(self.right.as_ref().unwrap()).resolve(facts, visited)?;
                    Ok(left || right)
                }
                Operator::Xor => {
                    let left =
                        RefCell::borrow(self.left.as_ref().unwrap()).resolve(facts, visited)?;
                    let right =
                        RefCell::borrow(self.right.as_ref().unwrap()).resolve(facts, visited)?;
                    Ok((left && !right) || (!left && right))
                }
                Operator::Not => {
                    let left =
                        RefCell::borrow(self.left.as_ref().unwrap()).resolve(facts, visited)?;
                    return Ok(!left);
                }
            };
            visited.swap_remove(visited.iter().position(|id| *id == self.id).unwrap());
            return result;
        } else if self.has_left() {
            let result = RefCell::borrow(self.left.as_ref().unwrap()).resolve(facts, visited)?;
            visited.swap_remove(visited.iter().position(|id| *id == self.id).unwrap());
            return Ok(result);
        }
        visited.swap_remove(visited.iter().position(|id| *id == self.id).unwrap());
        Err("Empty Node".to_string())
    }

    pub fn resolve_conclusion(
        &self,
        result: bool,
        facts: &mut HashMap<char, bool>,
    ) -> Result<bool, String> {
        if self.fact.is_some() {
            if self.operator_eq(&Operator::Not) {
                facts.insert(RefCell::borrow(self.fact.as_ref().unwrap()).repr, !result);
                return Ok(!result);
            }
            facts.insert(RefCell::borrow(self.fact.as_ref().unwrap()).repr, result);
            return Ok(result);
        } else if let Some(op) = &self.operator {
            let result = match op {
                Operator::And => {
                    RefCell::borrow(self.left.as_ref().unwrap())
                        .resolve_conclusion(result, facts)?;
                    RefCell::borrow(self.right.as_ref().unwrap())
                        .resolve_conclusion(result, facts)?;
                    Ok(result)
                }
                Operator::Or => {
                    // TODO Check ambiguous
                    RefCell::borrow(self.left.as_ref().unwrap())
                        .resolve_conclusion(result, facts)?;
                    RefCell::borrow(self.right.as_ref().unwrap())
                        .resolve_conclusion(result, facts)?;
                    Ok(result)
                }
                Operator::Xor => {
                    // TODO Check ambiguous
                    RefCell::borrow(self.left.as_ref().unwrap())
                        .resolve_conclusion(result, facts)?;
                    RefCell::borrow(self.right.as_ref().unwrap())
                        .resolve_conclusion(result, facts)?;
                    Ok(result)
                }
                Operator::Not => {
                    let left = RefCell::borrow(self.left.as_ref().unwrap())
                        .resolve_conclusion(false, facts)?;
                    return Ok(!left);
                }
                _ => Err("Unallowed operator in conclusion".to_string()),
            }?;
            return Ok(result);
        } else if self.has_left() {
            let result =
                RefCell::borrow(self.left.as_ref().unwrap()).resolve_conclusion(result, facts)?;
            return Ok(result);
        }
        Err("Empty Node".to_string())
    }
}
