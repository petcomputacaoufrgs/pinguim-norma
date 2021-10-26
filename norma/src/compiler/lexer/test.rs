use super::{
    super::{
        error::Diagnostics,
        position::{Position, Span},
    },
    generate_tokens,
    token::{BuiltInOperation, BuiltInTest, Token, TokenType},
};

#[test]
fn empty_src() {
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens("", &mut diagnostics);
    assert!(diagnostics.is_ok());
    assert_eq!(tokens, Vec::new());
}

#[test]
fn single_main() {
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens(" main ", &mut diagnostics);
    assert!(diagnostics.is_ok());
    assert_eq!(
        tokens,
        &[Token {
            token_type: TokenType::Main,
            content: "main".to_owned(),
            span: Span {
                start: Position {
                    line: 1,
                    column: 2,
                    utf8_index: 1,
                    utf16_index: 1,
                },
                end: Position {
                    line: 1,
                    column: 6,
                    utf8_index: 5,
                    utf16_index: 5,
                },
            },
        }]
    );
}

#[test]
fn single_open_curly_space_before() {
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens(" {", &mut diagnostics);
    assert_eq!(
        tokens,
        &[Token {
            token_type: TokenType::OpenCurly,
            content: "{".to_owned(),
            span: Span {
                start: Position {
                    line: 1,
                    column: 2,
                    utf8_index: 1,
                    utf16_index: 1,
                },
                end: Position {
                    line: 1,
                    column: 3,
                    utf8_index: 2,
                    utf16_index: 2,
                },
            }
        }]
    );
    assert!(diagnostics.is_ok());
}

#[test]
fn single_close_curly_space_after() {
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens("} ", &mut diagnostics);
    assert!(diagnostics.is_ok());
    assert_eq!(
        tokens,
        &[Token {
            token_type: TokenType::CloseCurly,
            content: "}".to_owned(),
            span: Span {
                start: Position {
                    line: 1,
                    column: 1,
                    utf8_index: 0,
                    utf16_index: 0,
                },
                end: Position {
                    line: 1,
                    column: 2,
                    utf8_index: 1,
                    utf16_index: 1,
                },
            },
        }]
    );
}

#[test]
fn single_open_paren_space_around() {
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens(" ( ", &mut diagnostics);
    assert!(diagnostics.is_ok());
    assert_eq!(
        tokens,
        &[Token {
            token_type: TokenType::OpenParen,
            content: "(".to_owned(),
            span: Span {
                start: Position {
                    line: 1,
                    column: 2,
                    utf8_index: 1,
                    utf16_index: 1,
                },
                end: Position {
                    line: 1,
                    column: 3,
                    utf8_index: 2,
                    utf16_index: 2,
                },
            },
        }]
    );
    assert!(diagnostics.is_ok());
}

#[test]
fn single_close_paren_many_spaces_before() {
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens("     )", &mut diagnostics);
    assert!(diagnostics.is_ok());
    assert_eq!(
        tokens,
        &[Token {
            token_type: TokenType::CloseParen,
            content: ")".to_owned(),
            span: Span {
                start: Position {
                    line: 1,
                    column: 6,
                    utf8_index: 5,
                    utf16_index: 5,
                },
                end: Position {
                    line: 1,
                    column: 7,
                    utf8_index: 6,
                    utf16_index: 6,
                },
            },
        }]
    );
}

#[test]
fn single_colon_many_spaces_after() {
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens(":         ", &mut diagnostics);
    assert!(diagnostics.is_ok());
    assert_eq!(
        tokens,
        &[Token {
            token_type: TokenType::Colon,
            content: ":".to_owned(),
            span: Span {
                start: Position {
                    line: 1,
                    column: 1,
                    utf8_index: 0,
                    utf16_index: 0,
                },
                end: Position {
                    line: 1,
                    column: 2,
                    utf8_index: 1,
                    utf16_index: 1,
                },
            },
        }]
    );
    assert!(diagnostics.is_ok());
}

