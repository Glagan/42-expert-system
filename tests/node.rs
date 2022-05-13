use expert_system::input::Input;

#[test]
fn basic_query_resolve_1() {
    let mut input = Input::new();
    let result = input.parse_content("A => B\n=\n?B");
    assert!(result.is_ok());
    let mut path: Vec<String> = vec![];
    let query_result = input
        .facts
        .get(input.queries.first().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_false());
}

#[test]
fn basic_query_resolve_2() {
    let mut input = Input::new();
    let result = input.parse_content("A => B\n=A\n?B");
    assert!(result.is_ok());
    let mut path: Vec<String> = vec![];
    let query_result = input
        .facts
        .get(input.queries.first().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_true());
}

#[test]
fn basic_query_resolve_nested() {
    let mut input = Input::new();
    let result = input.parse_content("A => B\nB => C\n=A\n?C");
    assert!(result.is_ok());
    let mut path: Vec<String> = vec![];
    let query_result = input
        .facts
        .get(input.queries.first().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_true());
}

#[test]
fn basic_query_or_1() {
    let mut input = Input::new();
    let result = input.parse_content("A | B => C\n=\n?C");
    assert!(result.is_ok());
    let mut path: Vec<String> = vec![];
    let query_result = input
        .facts
        .get(input.queries.first().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_false());
}

#[test]
fn basic_query_or_2() {
    let mut input = Input::new();
    let result = input.parse_content("A | B => C\n=A\n?C");
    assert!(result.is_ok());
    let mut path: Vec<String> = vec![];
    let query_result = input
        .facts
        .get(input.queries.first().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_true());
}

#[test]
fn basic_query_or_3() {
    let mut input = Input::new();
    let result = input.parse_content("A | B => C\n=B\n?C");
    assert!(result.is_ok());
    let mut path: Vec<String> = vec![];
    let query_result = input
        .facts
        .get(input.queries.first().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_true());
}

#[test]
fn basic_query_or_4() {
    let mut input = Input::new();
    let result = input.parse_content("A | B => C\n=AB\n?C");
    assert!(result.is_ok());
    let mut path: Vec<String> = vec![];
    let query_result = input
        .facts
        .get(input.queries.first().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_true());
}

#[test]
fn basic_query_and_1() {
    let mut input = Input::new();
    let result = input.parse_content("A + B => C\n=\n?C");
    assert!(result.is_ok());
    let mut path: Vec<String> = vec![];
    let query_result = input
        .facts
        .get(input.queries.first().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_false());
}

#[test]
fn basic_query_and_2() {
    let mut input = Input::new();
    let result = input.parse_content("A + B => C\n=A\n?C");
    assert!(result.is_ok());
    let mut path: Vec<String> = vec![];
    let query_result = input
        .facts
        .get(input.queries.first().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_false());
}

#[test]
fn basic_query_and_3() {
    let mut input = Input::new();
    let result = input.parse_content("A + B => C\n=B\n?C");
    assert!(result.is_ok());
    let mut path: Vec<String> = vec![];
    let query_result = input
        .facts
        .get(input.queries.first().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_false());
}

#[test]
fn basic_query_and_4() {
    let mut input = Input::new();
    let result = input.parse_content("A + B => C\n=AB\n?C");
    assert!(result.is_ok());
    let mut path: Vec<String> = vec![];
    let query_result = input
        .facts
        .get(input.queries.first().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_true());
}

#[test]
fn basic_query_xor_1() {
    let mut input = Input::new();
    let result = input.parse_content("A ^ B => C\n=\n?C");
    assert!(result.is_ok());
    let mut path: Vec<String> = vec![];
    let query_result = input
        .facts
        .get(input.queries.first().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_false());
}

#[test]
fn basic_query_xor_2() {
    let mut input = Input::new();
    let result = input.parse_content("A ^ B => C\n=A\n?C");
    assert!(result.is_ok());
    let mut path: Vec<String> = vec![];
    let query_result = input
        .facts
        .get(input.queries.first().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_true());
}

#[test]
fn basic_query_xor_3() {
    let mut input = Input::new();
    let result = input.parse_content("A ^ B => C\n=B\n?C");
    assert!(result.is_ok());
    let mut path: Vec<String> = vec![];
    let query_result = input
        .facts
        .get(input.queries.first().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_true());
}

