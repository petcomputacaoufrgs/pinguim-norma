use crate::compiler::{
    error::Diagnostics, lexer::generate_tokens, parser::parse,
    test::greater_than_one,
};

#[test]
fn parse_greater_than_one() {
    let source_code = greater_than_one::source_code();
    let expected_result = greater_than_one::ast();

    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens(source_code, &mut diagnostics);
    let result = parse(tokens, &mut diagnostics);

    assert_eq!(result, Some(expected_result));
    assert!(diagnostics.is_ok());
}

#[test]
fn do_keyword_calls_builtin_test() {
    let code = "main {
    1: do zero X then goto 2 else goto 0
    2: do inc X then goto 0
}";

    let mut diagnostics = Diagnostics::new();

    let tokens = generate_tokens(code, &mut diagnostics);
    eprintln!("tokens ok");
    let result = parse(tokens, &mut diagnostics);
    eprintln!("parse ok");
    let errors =
        diagnostics.iter().map(ToString::to_string).collect::<Vec<_>>();

    eprintln!("{:#?}", errors);
}

#[test]
fn if_keyword_calls_builtin_operation() {}
