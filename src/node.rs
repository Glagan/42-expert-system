use colored::Colorize;
use std::{
    cell::RefCell,
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Resolve {
    True,
    Ambiguous,
    False,
}

impl Resolve {
    pub fn not(&self) -> Resolve {
        if *self == Resolve::True {
            return Resolve::False;
        } else if *self == Resolve::False {
            return Resolve::True;
        }
        Resolve::Ambiguous
    }

    pub fn is_true(&self) -> bool {
        *self == Resolve::True
    }

    pub fn is_ambiguous(&self) -> bool {
        *self == Resolve::Ambiguous
    }

    pub fn is_false(&self) -> bool {
        *self == Resolve::False
    }
}

#[derive(Clone, Debug)]
pub struct Fact {
    pub repr: char,
    pub value: RefCell<Resolve>,
    pub resolved: RefCell<bool>,
    pub rules: Vec<Rc<RefCell<Node>>>,
}

impl Fact {
    pub fn set(&self, value: Resolve) {
        *self.value.borrow_mut() = value;
        *self.resolved.borrow_mut() = true;
    }

    pub fn resolve(&self, path: &mut Vec<String>) -> Result<Resolve, String> {
        if *self.resolved.borrow() {
            path.push(format!(
                "{} is {}",
                self.repr,
                if *self.value.borrow() == Resolve::True {
                    "true".cyan()
                } else if *self.value.borrow() == Resolve::Ambiguous {
                    "ambiguous".purple()
                } else {
                    "false".yellow()
                }
            ));
            return Ok(*self.value.borrow());
        }
        *self.resolved.borrow_mut() = true;
        if !self.rules.is_empty() {
            for rule in self.rules.iter() {
                let result = RefCell::borrow(rule).resolve(path)?;
                if result.is_true() {
                    *self.value.borrow_mut() = result;
                    path.push(format!("{} is {}", self.repr, "true".cyan()));
                    return Ok(result);
                }
            }
        }
        path.push(format!("{} is {}", self.repr, "false".yellow()));
        Ok(*self.value.borrow())
    }
}

#[derive(Clone, Debug)]
pub struct Node {
    pub visited: RefCell<bool>,
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

impl Default for Node {
    fn default() -> Self {
        Self::new()
    }
}

impl Node {
    pub fn new() -> Node {
        Node {
            visited: RefCell::new(false),
            fact: None,
            left: None,
            right: None,
            operator: None,
        }
    }

    pub fn operator(operator: Operator) -> Node {
        Node {
            visited: RefCell::new(false),
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
            let repr = if *RefCell::borrow(self.fact.as_ref().unwrap())
                .resolved
                .borrow()
            {
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

    pub fn resolve(&self, path: &mut Vec<String>) -> Result<Resolve, String> {
        if *self.visited.borrow() {
            return Err(format!("Infinite rule {}", self));
        }
        *self.visited.borrow_mut() = true;
        if self.fact.is_some() {
            let result = RefCell::borrow(self.fact.as_ref().unwrap()).resolve(path)?;
            if self.operator_eq(&Operator::Not) {
                *self.visited.borrow_mut() = false;
                return Ok(result.not());
            }
            *self.visited.borrow_mut() = false;
            return Ok(result);
        } else if let Some(op) = &self.operator {
            path.push(self.to_string());
            let result = match op {
                Operator::Implies => {
                    let result = RefCell::borrow(self.left.as_ref().unwrap()).resolve(path)?;
                    if result.is_true() {
                        RefCell::borrow(self.right.as_ref().unwrap()).resolve_conclusion(result)
                    } else {
                        Ok(result)
                    }
                }
                Operator::IfAndOnlyIf => {
                    let left = RefCell::borrow(self.left.as_ref().unwrap()).resolve(path)?;
                    let right = RefCell::borrow(self.right.as_ref().unwrap()).resolve(path)?;
                    // TODO Resolve conclusion on both sides ? Always set to true if true
                    // if left && right {
                    //     RefCell::borrow(self.right.as_ref().unwrap())
                    //         .resolve_conclusion(result,  visited)
                    // }
                    if left.is_ambiguous() || right.is_ambiguous() {
                        Ok(Resolve::Ambiguous)
                    } else if left.is_true() && right.is_true() {
                        Ok(Resolve::True)
                    } else {
                        Ok(Resolve::False)
                    }
                }
                Operator::And => {
                    let left = RefCell::borrow(self.left.as_ref().unwrap()).resolve(path)?;
                    let right = RefCell::borrow(self.right.as_ref().unwrap()).resolve(path)?;
                    if left.is_ambiguous() || right.is_ambiguous() {
                        Ok(Resolve::Ambiguous)
                    } else if left.is_true() && right.is_true() {
                        Ok(Resolve::True)
                    } else {
                        Ok(Resolve::False)
                    }
                }
                Operator::Or => {
                    let left = RefCell::borrow(self.left.as_ref().unwrap()).resolve(path)?;
                    let right = RefCell::borrow(self.right.as_ref().unwrap()).resolve(path)?;
                    if left.is_ambiguous() || right.is_ambiguous() {
                        Ok(Resolve::Ambiguous)
                    } else if left.is_true() || right.is_true() {
                        Ok(Resolve::True)
                    } else {
                        Ok(Resolve::False)
                    }
                }
                Operator::Xor => {
                    let left = RefCell::borrow(self.left.as_ref().unwrap()).resolve(path)?;
                    let right = RefCell::borrow(self.right.as_ref().unwrap()).resolve(path)?;
                    if left.is_ambiguous() || right.is_ambiguous() {
                        Ok(Resolve::Ambiguous)
                    } else if (left.is_true() && right.is_false())
                        || (left.is_false() && right.is_true())
                    {
                        Ok(Resolve::True)
                    } else {
                        Ok(Resolve::False)
                    }
                }
                Operator::Not => {
                    let left = RefCell::borrow(self.left.as_ref().unwrap()).resolve(path)?;
                    Ok(left.not())
                }
            };
            *self.visited.borrow_mut() = false;
            return result;
        } else if self.has_left() {
            let result = RefCell::borrow(self.left.as_ref().unwrap()).resolve(path)?;
            *self.visited.borrow_mut() = false;
            return Ok(result);
        }
        *self.visited.borrow_mut() = false;
        Err("Empty Node".to_string())
    }

    // TODO Collect used Facts and set them to True *or* False if the result is not ambiguous
    pub fn resolve_conclusion(&self, result: Resolve) -> Result<Resolve, String> {
        if self.fact.is_some() {
            RefCell::borrow(self.fact.as_ref().unwrap()).set(result);
            if self.operator_eq(&Operator::Not) {
                return Ok(result.not());
            }
            return Ok(result);
        } else if let Some(op) = &self.operator {
            let result = match op {
                Operator::And => {
                    RefCell::borrow(self.left.as_ref().unwrap()).resolve_conclusion(result)?;
                    RefCell::borrow(self.right.as_ref().unwrap()).resolve_conclusion(result)?;
                    Ok(result)
                }
                Operator::Or => {
                    // TODO Check ambiguous
                    RefCell::borrow(self.left.as_ref().unwrap()).resolve_conclusion(result)?;
                    RefCell::borrow(self.right.as_ref().unwrap()).resolve_conclusion(result)?;
                    Ok(result)
                }
                Operator::Xor => {
                    // TODO Check ambiguous
                    RefCell::borrow(self.left.as_ref().unwrap()).resolve_conclusion(result)?;
                    RefCell::borrow(self.right.as_ref().unwrap()).resolve_conclusion(result)?;
                    Ok(result)
                }
                Operator::Not => {
                    let left =
                        RefCell::borrow(self.left.as_ref().unwrap()).resolve_conclusion(result)?;
                    Ok(left.not())
                }
                _ => Err("Unallowed operator in conclusion".to_string()),
            }?;
            return Ok(result);
        } else if self.has_left() {
            let result = RefCell::borrow(self.left.as_ref().unwrap()).resolve_conclusion(result)?;
            return Ok(result);
        }
        Err("Empty Node".to_string())
    }
}