#[test]
fn basic_query_xor_4() {
    let mut input = Input::new();
    let result = input.parse_content("A ^ B => C\n=AB\n?C");
    assert!(result.is_ok());
    let mut path: Vec<String> = vec![];
    let query_result = input
        .facts
        .get(input.queries.first().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_false());
}

#[test]
fn basic_query_negation_1() {
    let mut input = Input::new();
    let result = input.parse_content("!A => B\n=\n?B");
    assert!(result.is_ok());
    let mut path: Vec<String> = vec![];
    let query_result = input
        .facts
        .get(input.queries.first().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_true());
}

#[test]
fn basic_query_negation_2() {
    let mut input = Input::new();
    let result = input.parse_content("!A => B\n=A\n?B");
    assert!(result.is_ok());
    let mut path: Vec<String> = vec![];
    let query_result = input
        .facts
        .get(input.queries.first().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_false());
}

#[test]
fn basic_query_and_negation_1() {
    let mut input = Input::new();
    let result = input.parse_content("!A + B => C\n=\n?C");
    assert!(result.is_ok());
    let mut path: Vec<String> = vec![];
    let query_result = input
        .facts
        .get(input.queries.first().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_false());
}

#[test]
fn basic_query_and_negation_2() {
    let mut input = Input::new();
    let result = input.parse_content("!A + B => C\n=A\n?C");
    assert!(result.is_ok());
    let mut path: Vec<String> = vec![];
    let query_result = input
        .facts
        .get(input.queries.first().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_false());
}

#[test]
fn basic_query_and_negation_3() {
    let mut input = Input::new();
    let result = input.parse_content("!A + B => C\n=B\n?C");
    assert!(result.is_ok());
    let mut path: Vec<String> = vec![];
    let query_result = input
        .facts
        .get(input.queries.first().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_true());
}

#[test]
fn basic_query_and_negation_4() {
    let mut input = Input::new();
    let result = input.parse_content("!A + B => C\n=AB\n?C");
    assert!(result.is_ok());
    let mut path: Vec<String> = vec![];
    let query_result = input
        .facts
        .get(input.queries.first().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_false());
}

#[test]
fn basic_query_parenthesis_1() {
    let mut input = Input::new();
    let result = input.parse_content("(A) => B\n=\n?B");
    assert!(result.is_ok());
    let mut path: Vec<String> = vec![];
    let query_result = input
        .facts
        .get(input.queries.first().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_false());
}

#[test]
fn basic_query_parenthesis_2() {
    let mut input = Input::new();
    let result = input.parse_content("(A) => B\n=A\n?B");
    assert!(result.is_ok());
    let mut path: Vec<String> = vec![];
    let query_result = input
        .facts
        .get(input.queries.first().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_true());
}

#[test]
fn basic_query_parenthesis_negation_1() {
    let mut input = Input::new();
    let result = input.parse_content("!(A) => B\n=\n?B");
    assert!(result.is_ok());
    let mut path: Vec<String> = vec![];
    let query_result = input
        .facts
        .get(input.queries.first().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_true());
}

#[test]
fn basic_query_parenthesis_negation_2() {
    let mut input = Input::new();
    let result = input.parse_content("!(A) => B\n=A\n?B");
    assert!(result.is_ok());
    let mut path: Vec<String> = vec![];
    let query_result = input
        .facts
        .get(input.queries.first().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_false());
}

#[test]
fn negative_conclusion_1() {
    let mut input = Input::new();
    let result = input.parse_content("A => !B\n=\n?B");
    assert!(result.is_ok());
    let mut path: Vec<String> = vec![];
    let query_result = input
        .facts
        .get(input.queries.first().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_false());
}

#[test]
fn negative_conclusion_2() {
    let mut input = Input::new();
    let result = input.parse_content("A => !B\n=A\n?B");
    assert!(result.is_ok());
    let mut path: Vec<String> = vec![];
    let query_result = input
        .facts
        .get(input.queries.first().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_false());
}

#[test]
fn and_conclusion_1() {
    let mut input = Input::new();
    let result = input.parse_content("A => B + C\n=\n?BC");
    assert!(result.is_ok());
    let mut path: Vec<String> = vec![];
    let query_result = input
        .facts
        .get(input.queries.first().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_false());
    let query_result = input
        .facts
        .get(input.queries.last().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_false());
}

