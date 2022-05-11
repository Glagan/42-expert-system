use expert_system::input::Input;

#[test]
fn context_unclosed() {
    let mut input = Input::new();
    let result = input.parse_rule_block("(");
    assert!(result.is_err())
}

#[test]
fn close_root_context() {
    let mut input = Input::new();
    let result = input.parse_rule_block(")");
    assert!(result.is_err())
}

#[test]
fn op_missing_fact_1() {
    let mut input = Input::new();
    let result = input.parse_rule_block("!");
    assert!(result.is_err())
}

#[test]
fn op_missing_fact_2() {
    let mut input = Input::new();
    let result = input.parse_rule_block("+");
    assert!(result.is_err())
}

#[test]
fn op_missing_fact_3() {
    let mut input = Input::new();
    let result = input.parse_rule_block("A+");
    assert!(result.is_err())
}

#[test]
fn op_missing_fact_4() {
    let mut input = Input::new();
    let result = input.parse_rule_block("+A");
    assert!(result.is_err())
}

#[test]
fn op_missing_fact_5() {
    let mut input = Input::new();
    let result = input.parse_rule_block("(A+)");
    assert!(result.is_err())
}

#[test]
fn op_missing_fact_6() {
    let mut input = Input::new();
    let result = input.parse_rule_block("!()");
    assert!(result.is_err())
}

#[test]
fn op_missing_fact_7() {
    let mut input = Input::new();
    let result = input.parse_rule_block("!(A+)");
    assert!(result.is_err())
}

#[test]
fn unused_nested_context() {
    let mut input = Input::new();
    let result = input.parse_rule_block("()");
    assert!(result.is_err())
}

#[test]
fn space_in_block() {
    // Any other characters than operators or symbols should already be removed when calling this function
    let mut input = Input::new();
    let result = input.parse_rule_block("A | B");
    assert!(result.is_err())
}

#[test]
fn success_1() {
    let mut input = Input::new();
    let result = input.parse_rule_block("A");
    assert!(result.is_ok())
}

#[test]
fn success_2() {
    let mut input = Input::new();
    let result = input.parse_rule_block("A+B");
    assert!(result.is_ok())
}

#[test]
fn success_3() {
    let mut input = Input::new();
    let result = input.parse_rule_block("(A+B)^C");
    assert!(result.is_ok())
}

#[test]
fn success_4() {
    let mut input = Input::new();
    let result = input.parse_rule_block("A+(B+C)+D");
    assert!(result.is_ok())
}

#[test]
fn success_5() {
    let mut input = Input::new();
    let result = input.parse_rule_block("!A");
    assert!(result.is_ok())
}

#[test]
fn success_6() {
    let mut input = Input::new();
    let result = input.parse_rule_block("!(A)");
    assert!(result.is_ok())
}

#[test]
fn success_7() {
    let mut input = Input::new();
    let result = input.parse_rule_block("!(A+B)");
    assert!(result.is_ok())
}

#[test]
fn success_8() {
    let mut input = Input::new();
    let result = input.parse_rule_block("(F^G)|(T+I)");
    assert!(result.is_ok())
}

#[test]
fn missing_facts() {
    let mut input = Input::new();
    let result = input.parse_content("A => B\n\n?B");
    assert!(result.is_err());
}

#[test]
fn missing_queries() {
    let mut input = Input::new();
    let result = input.parse_content("A => B\n=");
    assert!(result.is_err());
}

#[test]
fn empty_queries() {
    let mut input = Input::new();
    let result = input.parse_content("A => B\n=\n?");
    assert!(result.is_err());
}
