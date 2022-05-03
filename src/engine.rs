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
    pub left: Option<Box<Symbol>>,
    pub right: Option<Box<Symbol>>,
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
}