#[test]
fn single_add_many_spaces_around() {
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens("   add       ", &mut diagnostics);
    assert!(diagnostics.is_ok());
    assert_eq!(
        tokens,
        &[Token {
            token_type: TokenType::Identifier,
            content: "add".to_owned(),
            span: Span {
                start: Position {
                    line: 1,
                    column: 4,
                    utf8_index: 3,
                    utf16_index: 3,
                },
                end: Position {
                    line: 1,
                    column: 7,
                    utf8_index: 6,
                    utf16_index: 6,
                },
            },
        }]
    );
}

#[test]
fn empty_main() {
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens("main {}", &mut diagnostics);
    assert!(diagnostics.is_ok());
    assert_eq!(
        tokens,
        &[
            Token {
                token_type: TokenType::Main,
                content: "main".to_owned(),
                span: Span {
                    start: Position {
                        line: 1,
                        column: 1,
                        utf8_index: 0,
                        utf16_index: 0,
                    },
                    end: Position {
                        line: 1,
                        column: 5,
                        utf8_index: 4,
                        utf16_index: 4,
                    },
                },
            },
            Token {
                token_type: TokenType::OpenCurly,
                content: "{".to_owned(),
                span: Span {
                    start: Position {
                        line: 1,
                        column: 6,
                        utf8_index: 5,
                        utf16_index: 5,
                    },
                    end: Position {
                        line: 1,
                        column: 7,
                        utf8_index: 6,
                        utf16_index: 6,
                    },
                },
            },
            Token {
                token_type: TokenType::CloseCurly,
                content: "}".to_owned(),
                span: Span {
                    start: Position {
                        line: 1,
                        column: 7,
                        utf8_index: 6,
                        utf16_index: 6,
                    },
                    end: Position {
                        line: 1,
                        column: 8,
                        utf8_index: 7,
                        utf16_index: 7,
                    },
                },
            },
        ]
    );
}

