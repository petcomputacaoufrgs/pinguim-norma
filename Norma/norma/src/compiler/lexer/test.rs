use super::{
    super::{
        error::Diagnostics,
        token::{Position, Span, Token, TokenType},
    },
    generate_tokens,
};

#[test]
fn empty_src() {
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens("".to_owned(), &mut diagnostics);
    assert!(diagnostics.is_ok());
    assert_eq!(tokens, Vec::new());
}

#[test]
fn single_main() {
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens(" main ".to_owned(), &mut diagnostics);
    assert!(diagnostics.is_ok());
    assert_eq!(
        tokens,
        &[Token {
            token_type: TokenType::Main,
            content: "main".to_owned(),
            span: Span {
                start: Position { line: 1, column: 2 },
                end: Position { line: 1, column: 5 },
                length: 4,
            },
        }]
    );
}

#[test]
fn single_open_curly_space_before() {
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens(" {".to_owned(), &mut diagnostics);
    assert_eq!(
        tokens,
        &[Token {
            token_type: TokenType::OpenCurly,
            content: "{".to_owned(),
            span: Span {
                start: Position { line: 1, column: 2 },
                end: Position { line: 1, column: 2 },
                length: 1,
            }
        }]
    );
    assert!(diagnostics.is_ok());
}

#[test]
fn single_close_curly_space_after() {
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens("} ".to_owned(), &mut diagnostics);
    assert!(diagnostics.is_ok());
    assert_eq!(
        tokens,
        &[Token {
            token_type: TokenType::CloseCurly,
            content: "}".to_owned(),
            span: Span {
                start: Position { line: 1, column: 1 },
                end: Position { line: 1, column: 1 },
                length: 1,
            },
        }]
    );
}

#[test]
fn single_open_paren_space_around() {
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens(" ( ".to_owned(), &mut diagnostics);
    assert!(diagnostics.is_ok());
    assert_eq!(
        tokens,
        &[Token {
            token_type: TokenType::OpenParen,
            content: "(".to_owned(),
            span: Span {
                start: Position { line: 1, column: 2 },
                end: Position { line: 1, column: 2 },
                length: 1,
            },
        }]
    );
    assert!(diagnostics.is_ok());
}

#[test]
fn single_close_paren_many_spaces_before() {
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens("     )".to_owned(), &mut diagnostics);
    assert!(diagnostics.is_ok());
    assert_eq!(
        tokens,
        &[Token {
            token_type: TokenType::CloseParen,
            content: ")".to_owned(),
            span: Span {
                start: Position { line: 1, column: 6 },
                end: Position { line: 1, column: 6 },
                length: 1,
            },
        }]
    );
}

#[test]
fn single_colon_many_spaces_after() {
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens(":         ".to_owned(), &mut diagnostics);
    assert!(diagnostics.is_ok());
    assert_eq!(
        tokens,
        &[Token {
            token_type: TokenType::Colon,
            content: ":".to_owned(),
            span: Span {
                start: Position { line: 1, column: 1 },
                end: Position { line: 1, column: 1 },
                length: 1,
            },
        }]
    );
    assert!(diagnostics.is_ok());
}

#[test]
fn single_add_many_spaces_around() {
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens("   add       ".to_owned(), &mut diagnostics);
    assert!(diagnostics.is_ok());
    assert_eq!(
        tokens,
        &[Token {
            token_type: TokenType::Add,
            content: "add".to_owned(),
            span: Span {
                start: Position { line: 1, column: 4 },
                end: Position { line: 1, column: 6 },
                length: 3,
            },
        }]
    );
}

#[test]
fn empty_main() {
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens("main {}".to_owned(), &mut diagnostics);
    assert!(diagnostics.is_ok());
    assert_eq!(
        tokens,
        &[
            Token {
                token_type: TokenType::Main,
                content: "main".to_owned(),
                span: Span {
                    start: Position { line: 1, column: 1 },
                    end: Position { line: 1, column: 4 },
                    length: 4,
                },
            },
            Token {
                token_type: TokenType::OpenCurly,
                content: "{".to_owned(),
                span: Span {
                    start: Position { line: 1, column: 6 },
                    end: Position { line: 1, column: 6 },
                    length: 1,
                },
            },
            Token {
                token_type: TokenType::CloseCurly,
                content: "}".to_owned(),
                span: Span {
                    start: Position { line: 1, column: 7 },
                    end: Position { line: 1, column: 7 },
                    length: 1,
                },
            },
        ]
    );
}