#[test]
fn and_conclusion_2() {
    let mut input = Input::new();
    let result = input.parse_content("A => B + C\n=A\n?BC");
    assert!(result.is_ok());
    let mut path: Vec<String> = vec![];
    let query_result = input
        .facts
        .get(input.queries.first().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_true());
    let query_result = input
        .facts
        .get(input.queries.last().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_true());
}

#[test]
fn and_conclusion_parenthesis_1() {
    let mut input = Input::new();
    let result = input.parse_content("A => (B + C)\n=\n?BC");
    assert!(result.is_ok());
    let mut path: Vec<String> = vec![];
    let query_result = input
        .facts
        .get(input.queries.first().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_false());
    let query_result = input
        .facts
        .get(input.queries.last().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_false());
}

#[test]
fn and_conclusion_parenthesis_2() {
    let mut input = Input::new();
    let result = input.parse_content("A => (B + C)\n=A\n?BC");
    assert!(result.is_ok());
    let mut path: Vec<String> = vec![];
    let query_result = input
        .facts
        .get(input.queries.first().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_true());
    let query_result = input
        .facts
        .get(input.queries.last().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_true());
}

#[test]
fn negative_and_conclusion_1() {
    let mut input = Input::new();
    let result = input.parse_content("A => B + !C\n=\n?BC");
    assert!(result.is_ok());
    let mut path: Vec<String> = vec![];
    let query_result = input
        .facts
        .get(input.queries.first().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_false());
    let query_result = input
        .facts
        .get(input.queries.last().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_false());
}

#[test]
fn negative_and_conclusion_2() {
    let mut input = Input::new();
    let result = input.parse_content("A => B + !C\n=A\n?BC");
    assert!(result.is_ok());
    let mut path: Vec<String> = vec![];
    let query_result = input
        .facts
        .get(input.queries.first().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_true());
    let query_result = input
        .facts
        .get(input.queries.last().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_true());
}

#[test]
fn nested_a_lot_1() {
    let mut input = Input::new();
    let result = input.parse_content(
        "!(!(A+!(!C))) | (!(!(A+!(!C)))) + !(!(A+!(!C))) | (!(!(A+!(!C)))) => B\n=\n?B",
    );
    assert!(result.is_ok());
    let mut path: Vec<String> = vec![];
    let query_result = input
        .facts
        .get(input.queries.first().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_false());
}

#[test]
fn nested_a_lot_2() {
    let mut input = Input::new();
    let result = input.parse_content(
        "!(!(A+!(!C))) | (!(!(A+!(!C)))) + !(!(A+!(!C))) | (!(!(A+!(!C)))) => B\n=A\n?B",
    );
    assert!(result.is_ok());
    let mut path: Vec<String> = vec![];
    let query_result = input
        .facts
        .get(input.queries.first().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_false());
}

#[test]
fn nested_a_lot_3() {
    let mut input = Input::new();
    let result = input.parse_content(
        "!(!(A+!(!C))) | (!(!(A+!(!C)))) + !(!(A+!(!C))) | (!(!(A+!(!C)))) => B\n=AC\n?B",
    );
    assert!(result.is_ok());
    let mut path: Vec<String> = vec![];
    let query_result = input
        .facts
        .get(input.queries.first().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_true());
}

#[test]
fn or_conclusion_1() {
    let mut input = Input::new();
    let result = input.parse_content("A => C | D\n=\n?CD");
    assert!(result.is_ok());
    let mut path: Vec<String> = vec![];
    let query_result = input
        .facts
        .get(input.queries.first().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_false());
    let query_result = input
        .facts
        .get(input.queries.last().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_false());
}

#[test]
fn or_conclusion_2() {
    let mut input = Input::new();
    let result = input.parse_content("A => C | D\n=A\n?CD");
    assert!(result.is_ok());
    let mut path: Vec<String> = vec![];
    let query_result = input
        .facts
        .get(input.queries.first().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_ambiguous());
    let query_result = input
        .facts
        .get(input.queries.last().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_ambiguous());
}

#[test]
fn resolved_or_conclusion_1() {
    let mut input = Input::new();
    let result = input.parse_content("A => C | D\nA => C\nC => D\n=\n?CD");
    assert!(result.is_ok());
    let mut path: Vec<String> = vec![];
    let query_result = input
        .facts
        .get(input.queries.first().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_false());
    let query_result = input
        .facts
        .get(input.queries.last().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_false());
}