#[test]
fn comments() {
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens(
        "//here\ngoto 3\n//hello\nif zero\n  //world\ndo inc",
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
                    start: Position {
                        line: 2,
                        column: 1,
                        utf8_index: 7,
                        utf16_index: 7,
                    },
                    end: Position {
                        line: 2,
                        column: 5,
                        utf8_index: 11,
                        utf16_index: 11,
                    },
                },
            },
            Token {
                token_type: TokenType::Number,
                content: "3".to_owned(),
                span: Span {
                    start: Position {
                        line: 2,
                        column: 6,
                        utf8_index: 12,
                        utf16_index: 12,
                    },
                    end: Position {
                        line: 2,
                        column: 7,
                        utf8_index: 13,
                        utf16_index: 13,
                    },
                },
            },
            Token {
                token_type: TokenType::If,
                content: "if".to_owned(),
                span: Span {
                    start: Position {
                        line: 4,
                        column: 1,
                        utf8_index: 22,
                        utf16_index: 22,
                    },
                    end: Position {
                        line: 4,
                        column: 3,
                        utf8_index: 24,
                        utf16_index: 24,
                    },
                },
            },
            Token {
                token_type: TokenType::BuiltInTest(BuiltInTest::Zero),
                content: "zero".to_owned(),
                span: Span {
                    start: Position {
                        line: 4,
                        column: 4,
                        utf8_index: 25,
                        utf16_index: 25,
                    },
                    end: Position {
                        line: 4,
                        column: 8,
                        utf8_index: 29,
                        utf16_index: 29,
                    },
                },
            },
            Token {
                token_type: TokenType::Do,
                content: "do".to_owned(),
                span: Span {
                    start: Position {
                        line: 6,
                        column: 1,
                        utf8_index: 40,
                        utf16_index: 40,
                    },
                    end: Position {
                        line: 6,
                        column: 3,
                        utf8_index: 42,
                        utf16_index: 42,
                    },
                },
            },
            Token {
                token_type: TokenType::BuiltInOper(BuiltInOperation::Inc),
                content: "inc".to_owned(),
                span: Span {
                    start: Position {
                        line: 6,
                        column: 4,
                        utf8_index: 43,
                        utf16_index: 43,
                    },
                    end: Position {
                        line: 6,
                        column: 7,
                        utf8_index: 46,
                        utf16_index: 46,
                    },
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
    let tokens = generate_tokens(&source, &mut diagnostics);
    assert!(diagnostics.is_ok());
    eprintln!("{:#?}", tokens);
    assert_eq!(
        tokens,
        &[
            Token {
                token_type: TokenType::Main,
                content: "main".to_owned(),
                span: Span {
                    start: Position {
                        line: 1,
                        column: 1,
                        utf8_index: 0,
                        utf16_index: 0,
                    },
                    end: Position {
                        line: 1,
                        column: 5,
                        utf8_index: 4,
                        utf16_index: 4,
                    },
                },
            },
            Token {
                token_type: TokenType::OpenCurly,
                content: "{".to_owned(),
                span: Span {
                    start: Position {
                        line: 1,
                        column: 6,
                        utf8_index: 5,
                        utf16_index: 5,
                    },
                    end: Position {
                        line: 1,
                        column: 7,
                        utf8_index: 6,
                        utf16_index: 6,
                    },
                },
            },
            Token {
                token_type: TokenType::Number,
                content: "1".to_owned(),
                span: Span {
                    start: Position {
                        line: 2,
                        column: 2,
                        utf8_index: 8,
                        utf16_index: 8,
                    },
                    end: Position {
                        line: 2,
                        column: 3,
                        utf8_index: 9,
                        utf16_index: 9,
                    },
                },
            },
            Token {
                token_type: TokenType::Colon,
                content: ":".to_owned(),
                span: Span {
                    start: Position {
                        line: 2,
                        column: 3,
                        utf8_index: 9,
                        utf16_index: 9,
                    },
                    end: Position {
                        line: 2,
                        column: 4,
                        utf8_index: 10,
                        utf16_index: 10,
                    },
                },
            },
            Token {
                token_type: TokenType::If,
                content: "if".to_owned(),
                span: Span {
                    start: Position {
                        line: 2,
                        column: 5,
                        utf8_index: 11,
                        utf16_index: 11,
                    },
                    end: Position {
                        line: 2,
                        column: 7,
                        utf8_index: 13,
                        utf16_index: 13,
                    },
                },
            },
            Token {
                token_type: TokenType::BuiltInTest(BuiltInTest::Zero),
                content: "zero".to_owned(),
                span: Span {
                    start: Position {
                        line: 2,
                        column: 8,
                        utf8_index: 14,
                        utf16_index: 14,
                    },
                    end: Position {
                        line: 2,
                        column: 12,
                        utf8_index: 18,
                        utf16_index: 18,
                    },
                },
            },
            Token {
                token_type: TokenType::Identifier,
                content: "X".to_owned(),
                span: Span {
                    start: Position {
                        line: 2,
                        column: 13,
                        utf8_index: 19,
                        utf16_index: 19,
                    },
                    end: Position {
                        line: 2,
                        column: 14,
                        utf8_index: 20,
                        utf16_index: 20,
                    },
                },
            },
            Token {
                token_type: TokenType::Then,
                content: "then".to_owned(),
                span: Span {
                    start: Position {
                        line: 2,
                        column: 15,
                        utf8_index: 21,
                        utf16_index: 21,
                    },
                    end: Position {
                        line: 2,
                        column: 19,
                        utf8_index: 25,
                        utf16_index: 25,
                    },
                },
            },
            Token {
                token_type: TokenType::Goto,
                content: "goto".to_owned(),
                span: Span {
                    start: Position {
                        line: 2,
                        column: 20,
                        utf8_index: 26,
                        utf16_index: 26,
                    },
                    end: Position {
                        line: 2,
                        column: 24,
                        utf8_index: 30,
                        utf16_index: 30,
                    },
                },
            },
            Token {
                token_type: TokenType::Number,
                content: "0".to_owned(),
                span: Span {
                    start: Position {
                        line: 2,
                        column: 25,
                        utf8_index: 31,
                        utf16_index: 31,
                    },
                    end: Position {
                        line: 2,
                        column: 26,
                        utf8_index: 32,
                        utf16_index: 32,
                    },
                },
            },
            Token {
                token_type: TokenType::Else,
                content: "else".to_owned(),
                span: Span {
                    start: Position {
                        line: 2,
                        column: 27,
                        utf8_index: 33,
                        utf16_index: 33,
                    },
                    end: Position {
                        line: 2,
                        column: 31,
                        utf8_index: 37,
                        utf16_index: 37,
                    },
                },
            },
            Token {
                token_type: TokenType::Goto,
                content: "goto".to_owned(),
                span: Span {
                    start: Position {
                        line: 2,
                        column: 32,
                        utf8_index: 38,
                        utf16_index: 38,
                    },
                    end: Position {
                        line: 2,
                        column: 36,
                        utf8_index: 42,
                        utf16_index: 42,
                    },
                },
            },
            Token {
                token_type: TokenType::Number,
                content: "2".to_owned(),
                span: Span {
                    start: Position {
                        line: 2,
                        column: 37,
                        utf8_index: 43,
                        utf16_index: 43,
                    },
                    end: Position {
                        line: 2,
                        column: 38,
                        utf8_index: 44,
                        utf16_index: 44,
                    },
                },
            },
            Token {
                token_type: TokenType::Number,
                content: "2".to_owned(),
                span: Span {
                    start: Position {
                        line: 3,
                        column: 2,
                        utf8_index: 46,
                        utf16_index: 46,
                    },
                    end: Position {
                        line: 3,
                        column: 3,
                        utf8_index: 47,
                        utf16_index: 47,
                    },
                },
            },
            Token {
                token_type: TokenType::Colon,
                content: ":".to_owned(),
                span: Span {
                    start: Position {
                        line: 3,
                        column: 3,
                        utf8_index: 47,
                        utf16_index: 47,
                    },
                    end: Position {
                        line: 3,
                        column: 4,
                        utf8_index: 48,
                        utf16_index: 48,
                    },
                },
            },
            Token {
                token_type: TokenType::Do,
                content: "do".to_owned(),
                span: Span {
                    start: Position {
                        line: 3,
                        column: 5,
                        utf8_index: 49,
                        utf16_index: 49,
                    },
                    end: Position {
                        line: 3,
                        column: 7,
                        utf8_index: 51,
                        utf16_index: 51,
                    },
                },
            },
            Token {
                token_type: TokenType::BuiltInOper(BuiltInOperation::Inc),
                content: "inc".to_owned(),
                span: Span {
                    start: Position {
                        line: 3,
                        column: 8,
                        utf8_index: 52,
                        utf16_index: 52,
                    },
                    end: Position {
                        line: 3,
                        column: 11,
                        utf8_index: 55,
                        utf16_index: 55,
                    },
                },
            },
            Token {
                token_type: TokenType::Identifier,
                content: "Y".to_owned(),
                span: Span {
                    start: Position {
                        line: 3,
                        column: 12,
                        utf8_index: 56,
                        utf16_index: 56,
                    },
                    end: Position {
                        line: 3,
                        column: 13,
                        utf8_index: 57,
                        utf16_index: 57,
                    },
                },
            },
            Token {
                token_type: TokenType::Goto,
                content: "goto".to_owned(),
                span: Span {
                    start: Position {
                        line: 3,
                        column: 14,
                        utf8_index: 58,
                        utf16_index: 58,
                    },
                    end: Position {
                        line: 3,
                        column: 18,
                        utf8_index: 62,
                        utf16_index: 62,
                    },
                },
            },
            Token {
                token_type: TokenType::Number,
                content: "3".to_owned(),
                span: Span {
                    start: Position {
                        line: 3,
                        column: 19,
                        utf8_index: 63,
                        utf16_index: 63,
                    },
                    end: Position {
                        line: 3,
                        column: 20,
                        utf8_index: 64,
                        utf16_index: 64,
                    },
                },
            },
            Token {
                token_type: TokenType::Number,
                content: "3".to_owned(),
                span: Span {
                    start: Position {
                        line: 4,
                        column: 2,
                        utf8_index: 66,
                        utf16_index: 66,
                    },
                    end: Position {
                        line: 4,
                        column: 3,
                        utf8_index: 67,
                        utf16_index: 67,
                    },
                },
            },
            Token {
                token_type: TokenType::Colon,
                content: ":".to_owned(),
                span: Span {
                    start: Position {
                        line: 4,
                        column: 3,
                        utf8_index: 67,
                        utf16_index: 67,
                    },
                    end: Position {
                        line: 4,
                        column: 4,
                        utf8_index: 68,
                        utf16_index: 68,
                    },
                },
            },
            Token {
                token_type: TokenType::Do,
                content: "do".to_owned(),
                span: Span {
                    start: Position {
                        line: 4,
                        column: 5,
                        utf8_index: 69,
                        utf16_index: 69,
                    },
                    end: Position {
                        line: 4,
                        column: 7,
                        utf8_index: 71,
                        utf16_index: 71,
                    },
                },
            },
            Token {
                token_type: TokenType::BuiltInOper(BuiltInOperation::Dec),
                content: "dec".to_owned(),
                span: Span {
                    start: Position {
                        line: 4,
                        column: 8,
                        utf8_index: 72,
                        utf16_index: 72,
                    },
                    end: Position {
                        line: 4,
                        column: 11,
                        utf8_index: 75,
                        utf16_index: 75,
                    },
                },
            },
            Token {
                token_type: TokenType::Identifier,
                content: "X".to_owned(),
                span: Span {
                    start: Position {
                        line: 4,
                        column: 12,
                        utf8_index: 76,
                        utf16_index: 76,
                    },
                    end: Position {
                        line: 4,
                        column: 13,
                        utf8_index: 77,
                        utf16_index: 77,
                    },
                },
            },
            Token {
                token_type: TokenType::Goto,
                content: "goto".to_owned(),
                span: Span {
                    start: Position {
                        line: 4,
                        column: 14,
                        utf8_index: 78,
                        utf16_index: 78,
                    },
                    end: Position {
                        line: 4,
                        column: 18,
                        utf8_index: 82,
                        utf16_index: 82,
                    },
                },
            },
            Token {
                token_type: TokenType::Number,
                content: "1".to_owned(),
                span: Span {
                    start: Position {
                        line: 4,
                        column: 19,
                        utf8_index: 83,
                        utf16_index: 83,
                    },
                    end: Position {
                        line: 4,
                        column: 20,
                        utf8_index: 84,
                        utf16_index: 84,
                    },
                },
            },
            Token {
                token_type: TokenType::CloseCurly,
                content: "}".to_owned(),
                span: Span {
                    start: Position {
                        line: 5,
                        column: 1,
                        utf8_index: 85,
                        utf16_index: 85,
                    },
                    end: Position {
                        line: 5,
                        column: 2,
                        utf8_index: 86,
                        utf16_index: 86,
                    },
                },
            },
        ]
    );
}

#[test]
fn invalid_char() {
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens("main${}", &mut diagnostics);
    assert!(diagnostics.is_err());

    let errors =
        diagnostics.iter().map(ToString::to_string).collect::<Vec<_>>();

    assert_eq!(errors, &["Caracter '$' é inválido, na linha 1 e coluna 5"]);

    assert_eq!(
        tokens,
        &[
            Token {
                token_type: TokenType::Main,
                content: "main".to_owned(),
                span: Span {
                    start: Position {
                        line: 1,
                        column: 1,
                        utf8_index: 0,
                        utf16_index: 0,
                    },
                    end: Position {
                        line: 1,
                        column: 5,
                        utf8_index: 4,
                        utf16_index: 4,
                    },
                },
            },
            Token {
                token_type: TokenType::OpenCurly,
                content: "{".to_owned(),
                span: Span {
                    start: Position {
                        line: 1,
                        column: 6,
                        utf8_index: 5,
                        utf16_index: 5,
                    },
                    end: Position {
                        line: 1,
                        column: 7,
                        utf8_index: 6,
                        utf16_index: 6,
                    },
                },
            },
            Token {
                token_type: TokenType::CloseCurly,
                content: "}".to_owned(),
                span: Span {
                    start: Position {
                        line: 1,
                        column: 7,
                        utf8_index: 6,
                        utf16_index: 6,
                    },
                    end: Position {
                        line: 1,
                        column: 8,
                        utf8_index: 7,
                        utf16_index: 7,
                    },
                },
            },
        ]
    );
}

#[test]
fn invalid_comment_start() {
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens("if /a/b\nc", &mut diagnostics);
    assert!(diagnostics.is_err());

    let errors =
        diagnostics.iter().map(ToString::to_string).collect::<Vec<_>>();

    assert_eq!(
        errors,
        &["Começo inválido de comentário, na linha 1 e coluna 4"]
    );

    assert_eq!(
        tokens,
        &[
            Token {
                token_type: TokenType::If,
                content: "if".to_owned(),
                span: Span {
                    start: Position {
                        line: 1,
                        column: 1,
                        utf8_index: 0,
                        utf16_index: 0,
                    },
                    end: Position {
                        line: 1,
                        column: 3,
                        utf8_index: 2,
                        utf16_index: 2,
                    },
                },
            },
            Token {
                token_type: TokenType::Identifier,
                content: "c".to_owned(),
                span: Span {
                    start: Position {
                        line: 2,
                        column: 1,
                        utf8_index: 8,
                        utf16_index: 8,
                    },
                    end: Position {
                        line: 2,
                        column: 2,
                        utf8_index: 9,
                        utf16_index: 9,
                    },
                },
            },
        ]
    );
}

#[test]
fn many_errors() {
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens("foo@{#}/a\nXY", &mut diagnostics);
    assert!(diagnostics.is_err());

    let errors =
        diagnostics.iter().map(ToString::to_string).collect::<Vec<_>>();

    assert_eq!(
        errors,
        &[
            "Caracter '@' é inválido, na linha 1 e coluna 4",
            "Caracter '#' é inválido, na linha 1 e coluna 6",
            "Começo inválido de comentário, na linha 1 e coluna 8"
        ]
    );

    assert_eq!(
        tokens,
        &[
            Token {
                token_type: TokenType::Identifier,
                content: "foo".to_owned(),
                span: Span {
                    start: Position {
                        line: 1,
                        column: 1,
                        utf8_index: 0,
                        utf16_index: 0,
                    },
                    end: Position {
                        line: 1,
                        column: 4,
                        utf8_index: 3,
                        utf16_index: 3,
                    },
                },
            },
            Token {
                token_type: TokenType::OpenCurly,
                content: "{".to_owned(),
                span: Span {
                    start: Position {
                        line: 1,
                        column: 5,
                        utf8_index: 4,
                        utf16_index: 4,
                    },
                    end: Position {
                        line: 1,
                        column: 6,
                        utf8_index: 5,
                        utf16_index: 5,
                    },
                },
            },
            Token {
                token_type: TokenType::CloseCurly,
                content: "}".to_owned(),
                span: Span {
                    start: Position {
                        line: 1,
                        column: 7,
                        utf8_index: 6,
                        utf16_index: 6,
                    },
                    end: Position {
                        line: 1,
                        column: 8,
                        utf8_index: 7,
                        utf16_index: 7,
                    },
                },
            },
            Token {
                token_type: TokenType::Identifier,
                content: "XY".to_owned(),
                span: Span {
                    start: Position {
                        line: 2,
                        column: 1,
                        utf8_index: 10,
                        utf16_index: 10,
                    },
                    end: Position {
                        line: 2,
                        column: 3,
                        utf8_index: 12,
                        utf16_index: 12,
                    },
                },
            },
        ]
    );
}
