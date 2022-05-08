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
pub struct Symbol {
    pub value: Option<char>,
    pub left: Option<Rc<RefCell<Symbol>>>,
    pub right: Option<Rc<RefCell<Symbol>>>,
    pub operator: Option<Operator>,
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.short(f)
    }
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

    fn symbol_has_symbol(symbol: &Rc<RefCell<Symbol>>, query: &char) -> bool {
        if let Some(value) = &RefCell::borrow(symbol).value {
            if *value == *query {
                return true;
            }
        }
        let mut side_result = false;
        if let Some(left) = &RefCell::borrow(symbol).left {
            side_result = Symbol::symbol_has_symbol(left, query);
        }
        if !side_result {
            if let Some(right) = &RefCell::borrow(symbol).right {
                side_result = Symbol::symbol_has_symbol(right, query);
            }
        }
        side_result
    }

    pub fn imply_symbol(&self, query: &char) -> bool {
        if let Some(right) = &self.right {
            if Symbol::symbol_has_symbol(right, query) {
                return true;
            }
        }
        if self.operator_eq(&Operator::IfAndOnlyIf) {
            if let Some(left) = &self.left {
                if Symbol::symbol_has_symbol(left, query) {
                    return true;
                }
            }
        }
        false
    }

    fn symbol_has_operator(symbol: &Rc<RefCell<Symbol>>, op: &Operator) -> bool {
        if RefCell::borrow(symbol).operator_eq(op) {
            return true;
        }
        let mut side_result = false;
        if let Some(left) = &RefCell::borrow(symbol).left {
            side_result = Symbol::symbol_has_operator(left, op);
        }
        if !side_result {
            if let Some(right) = &RefCell::borrow(symbol).right {
                side_result = Symbol::symbol_has_operator(right, op);
            }
        }
        side_result
    }

    pub fn is_ambiguous(&self) -> bool {
        if let Some(right) = &self.right {
            if Symbol::symbol_has_operator(right, &Operator::Or) {
                return true;
            }
        }
        if self.operator_eq(&Operator::IfAndOnlyIf) {
            if let Some(left) = &self.left {
                if Symbol::symbol_has_operator(left, &Operator::Or) {
                    return true;
                }
            }
        }
        false
    }

    pub fn list_of_symbols(&self) -> Vec<char> {
        let mut symbols = vec![];
        if let Some(value) = &self.value {
            symbols.push(*value)
        }
        if let Some(left) = &self.left {
            let left_symbols = RefCell::borrow(left).list_of_symbols();
            symbols = [symbols, left_symbols].concat();
        }
        if let Some(right) = &self.right {
            let right_symbols = RefCell::borrow(right).list_of_symbols();
            symbols = [symbols, right_symbols].concat();
        }
        symbols
    }

    pub fn is_infinite(&self) -> bool {
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
    }

    pub fn short(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.has_value() {
            if self.operator_eq(&Operator::Not) {
                write!(f, "not {}", self.value.unwrap())?;
            } else {
                write!(f, "{}", self.value.unwrap())?;
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