#[test]
fn resolved_or_conclusion_2() {
    let mut input = Input::new();
    let result = input.parse_content("A => C | D\nA => C\nC => D\n=A\n?CD");
    assert!(result.is_ok());
    let mut path: Vec<String> = vec![];
    let query_result = input
        .facts
        .get(input.queries.first().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_true());
    let query_result = input
        .facts
        .get(input.queries.last().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_true());
}

#[test]
fn if_and_only_if_1() {
    let mut input = Input::new();
    let result = input.parse_content("A <=> B\n=\n?AB");
    assert!(result.is_ok());
    let mut path: Vec<String> = vec![];
    let query_result = input
        .facts
        .get(input.queries.first().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_false());
    let query_result = input
        .facts
        .get(input.queries.last().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_false());
}

#[test]
fn if_and_only_if_2() {
    let mut input = Input::new();
    let result = input.parse_content("A <=> B\n=A\n?AB");
    assert!(result.is_ok());
    let mut path: Vec<String> = vec![];
    let query_result = input
        .facts
        .get(input.queries.first().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_true());
    let query_result = input
        .facts
        .get(input.queries.last().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_true());
}

#[test]
fn if_and_only_if_3() {
    let mut input = Input::new();
    let result = input.parse_content("A <=> B\n=AB\n?AB");
    assert!(result.is_ok());
    let mut path: Vec<String> = vec![];
    let query_result = input
        .facts
        .get(input.queries.first().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_true());
    let query_result = input
        .facts
        .get(input.queries.last().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_true());
}

#[test]
fn if_and_only_if_4() {
    let mut input = Input::new();
    let result = input.parse_content("A <=> B\n=AB\n?AB");
    assert!(result.is_ok());
    let mut path: Vec<String> = vec![];
    let query_result = input
        .facts
        .get(input.queries.first().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_true());
    let query_result = input
        .facts
        .get(input.queries.last().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_true());
}

#[test]
fn if_and_only_if_and_conclusion_1() {
    let mut input = Input::new();
    let result = input.parse_content("A <=> B + C\n=\n?BC");
    assert!(result.is_ok());
    let mut path: Vec<String> = vec![];
    let query_result = input
        .facts
        .get(input.queries.first().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_false());
    let query_result = input
        .facts
        .get(input.queries.last().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_false());
}

#[test]
fn if_and_only_if_and_conclusion_2() {
    let mut input = Input::new();
    let result = input.parse_content("A <=> B + C\n=A\n?BC");
    assert!(result.is_ok());
    let mut path: Vec<String> = vec![];
    let query_result = input
        .facts
        .get(input.queries.first().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_true());
    let query_result = input
        .facts
        .get(input.queries.last().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_true());
}

#[test]
fn basic_query_if_and_only_if_1() {
    let mut input = Input::new();
    let result = input.parse_content("A + B <=> C\n=\n?C");
    assert!(result.is_ok());
    let mut path: Vec<String> = vec![];
    let query_result = input
        .facts
        .get(input.queries.first().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_false());
}

#[test]
fn basic_query_if_and_only_if_2() {
    let mut input = Input::new();
    let result = input.parse_content("A + B <=> C\n=A\n?C");
    assert!(result.is_ok());
    let mut path: Vec<String> = vec![];
    let query_result = input
        .facts
        .get(input.queries.first().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_false());
}

#[test]
fn basic_query_if_and_only_if_3() {
    let mut input = Input::new();
    let result = input.parse_content("A + B <=> C\n=B\n?C");
    assert!(result.is_ok());
    let mut path: Vec<String> = vec![];
    let query_result = input
        .facts
        .get(input.queries.first().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_false());
}

#[test]
fn basic_query_if_and_only_if_4() {
    let mut input = Input::new();
    let result = input.parse_content("A + B <=> C\n=AB\n?C");
    assert!(result.is_ok());
    let mut path: Vec<String> = vec![];
    let query_result = input
        .facts
        .get(input.queries.first().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_true());
}

#[test]
fn if_and_only_if_negation_1() {
    let mut input = Input::new();
    let result = input.parse_content("A <=> !C\n=\n?C");
    assert!(result.is_ok());
    let mut path: Vec<String> = vec![];
    let query_result = input
        .facts
        .get(input.queries.first().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_false());
}

