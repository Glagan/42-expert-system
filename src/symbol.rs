use std::{cell::RefCell, rc::Rc};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Operator {
    Not,
    And,
    Or,
    Xor,
    Implies,
    IfAndOnlyIf,
}

#[derive(Clone, Debug)]
pub struct Symbol {
    pub value: Option<char>,
    pub left: Option<Rc<RefCell<Symbol>>>,
    pub right: Option<Rc<RefCell<Symbol>>>,
    pub operator: Option<Operator>,
}

impl Symbol {
    pub fn new() -> Symbol {
        Symbol {
            value: None,
            left: None,
            right: None,
            operator: None,
        }
    }

    pub fn unit(value: char) -> Symbol {
        Symbol {
            value: Some(value),
            left: None,
            right: None,
            operator: None,
        }
    }

    pub fn operator(operator: Operator) -> Symbol {
        Symbol {
            value: None,
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

    pub fn has_value(&self) -> bool {
        self.value.is_some()
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
}
