use crate::input::Input;

#[derive(Debug)]
#[allow(dead_code)]
pub enum ResultStatus {
    Success,
    Ambiguous,
    Failure,
}

#[derive(Debug)]
pub struct QueryResult {
    pub status: ResultStatus,
    pub value: bool,
    pub ambiguous_symbols: Vec<char>,
}

#[derive(Debug)]
pub struct Engine {
    pub input: Input,
}

impl Engine {
    pub fn resolve_query(&self, query: &char) -> Result<QueryResult, String> {
        // If the symbol doesn't exist anywhere in the input it's an error
        // ? Add it to check in the parser instead of here ?
        if !self.input.symbols.contains(query) {
            return Err(format!(
                "The symbol {} does not exists in the list of symbols.",
                query
            ));
        }

        // Resolve initial facts queries
        if self.input.initial_facts.contains_key(query)
            && *self.input.initial_facts.get(query).unwrap()
        {
            return Ok(QueryResult {
                status: ResultStatus::Success,
                value: true,
                ambiguous_symbols: vec![],
            });
        }

        // TODO
        Ok(QueryResult {
            status: ResultStatus::Ambiguous,
            value: false,
            ambiguous_symbols: vec![],
        })
    }
}