#[test]
fn comments() {
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens(
        "//here\ngoto 3\n//hello\nif zero\n  //world\ndo inc".to_owned(),
        &mut diagnostics,
    );
    assert!(diagnostics.is_ok());

    assert_eq!(
        tokens,
        &[
            Token {
                token_type: TokenType::Goto,
                content: "goto".to_owned(),
                span: Span {
                    start: Position { line: 2, column: 1 },
                    end: Position { line: 2, column: 4 },
                    length: 4,
                },
            },
            Token {
                token_type: TokenType::Number,
                content: "3".to_owned(),
                span: Span {
                    start: Position { line: 2, column: 6 },
                    end: Position { line: 2, column: 6 },
                    length: 1,
                },
            },
            Token {
                token_type: TokenType::If,
                content: "if".to_owned(),
                span: Span {
                    start: Position { line: 4, column: 1 },
                    end: Position { line: 4, column: 2 },
                    length: 2,
                },
            },
            Token {
                token_type: TokenType::Zero,
                content: "zero".to_owned(),
                span: Span {
                    start: Position { line: 4, column: 4 },
                    end: Position { line: 4, column: 7 },
                    length: 4,
                },
            },
            Token {
                token_type: TokenType::Do,
                content: "do".to_owned(),
                span: Span {
                    start: Position { line: 6, column: 1 },
                    end: Position { line: 6, column: 2 },
                    length: 2,
                },
            },
            Token {
                token_type: TokenType::Inc,
                content: "inc".to_owned(),
                span: Span {
                    start: Position { line: 6, column: 4 },
                    end: Position { line: 6, column: 6 },
                    length: 3,
                },
            },
        ]
    );
}

