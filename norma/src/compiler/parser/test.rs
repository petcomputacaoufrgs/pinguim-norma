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
    1: do zero X goto 2
    2: do inc X then goto 0
}";

    let mut diagnostics = Diagnostics::new();

    let tokens = generate_tokens(code, &mut diagnostics);
    let result = parse(tokens, &mut diagnostics);
    let errors =
        diagnostics.iter().map(ToString::to_string).collect::<Vec<_>>();

    eprintln!("{:#?}", errors);
}

#[test]
fn if_keyword_calls_builtin_operation_inc() {
    let code = "main {
    1: if inc X then goto 2 else goto 0
    2: do inc X then goto 0
}";
    
        let mut diagnostics = Diagnostics::new();
    
        let tokens = generate_tokens(code, &mut diagnostics);
        let result = parse(tokens, &mut diagnostics);
        let errors =
            diagnostics.iter().map(ToString::to_string).collect::<Vec<_>>();
    
        eprintln!("{:#?}", errors);
}

#[test]
fn if_keyword_calls_builtin_operation_dec() {
    let code = "main {
    1: if dec X then goto 2 else goto 0
    2: do dec X then goto 0
}";
    
        let mut diagnostics = Diagnostics::new();
    
        let tokens = generate_tokens(code, &mut diagnostics);
        let result = parse(tokens, &mut diagnostics);
        let errors =
            diagnostics.iter().map(ToString::to_string).collect::<Vec<_>>();
    
        eprintln!("{:#?}", errors);
}

#[test]
fn code_starting_with_unexpected_token_type() {
    let code = "a";
    let mut diagnostics = Diagnostics::new();

    let tokens = generate_tokens(code, &mut diagnostics);
    let result = parse(tokens, &mut diagnostics);
    let errors =
        diagnostics.iter().map(ToString::to_string).collect::<Vec<_>>();

    assert_eq!(
        errors, 
        &[
            "Token inesperado encontrado, esperava-se um \"main\", \"operation\" ou \"test\", na linha 1 e coluna 1", 
            "Main não foi declarada neste programa"
        ]
    )
}

#[test]
fn normal_label_as_true() {
    let code = "main {
    true: do inc X goto 0
}";
    let mut diagnostics = Diagnostics::new();

    let tokens = generate_tokens(code, &mut diagnostics);
    let result = parse(tokens, &mut diagnostics);
    let errors =
        diagnostics.iter().map(ToString::to_string).collect::<Vec<_>>();

    assert_eq!(
        errors, 
        &["Nome de label não pode ser \"true\" nem \"false\", na linha 2 e coluna 5"]
    )
}

#[test]
fn normal_label_as_false() {
    let code = "main {
    false: do inc X goto 0
}";
    let mut diagnostics = Diagnostics::new();

    let tokens = generate_tokens(code, &mut diagnostics);
    let result = parse(tokens, &mut diagnostics);
    let errors =
        diagnostics.iter().map(ToString::to_string).collect::<Vec<_>>();

    assert_eq!(
        errors, 
        &["Nome de label não pode ser \"true\" nem \"false\", na linha 2 e coluna 5"]
    )
}


