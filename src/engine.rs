#[derive(Clone, Debug)]
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
    pub value: Option<String>,
    pub left: Option<Box<Symbol>>,
    pub right: Option<Box<Symbol>>,
    pub operator: Operator,
}