#[test]
fn id_program() {
    let mut diagnostics = Diagnostics::new();
    let mut source = "main {\n".to_owned();
    source.push_str("\t1: if zero X then goto 0 else goto 2\n");
    source.push_str("\t2: do inc Y goto 3\n");
    source.push_str("\t3: do dec X goto 1\n");
    source.push_str("}\n");
    let tokens = generate_tokens(source, &mut diagnostics);
    assert!(diagnostics.is_ok());
    assert_eq!(
        tokens,
        &[
            Token {
                token_type: TokenType::Main,
                content: "main".to_owned(),
                span: Span {
                    start: Position { line: 1, column: 1 },
                    end: Position { line: 1, column: 4 },
                    length: 4
                },
            },
            Token {
                token_type: TokenType::OpenCurly,
                content: "{".to_owned(),
                span: Span {
                    start: Position { line: 1, column: 6 },
                    end: Position { line: 1, column: 6 },
                    length: 1
                },
            },
            Token {
                token_type: TokenType::Number,
                content: "1".to_owned(),
                span: Span {
                    start: Position { line: 2, column: 2 },
                    end: Position { line: 2, column: 2 },
                    length: 1
                },
            },
            Token {
                token_type: TokenType::Colon,
                content: ":".to_owned(),
                span: Span {
                    start: Position { line: 2, column: 3 },
                    end: Position { line: 2, column: 3 },
                    length: 1
                },
            },
            Token {
                token_type: TokenType::If,
                content: "if".to_owned(),
                span: Span {
                    start: Position { line: 2, column: 5 },
                    end: Position { line: 2, column: 6 },
                    length: 2
                },
            },
            Token {
                token_type: TokenType::Zero,
                content: "zero".to_owned(),
                span: Span {
                    start: Position { line: 2, column: 8 },
                    end: Position { line: 2, column: 11 },
                    length: 4
                },
            },
            Token {
                token_type: TokenType::Register,
                content: "X".to_owned(),
                span: Span {
                    start: Position { line: 2, column: 13 },
                    end: Position { line: 2, column: 13 },
                    length: 1
                },
            },
            Token {
                token_type: TokenType::Then,
                content: "then".to_owned(),
                span: Span {
                    start: Position { line: 2, column: 15 },
                    end: Position { line: 2, column: 18 },
                    length: 4
                },
            },
            Token {
                token_type: TokenType::Goto,
                content: "goto".to_owned(),
                span: Span {
                    start: Position { line: 2, column: 20 },
                    end: Position { line: 2, column: 23 },
                    length: 4
                },
            },
            Token {
                token_type: TokenType::Number,
                content: "0".to_owned(),
                span: Span {
                    start: Position { line: 2, column: 25 },
                    end: Position { line: 2, column: 25 },
                    length: 1
                },
            },
            Token {
                token_type: TokenType::Else,
                content: "else".to_owned(),
                span: Span {
                    start: Position { line: 2, column: 27 },
                    end: Position { line: 2, column: 30 },
                    length: 4
                },
            },
            Token {
                token_type: TokenType::Goto,
                content: "goto".to_owned(),
                span: Span {
                    start: Position { line: 2, column: 32 },
                    end: Position { line: 2, column: 35 },
                    length: 4
                },
            },
            Token {
                token_type: TokenType::Number,
                content: "2".to_owned(),
                span: Span {
                    start: Position { line: 2, column: 37 },
                    end: Position { line: 2, column: 37 },
                    length: 1
                },
            },
            Token {
                token_type: TokenType::Number,
                content: "2".to_owned(),
                span: Span {
                    start: Position { line: 3, column: 2 },
                    end: Position { line: 3, column: 2 },
                    length: 1
                },
            },
            Token {
                token_type: TokenType::Colon,
                content: ":".to_owned(),
                span: Span {
                    start: Position { line: 3, column: 3 },
                    end: Position { line: 3, column: 3 },
                    length: 1
                },
            },
            Token {
                token_type: TokenType::Do,
                content: "do".to_owned(),
                span: Span {
                    start: Position { line: 3, column: 5 },
                    end: Position { line: 3, column: 6 },
                    length: 2
                },
            },
            Token {
                token_type: TokenType::Inc,
                content: "inc".to_owned(),
                span: Span {
                    start: Position { line: 3, column: 8 },
                    end: Position { line: 3, column: 10 },
                    length: 3
                },
            },
            Token {
                token_type: TokenType::Register,
                content: "Y".to_owned(),
                span: Span {
                    start: Position { line: 3, column: 12 },
                    end: Position { line: 3, column: 12 },
                    length: 1
                },
            },
            Token {
                token_type: TokenType::Goto,
                content: "goto".to_owned(),
                span: Span {
                    start: Position { line: 3, column: 14 },
                    end: Position { line: 3, column: 17 },
                    length: 4
                },
            },
            Token {
                token_type: TokenType::Number,
                content: "3".to_owned(),
                span: Span {
                    start: Position { line: 3, column: 19 },
                    end: Position { line: 3, column: 19 },
                    length: 1
                },
            },
            Token {
                token_type: TokenType::Number,
                content: "3".to_owned(),
                span: Span {
                    start: Position { line: 4, column: 2 },
                    end: Position { line: 4, column: 2 },
                    length: 1
                },
            },
            Token {
                token_type: TokenType::Colon,
                content: ":".to_owned(),
                span: Span {
                    start: Position { line: 4, column: 3 },
                    end: Position { line: 4, column: 3 },
                    length: 1
                },
            },
            Token {
                token_type: TokenType::Do,
                content: "do".to_owned(),
                span: Span {
                    start: Position { line: 4, column: 5 },
                    end: Position { line: 4, column: 6 },
                    length: 2
                },
            },
            Token {
                token_type: TokenType::Dec,
                content: "dec".to_owned(),
                span: Span {
                    start: Position { line: 4, column: 8 },
                    end: Position { line: 4, column: 10 },
                    length: 3
                },
            },
            Token {
                token_type: TokenType::Register,
                content: "X".to_owned(),
                span: Span {
                    start: Position { line: 4, column: 12 },
                    end: Position { line: 4, column: 12 },
                    length: 1
                },
            },
            Token {
                token_type: TokenType::Goto,
                content: "goto".to_owned(),
                span: Span {
                    start: Position { line: 4, column: 14 },
                    end: Position { line: 4, column: 17 },
                    length: 4
                },
            },
            Token {
                token_type: TokenType::Number,
                content: "1".to_owned(),
                span: Span {
                    start: Position { line: 4, column: 19 },
                    end: Position { line: 4, column: 19 },
                    length: 1
                },
            },
            Token {
                token_type: TokenType::CloseCurly,
                content: "}".to_owned(),
                span: Span {
                    start: Position { line: 5, column: 1 },
                    end: Position { line: 5, column: 1 },
                    length: 1
                },
            },
        ]
    );
}

#[test]
fn invalid_char() {
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens("main${}".to_owned(), &mut diagnostics);
    assert!(diagnostics.is_err());

    let errors =
        diagnostics.iter().map(ToString::to_string).collect::<Vec<_>>();

    assert_eq!(errors, &["Caracter '$' é inválido, linha 1 e coluna 5"]);

    assert_eq!(
        tokens,
        &[
            Token {
                token_type: TokenType::Main,
                content: "main".to_owned(),
                span: Span {
                    start: Position { line: 1, column: 1 },
                    end: Position { line: 1, column: 4 },
                    length: 4,
                },
            },
            Token {
                token_type: TokenType::OpenCurly,
                content: "{".to_owned(),
                span: Span {
                    start: Position { line: 1, column: 6 },
                    end: Position { line: 1, column: 6 },
                    length: 1,
                },
            },
            Token {
                token_type: TokenType::CloseCurly,
                content: "}".to_owned(),
                span: Span {
                    start: Position { line: 1, column: 7 },
                    end: Position { line: 1, column: 7 },
                    length: 1,
                },
            },
        ]
    );
}

