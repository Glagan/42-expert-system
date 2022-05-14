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
        *RefCell::borrow_mut(&self.value) = value;
        *RefCell::borrow_mut(&self.resolved) = true;
    }

    pub fn set_value(&self, value: Resolve) {
        *RefCell::borrow_mut(&self.value) = value;
    }

    pub fn cleanup(&self) {
        for rule in self.rules.iter() {
            RefCell::borrow(rule).cleanup()
        }
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
        if !self.rules.is_empty() {
            let mut final_result: Option<Resolve> = None;
            let rules_len = self.rules.len();
            for (index, rule) in self.rules.iter().enumerate() {
                // Skip infinite rules for Operator::IfAndOnlyIf if there is multiple rules that *could* resolve
                if *RefCell::borrow(rule).visited.borrow()
                    && rules_len > 1
                    && index + 1 != rules_len
                {
                    continue;
                }
                // Resolve the rule
                let result = RefCell::borrow(rule).resolve(&self.repr, path);
                if let Ok(result) = result {
                    if result.is_true() {
                        *RefCell::borrow_mut(&self.value) = result;
                        path.push(format!("{} is {}", self.repr, "true".cyan()));
                        return Ok(result);
                    } else if final_result.is_none() {
                        final_result = Some(result);
                    } else if final_result.unwrap().is_ambiguous() && result.is_false() {
                        final_result = Some(result);
                    }
                }
                // Infinite IfAndOnlyIf implications resolve to false
                else if RefCell::borrow(rule).operator_eq(&Operator::IfAndOnlyIf) {
                    return Ok(Resolve::False);
                } else {
                    return result;
                }
            }
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
            return Ok(final_result.unwrap());
        }
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

    pub fn contains_fact(&self, fact: &char) -> bool {
        if self.has_fact() && RefCell::borrow(self.fact.as_ref().unwrap()).repr == *fact {
            return true;
        }
        if self.has_left() {
            let has_on_left = RefCell::borrow(self.left.as_ref().unwrap()).contains_fact(fact);
            if has_on_left {
                return true;
            }
        }
        if self.has_right() {
            let has_on_right = RefCell::borrow(self.right.as_ref().unwrap()).contains_fact(fact);
            if has_on_right {
                return true;
            }
        }
        return false;
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
            if self.operator_eq(&Operator::Not) {
                print!("!");
            }
            if !self.operator_eq(&Operator::Implies) && !self.operator_eq(&Operator::IfAndOnlyIf) {
                print!("(");
            }
            RefCell::borrow(self.left.as_ref().unwrap()).print_short();
            if self.has_right() {
                print!(" ");
                match self.operator.unwrap() {
                    Operator::And => print!("+"),
                    Operator::Or => print!("|"),
                    Operator::Xor => print!("^"),
                    Operator::Not => (),
                    Operator::Implies => print!("=>"),
                    Operator::IfAndOnlyIf => print!("<=>"),
                };
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

    pub fn cleanup(&self) {
        if *self.visited.borrow() {
            *RefCell::borrow_mut(&self.visited) = false;
        }
        if self.has_left() {
            RefCell::borrow(self.left.as_ref().unwrap()).cleanup();
        }
        if self.has_right() {
            RefCell::borrow(self.right.as_ref().unwrap()).cleanup();
        }
    }

    pub fn resolve(&self, for_query: &char, path: &mut Vec<String>) -> Result<Resolve, String> {
        if *self.visited.borrow() {
            return Err(format!("Infinite rule {}", self));
        }
        *RefCell::borrow_mut(&self.visited) = true;
        if self.fact.is_some() {
            let result = RefCell::borrow(self.fact.as_ref().unwrap()).resolve(path)?;
            if self.operator_eq(&Operator::Not) {
                *RefCell::borrow_mut(&self.visited) = false;
                return Ok(result.not());
            }
            *RefCell::borrow_mut(&self.visited) = false;
            return Ok(result);
        } else if let Some(op) = &self.operator {
            path.push(self.to_string());
            let result = match op {
                Operator::Implies => {
                    let result =
                        RefCell::borrow(self.left.as_ref().unwrap()).resolve(for_query, path)?;
                    if result.is_true() {
                        let mut facts: Vec<Rc<RefCell<Fact>>> = vec![];
                        let result = RefCell::borrow(self.right.as_ref().unwrap())
                            .resolve_conclusion(result, &mut facts)?;
                        for fact in facts {
                            if result.is_true() {
                                RefCell::borrow(&fact).set(result);
                            } else if !*RefCell::borrow(&fact).resolved.borrow()
                                || (result.is_false()
                                    && RefCell::borrow(&fact).value.borrow().is_ambiguous())
                            {
                                RefCell::borrow(&fact).set_value(result);
                            }
                        }
                        Ok(result)
                    } else {
                        Ok(result)
                    }
                }
                Operator::IfAndOnlyIf => {
                    // Resolve left if for_query is on the right
                    if RefCell::borrow(self.right.as_ref().unwrap()).contains_fact(for_query) {
                        let left = RefCell::borrow(self.left.as_ref().unwrap())
                            .resolve(for_query, path)?;
                        if left.is_true() {
                            let mut facts: Vec<Rc<RefCell<Fact>>> = vec![];
                            let right_result = RefCell::borrow(self.right.as_ref().unwrap())
                                .resolve_conclusion(left, &mut facts)?;
                            if right_result.is_true() {
                                for fact in facts {
                                    RefCell::borrow(&fact).set(Resolve::True);
                                }
                                Ok(Resolve::True)
                            } else {
                                Ok(right_result)
                            }
                        } else {
                            Ok(left)
                        }
                    }
                    // -- else resolve right if for_query is on the left
                    else {
                        let right = RefCell::borrow(self.right.as_ref().unwrap())
                            .resolve(for_query, path)?;
                        if right.is_true() {
                            let mut facts: Vec<Rc<RefCell<Fact>>> = vec![];
                            let left_result = RefCell::borrow(self.left.as_ref().unwrap())
                                .resolve_conclusion(right, &mut facts)?;
                            if left_result.is_true() {
                                for fact in facts {
                                    RefCell::borrow(&fact).set(Resolve::True);
                                }
                                Ok(Resolve::True)
                            } else {
                                Ok(left_result)
                            }
                        } else {
                            Ok(right)
                        }
                    }
                }
                Operator::And => {
                    let left =
                        RefCell::borrow(self.left.as_ref().unwrap()).resolve(for_query, path)?;
                    let right =
                        RefCell::borrow(self.right.as_ref().unwrap()).resolve(for_query, path)?;
                    if left.is_ambiguous() || right.is_ambiguous() {
                        Ok(Resolve::Ambiguous)
                    } else if left.is_true() && right.is_true() {
                        Ok(Resolve::True)
                    } else {
                        Ok(Resolve::False)
                    }
                }
                Operator::Or => {
                    let left =
                        RefCell::borrow(self.left.as_ref().unwrap()).resolve(for_query, path)?;
                    let right =
                        RefCell::borrow(self.right.as_ref().unwrap()).resolve(for_query, path)?;
                    if left.is_ambiguous() || right.is_ambiguous() {
                        Ok(Resolve::Ambiguous)
                    } else if left.is_true() || right.is_true() {
                        Ok(Resolve::True)
                    } else {
                        Ok(Resolve::False)
                    }
                }
                Operator::Xor => {
                    let left =
                        RefCell::borrow(self.left.as_ref().unwrap()).resolve(for_query, path)?;
                    let right =
                        RefCell::borrow(self.right.as_ref().unwrap()).resolve(for_query, path)?;
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
                    let left =
                        RefCell::borrow(self.left.as_ref().unwrap()).resolve(for_query, path)?;
                    Ok(left.not())
                }
            };
            *RefCell::borrow_mut(&self.visited) = false;
            return result;
        } else if self.has_left() {
            let result = RefCell::borrow(self.left.as_ref().unwrap()).resolve(for_query, path)?;
            *RefCell::borrow_mut(&self.visited) = false;
            return Ok(result);
        }
        *RefCell::borrow_mut(&self.visited) = false;
        Err("Empty Node".to_string())
    }

    pub fn resolve_conclusion(
        &self,
        result: Resolve,
        facts: &mut Vec<Rc<RefCell<Fact>>>,
    ) -> Result<Resolve, String> {
        if self.fact.is_some() {
            facts.push(Rc::clone(self.fact.as_ref().unwrap()));
            if self.operator_eq(&Operator::Not) {
                return Ok(result.not());
            }
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
                    RefCell::borrow(self.left.as_ref().unwrap())
                        .resolve_conclusion(Resolve::Ambiguous, facts)?;
                    RefCell::borrow(self.right.as_ref().unwrap())
                        .resolve_conclusion(Resolve::Ambiguous, facts)?;
                    Ok(Resolve::Ambiguous)
                }
                Operator::Xor => {
                    RefCell::borrow(self.left.as_ref().unwrap())
                        .resolve_conclusion(Resolve::Ambiguous, facts)?;
                    RefCell::borrow(self.right.as_ref().unwrap())
                        .resolve_conclusion(Resolve::Ambiguous, facts)?;
                    Ok(Resolve::Ambiguous)
                }
                Operator::Not => {
                    let left = RefCell::borrow(self.left.as_ref().unwrap())
                        .resolve_conclusion(result, facts)?;
                    Ok(left.not())
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
