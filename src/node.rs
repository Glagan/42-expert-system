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

#[derive(Clone, Debug)]
pub struct Fact {
    pub repr: char,
    pub value: bool,
    pub resolved: bool,
    pub rules: Vec<Rc<RefCell<Node>>>,
}

impl Fact {
    #[allow(dead_code)]
    pub fn set(&mut self, value: bool) {
        self.value = value;
        self.resolved = true;
    }
}

#[derive(Clone, Debug)]
pub struct Node {
    pub fact: Option<Rc<RefCell<Fact>>>,
    pub left: Option<Rc<RefCell<Node>>>,
    pub right: Option<Rc<RefCell<Node>>>,
    pub operator: Option<Operator>,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.short(f)
    }
}

impl Node {
    pub fn new() -> Node {
        Node {
            fact: None,
            left: None,
            right: None,
            operator: None,
        }
    }

    pub fn operator(operator: Operator) -> Node {
        Node {
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

    // TODO Rewrite ?
    fn symbol_has_operator(symbol: &Rc<RefCell<Node>>, op: &Operator) -> bool {
        if RefCell::borrow(symbol).operator_eq(op) {
            return true;
        }
        let mut side_result = false;
        if let Some(left) = &RefCell::borrow(symbol).left {
            side_result = Node::symbol_has_operator(left, op);
        }
        if !side_result {
            if let Some(right) = &RefCell::borrow(symbol).right {
                side_result = Node::symbol_has_operator(right, op);
            }
        }
        side_result
    }

    // TODO Rewrite ?
    pub fn is_ambiguous(&self) -> bool {
        if let Some(right) = &self.right {
            if Node::symbol_has_operator(right, &Operator::Or) {
                return true;
            }
        }
        if self.operator_eq(&Operator::IfAndOnlyIf) {
            if let Some(left) = &self.left {
                if Node::symbol_has_operator(left, &Operator::Or) {
                    return true;
                }
            }
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

    // TODO Rewrite ?
    /*pub fn is_infinite(&self) -> bool {
        if let Some(left) = &self.left {
            if let Some(right) = &self.right {
                let left_symbols = RefCell::borrow(left).list_of_symbols();
                let right_symbols = RefCell::borrow(right).list_of_symbols();
                for right_symbol in right_symbols.iter() {
                    if left_symbols.contains(right_symbol) {
                        return true;
                    }
                }
            }
        }
        false
    }*/

    pub fn short(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.has_fact() {
            if self.operator_eq(&Operator::Not) {
                write!(
                    f,
                    "not {}",
                    RefCell::borrow(&self.fact.as_ref().unwrap()).repr
                )?;
            } else {
                write!(f, "{}", RefCell::borrow(&self.fact.as_ref().unwrap()).repr)?;
            }
        } else if self.has_operator() {
            RefCell::borrow(&self.left.as_ref().unwrap()).short(f)?;
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
                RefCell::borrow(&self.right.as_ref().unwrap()).short(f)?;
            }
        } else {
            if self.has_left() {
                RefCell::borrow(&self.left.as_ref().unwrap()).short(f)?;
            }
            if self.has_right() {
                write!(f, " ")?;
                RefCell::borrow(&self.right.as_ref().unwrap()).short(f)?;
            }
        }
        Ok(())
    }
}
