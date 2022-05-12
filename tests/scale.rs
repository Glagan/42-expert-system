use expert_system::input::Input;

fn expect_true(input: &Input, queries: Vec<char>) -> bool {
    for query in queries.iter() {
        let mut path: Vec<String> = vec![];
        let query_result = input
            .facts
            .get(query)
            .unwrap()
            .as_ref()
            .borrow()
            .resolve(&mut path);
        assert!(query_result.is_ok());
        assert!(query_result.unwrap().is_true());
    }
    true
}

fn expect_false(input: &Input, queries: Vec<char>) -> bool {
    for query in queries.iter() {
        let mut path: Vec<String> = vec![];
        let query_result = input
            .facts
            .get(query)
            .unwrap()
            .as_ref()
            .borrow()
            .resolve(&mut path);
        assert!(query_result.is_ok());
        assert!(query_result.unwrap().is_false());
    }
    true
}

// * Scale 1

fn scale_1(initial_facts: &str) -> String {
    format!(
        "B => A
		D + E => B
		G + H => F
		I + J => G
		G => H
		L + M => K
		O + P => L + N
		N => M
		={}
		?AFKP",
        initial_facts
    )
}

#[test]
fn scale_1_1() {
    let mut input = Input::new();
    let parse_result = input.parse_content(&scale_1(&"DEIJOP"));
    assert!(parse_result.is_ok());
    expect_true(&input, vec!['A', 'F', 'K', 'P']);
}

#[test]
fn scale_1_2() {
    let mut input = Input::new();
    let parse_result = input.parse_content(&scale_1(&"DEIJP"));
    assert!(parse_result.is_ok());
    expect_true(&input, vec!['A', 'F', 'P']);
    expect_false(&input, vec!['K']);
}

// * Scale 2

fn scale_2(initial_facts: &str) -> String {
    format!(
        "B + C => A
		D | E => B
		B => C
		={}
		?A",
        initial_facts
    )
}

#[test]
fn scale_2_1() {
    let mut input = Input::new();
    let parse_result = input.parse_content(&scale_2(&""));
    assert!(parse_result.is_ok());
    expect_false(&input, vec!['A']);
}

#[test]
fn scale_2_2() {
    let mut input = Input::new();
    let parse_result = input.parse_content(&scale_2(&"D"));
    assert!(parse_result.is_ok());
    expect_true(&input, vec!['A']);
}

#[test]
fn scale_2_3() {
    let mut input = Input::new();
    let parse_result = input.parse_content(&scale_2(&"E"));
    assert!(parse_result.is_ok());
    expect_true(&input, vec!['A']);
}

#[test]
fn scale_2_4() {
    let mut input = Input::new();
    let parse_result = input.parse_content(&scale_2(&"DE"));
    assert!(parse_result.is_ok());
    expect_true(&input, vec!['A']);
}

// * Scale 3

fn scale_3(initial_facts: &str) -> String {
    format!(
        "B + C => A
		D ^ E => B
		B => C
		={}
		?A",
        initial_facts
    )
}

#[test]
fn scale_3_1() {
    let mut input = Input::new();
    let parse_result = input.parse_content(&scale_3(&""));
    assert!(parse_result.is_ok());
    expect_false(&input, vec!['A']);
}

#[test]
fn scale_3_2() {
    let mut input = Input::new();
    let parse_result = input.parse_content(&scale_3(&"D"));
    assert!(parse_result.is_ok());
    expect_true(&input, vec!['A']);
}

#[test]
fn scale_3_3() {
    let mut input = Input::new();
    let parse_result = input.parse_content(&scale_3(&"E"));
    assert!(parse_result.is_ok());
    expect_true(&input, vec!['A']);
}

#[test]
fn scale_3_4() {
    let mut input = Input::new();
    let parse_result = input.parse_content(&scale_3(&"DE"));
    assert!(parse_result.is_ok());
    expect_false(&input, vec!['A']);
}

// * Scale 4

fn scale_4(initial_facts: &str) -> String {
    format!(
        "B + !C => A
		={}
		?A",
        initial_facts
    )
}

#[test]
fn scale_4_1() {
    let mut input = Input::new();
    let parse_result = input.parse_content(&scale_4(&""));
    assert!(parse_result.is_ok());
    expect_false(&input, vec!['A']);
}