#[test]
fn invalid_comment_start() {
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens("if /a/b\nc".to_owned(), &mut diagnostics);
    assert!(diagnostics.is_err());

    let errors =
        diagnostics.iter().map(ToString::to_string).collect::<Vec<_>>();

    assert_eq!(errors, &["Começo inválido de comentário, linha 1 e coluna 5"]);

    assert_eq!(
        tokens,
        &[
            Token {
                token_type: TokenType::If,
                content: "if".to_owned(),
                span: Span {
                    start: Position { line: 1, column: 1 },
                    end: Position { line: 1, column: 2 },
                    length: 2,
                },
            },
            Token {
                token_type: TokenType::String,
                content: "c".to_owned(),
                span: Span {
                    start: Position { line: 2, column: 1 },
                    end: Position { line: 2, column: 1 },
                    length: 1,
                },
            },
        ]
    );
}

#[test]
fn invalid_register() {
    let mut diagnostics = Diagnostics::new();
    let tokens =
        generate_tokens("do inc Yxz goto 31".to_owned(), &mut diagnostics);
    assert!(diagnostics.is_err());

    let errors =
        diagnostics.iter().map(ToString::to_string).collect::<Vec<_>>();

    assert_eq!(
        errors,
        &["Apenas letras maiúsculas são permitidas em registradores, \"Yx\" \
           é um registrador inválido, de linha 1 e coluna 8, até coluna 9"]
    );

    assert_eq!(
        tokens,
        &[
            Token {
                token_type: TokenType::Do,
                content: "do".to_owned(),
                span: Span {
                    start: Position { line: 1, column: 1 },
                    end: Position { line: 1, column: 2 },
                    length: 2,
                },
            },
            Token {
                token_type: TokenType::Inc,
                content: "inc".to_owned(),
                span: Span {
                    start: Position { line: 1, column: 4 },
                    end: Position { line: 1, column: 6 },
                    length: 3,
                },
            },
            Token {
                token_type: TokenType::Register,
                content: "Yxz".to_owned(),
                span: Span {
                    start: Position { line: 1, column: 8 },
                    end: Position { line: 1, column: 10 },
                    length: 3,
                },
            },
            Token {
                token_type: TokenType::Goto,
                content: "goto".to_owned(),
                span: Span {
                    start: Position { line: 1, column: 12 },
                    end: Position { line: 1, column: 15 },
                    length: 4,
                },
            },
            Token {
                token_type: TokenType::Number,
                content: "31".to_owned(),
                span: Span {
                    start: Position { line: 1, column: 17 },
                    end: Position { line: 1, column: 18 },
                    length: 2,
                },
            },
        ]
    );
}

#[test]
fn many_errors() {
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens("foo@{#}Xy".to_owned(), &mut diagnostics);
    assert!(diagnostics.is_err());

    let errors =
        diagnostics.iter().map(ToString::to_string).collect::<Vec<_>>();

    assert_eq!(
        errors,
        &[
            "Caracter '@' é inválido, linha 1 e coluna 4",
            "Caracter '#' é inválido, linha 1 e coluna 6",
            "Apenas letras maiúsculas são permitidas em registradores, \"Xy\" \
             é um registrador inválido, de linha 1 e coluna 8, até coluna 9"
        ]
    );

    assert_eq!(
        tokens,
        &[
            Token {
                token_type: TokenType::String,
                content: "foo".to_owned(),
                span: Span {
                    start: Position { line: 1, column: 1 },
                    end: Position { line: 1, column: 3 },
                    length: 3,
                },
            },
            Token {
                token_type: TokenType::OpenCurly,
                content: "{".to_owned(),
                span: Span {
                    start: Position { line: 1, column: 5 },
                    end: Position { line: 1, column: 5 },
                    length: 1,
                },
            },
            Token {
                token_type: TokenType::CloseCurly,
                content: "}".to_owned(),
                span: Span {
                    start: Position { line: 1, column: 7 },
                    end: Position { line: 1, column: 7 },
                    length: 1,
                },
            },
            Token {
                token_type: TokenType::Register,
                content: "Xy".to_owned(),
                span: Span {
                    start: Position { line: 1, column: 8 },
                    end: Position { line: 1, column: 9 },
                    length: 2,
                },
            },
        ]
    );
}