#[test]
fn if_and_only_if_negation_2() {
    let mut input = Input::new();
    let result = input.parse_content("A <=> !C\n=A\n?C");
    assert!(result.is_ok());
    let mut path: Vec<String> = vec![];
    let query_result = input
        .facts
        .get(input.queries.first().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_false());
}

#[test]
fn if_and_only_if_negation_3() {
    let mut input = Input::new();
    let result = input.parse_content("A <=> !C\n=\n?A");
    assert!(result.is_ok());
    let mut path: Vec<String> = vec![];
    let query_result = input
        .facts
        .get(input.queries.first().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_true());
}

#[test]
fn if_and_only_if_negation_4() {
    let mut input = Input::new();
    let result = input.parse_content("A <=> !C\n=C\n?A");
    assert!(result.is_ok());
    let mut path: Vec<String> = vec![];
    let query_result = input
        .facts
        .get(input.queries.first().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_false());
}

#[test]
fn and_negation_if_and_only_if_1() {
    let mut input = Input::new();
    let result = input.parse_content("A + !B <=> C\n=\n?ABC");
    assert!(result.is_ok());
    let mut path: Vec<String> = vec![];
    let query_result = input
        .facts
        .get(input.queries.first().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_false());
    let query_result = input
        .facts
        .get(input.queries.get(1).unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_false());
    let query_result = input
        .facts
        .get(input.queries.last().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_false());
}

#[test]
fn and_negation_if_and_only_if_2() {
    let mut input = Input::new();
    let result = input.parse_content("A + !B <=> C\n=A\n?ABC");
    assert!(result.is_ok());
    let mut path: Vec<String> = vec![];
    let query_result = input
        .facts
        .get(input.queries.first().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_true());
    let query_result = input
        .facts
        .get(input.queries.get(1).unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_false());
    let query_result = input
        .facts
        .get(input.queries.last().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_true());
}

#[test]
fn and_negation_if_and_only_if_3() {
    let mut input = Input::new();
    let result = input.parse_content("A + !B <=> C\n=B\n?ABC");
    assert!(result.is_ok());
    let mut path: Vec<String> = vec![];
    let query_result = input
        .facts
        .get(input.queries.first().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_false());
    let query_result = input
        .facts
        .get(input.queries.get(1).unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_true());
    let query_result = input
        .facts
        .get(input.queries.last().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_false());
}

#[test]
fn and_negation_if_and_only_if_4() {
    let mut input = Input::new();
    let result = input.parse_content("A + !B <=> C\n=AB\n?ABC");
    assert!(result.is_ok());
    let mut path: Vec<String> = vec![];
    let query_result = input
        .facts
        .get(input.queries.first().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_true());
    let query_result = input
        .facts
        .get(input.queries.get(1).unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_true());
    let query_result = input
        .facts
        .get(input.queries.last().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_false());
}

#[test]
fn resolved_if_and_only_if_1() {
    let mut input = Input::new();
    let result = input.parse_content("A <=> B | C\nA => B\nA => C\n=\n?BC");
    assert!(result.is_ok());
    let mut path: Vec<String> = vec![];
    let query_result = input
        .facts
        .get(input.queries.first().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_false());
    let query_result = input
        .facts
        .get(input.queries.last().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_false());
}

#[test]
fn resolved_if_and_only_if_2() {
    let mut input = Input::new();
    let result = input.parse_content("A <=> B | C\n=A\n?BC");
    assert!(result.is_ok());
    let mut path: Vec<String> = vec![];
    let query_result = input
        .facts
        .get(input.queries.first().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_ambiguous());
    let query_result = input
        .facts
        .get(input.queries.last().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_ambiguous());
}

#[test]
fn resolved_if_and_only_if_3() {
    let mut input = Input::new();
    let result = input.parse_content("A => B | C\nA => B\nA => C\n=A\n?BC");
    assert!(result.is_ok());
    let mut path: Vec<String> = vec![];
    let query_result = input
        .facts
        .get(input.queries.first().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_true());
    let query_result = input
        .facts
        .get(input.queries.last().unwrap())
        .unwrap()
        .as_ref()
        .borrow()
        .resolve(&mut path);
    assert!(query_result.is_ok());
    assert!(query_result.unwrap().is_true());
}
