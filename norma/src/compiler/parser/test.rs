use crate::compiler::{
    error::Diagnostics,
    lexer::generate_tokens,
    parser::parse,
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