#[test]
fn scale_4_2() {
    let mut input = Input::new();
    let parse_result = input.parse_content(&scale_4(&"B"));
    assert!(parse_result.is_ok());
    expect_true(&input, vec!['A']);
}

#[test]
fn scale_4_3() {
    let mut input = Input::new();
    let parse_result = input.parse_content(&scale_4(&"C"));
    assert!(parse_result.is_ok());
    expect_false(&input, vec!['A']);
}

#[test]
fn scale_4_4() {
    let mut input = Input::new();
    let parse_result = input.parse_content(&scale_4(&"BC"));
    assert!(parse_result.is_ok());
    expect_false(&input, vec!['A']);
}

// * Scale 5

fn scale_5(initial_facts: &str) -> String {
    format!(
        "B => A
		C => A
		={}
		?A",
        initial_facts
    )
}

#[test]
fn scale_5_1() {
    let mut input = Input::new();
    let parse_result = input.parse_content(&scale_5(&""));
    assert!(parse_result.is_ok());
    expect_false(&input, vec!['A']);
}

#[test]
fn scale_5_2() {
    let mut input = Input::new();
    let parse_result = input.parse_content(&scale_5(&"B"));
    assert!(parse_result.is_ok());
    expect_true(&input, vec!['A']);
}

#[test]
fn scale_5_3() {
    let mut input = Input::new();
    let parse_result = input.parse_content(&scale_5(&"C"));
    assert!(parse_result.is_ok());
    expect_true(&input, vec!['A']);
}

#[test]
fn scale_5_4() {
    let mut input = Input::new();
    let parse_result = input.parse_content(&scale_5(&"BC"));
    assert!(parse_result.is_ok());
    expect_true(&input, vec!['A']);
}

// * Scale 6

fn scale_6(initial_facts: &str) -> String {
    format!(
        "A | B + C => E
		(F | G) + H => E
		={}
		?A",
        initial_facts
    )
}

#[test]
fn scale_6_1() {
    let mut input = Input::new();
    let parse_result = input.parse_content(&scale_6(&""));
    assert!(parse_result.is_ok());
    expect_false(&input, vec!['E']);
}

#[test]
fn scale_6_2() {
    let mut input = Input::new();
    let parse_result = input.parse_content(&scale_6(&"A"));
    assert!(parse_result.is_ok());
    expect_true(&input, vec!['E']);
}

#[test]
fn scale_6_3() {
    let mut input = Input::new();
    let parse_result = input.parse_content(&scale_6(&"B"));
    assert!(parse_result.is_ok());
    expect_false(&input, vec!['E']);
}

#[test]
fn scale_6_4() {
    let mut input = Input::new();
    let parse_result = input.parse_content(&scale_6(&"C"));
    assert!(parse_result.is_ok());
    expect_false(&input, vec!['E']);
}

#[test]
fn scale_6_5() {
    let mut input = Input::new();
    let parse_result = input.parse_content(&scale_6(&"AC"));
    assert!(parse_result.is_ok());
    expect_true(&input, vec!['E']);
}

#[test]
fn scale_6_6() {
    let mut input = Input::new();
    let parse_result = input.parse_content(&scale_6(&"BC"));
    assert!(parse_result.is_ok());
    expect_true(&input, vec!['E']);
}

#[test]
fn scale_6_7() {
    let mut input = Input::new();
    let parse_result = input.parse_content(&scale_6(&"F"));
    assert!(parse_result.is_ok());
    expect_false(&input, vec!['E']);
}

#[test]
fn scale_6_8() {
    let mut input = Input::new();
    let parse_result = input.parse_content(&scale_6(&"G"));
    assert!(parse_result.is_ok());
    expect_false(&input, vec!['E']);
}

#[test]
fn scale_6_9() {
    let mut input = Input::new();
    let parse_result = input.parse_content(&scale_6(&"H"));
    assert!(parse_result.is_ok());
    expect_false(&input, vec!['E']);
}

#[test]
fn scale_6_10() {
    let mut input = Input::new();
    let parse_result = input.parse_content(&scale_6(&"FH"));
    assert!(parse_result.is_ok());
    expect_true(&input, vec!['E']);
}

#[test]
fn scale_6_11() {
    let mut input = Input::new();
    let parse_result = input.parse_content(&scale_6(&"GH"));
    assert!(parse_result.is_ok());
    expect_true(&input, vec!['E']);
}
