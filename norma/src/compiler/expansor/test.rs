use crate::compiler::{
    error::Diagnostics, expansor::expand, lexer::generate_tokens,
    parser::parse, test::greater_than_one,
};

#[test]
fn expand_greater_than_one() {
    let ast = greater_than_one::ast();
    let expected = greater_than_one::runtime_program();
    let mut diagnostics = Diagnostics::new();
    let found = expand(&ast, &mut diagnostics).unwrap();

    assert_eq!(found, expected);
}

#[test]
fn expand_recursive_macros() {
    let code = "test foo(A) {
    1: do bar(A) goto false
}

operation bar(B) {
    1: do baz(B) goto 0
}

operation baz(C) {
    1: if foo(C) then goto 0 else goto 2
    2: do simple (C) goto 3
    3: do bar (C) goto 0
}

operation simple(D) {
    1: do inc (D) goto 0
}

main {
    1: if foo(X) then goto 0 else goto 0
}";

    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens(code, &mut diagnostics);
    let ast = parse(tokens, &mut diagnostics).unwrap();
    expand(&ast, &mut diagnostics);
    let errors =
        diagnostics.iter().map(ToString::to_string).collect::<Vec<_>>();

    assert_eq!(
        errors,
        &[
            "Recursão detectada entre as macros: \"foo\", \"bar\" e \"baz\", da linha 10 e coluna 11, até a coluna 13",
            "Recursão detectada entre as macros: \"bar\" e \"baz\", da linha 12 e coluna 11, até a coluna 13"
        ]
    );
}
