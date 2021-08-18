use super::generate_tokens;
use crate::token::{Position, Token, TokenType};

// ISSUES:
// - just a single token might not be scanned at all
// - line and column should start at 1 perhaps?
// - column should be at the beginning of the token

#[test]
fn empty_src() {
    assert_eq!(generate_tokens("".to_owned()), Vec::new());
}

#[test]
fn single_main() {
    assert_eq!(
        generate_tokens(" main ".to_owned()),
        vec![Token {
            token_type: TokenType::Main,
            content: "main".to_owned(),
            position: Position { line: 1, column: 1 },
        }]
    );
}

#[test]
fn single_open_curly_space_before() {
    assert_eq!(
        generate_tokens(" {".to_owned()),
        vec![Token {
            token_type: TokenType::OpenCurly,
            content: "{".to_owned(),
            position: Position { line: 1, column: 2 },
        }]
    );
}

#[test]
fn single_close_curly_space_after() {
    assert_eq!(
        generate_tokens("} ".to_owned()),
        vec![Token {
            token_type: TokenType::CloseCurly,
            content: "}".to_owned(),
            position: Position { line: 1, column: 1 },
        }]
    );
}

#[test]
fn single_open_paren_space_around() {
    assert_eq!(
        generate_tokens(" ( ".to_owned()),
        vec![Token {
            token_type: TokenType::OpenParen,
            content: "(".to_owned(),
            position: Position { line: 1, column: 2 },
        }]
    );
}

#[test]
fn single_close_paren_many_spaces_before() {
    assert_eq!(
        generate_tokens("     )".to_owned()),
        vec![Token {
            token_type: TokenType::CloseCurly,
            content: ")".to_owned(),
            position: Position { line: 1, column: 6 },
        }]
    );
}

#[test]
fn single_close_paren_many_spaces_after() {
    assert_eq!(
        generate_tokens(":         ".to_owned()),
        vec![Token {
            token_type: TokenType::Colon,
            content: ":".to_owned(),
            position: Position { line: 1, column: 1 },
        }]
    );
}

#[test]
fn single_close_paren_many_spaces_around() {
    assert_eq!(
        generate_tokens("   ;       ".to_owned()),
        vec![Token {
            token_type: TokenType::Colon,
            content: ";".to_owned(),
            position: Position { line: 1, column: 4 },
        }]
    );
}

#[test]
fn empty_main() {
    assert_eq!(
        generate_tokens("main {}".to_owned()),
        vec![
            Token {
                token_type: TokenType::Main,
                content: "main".to_owned(),
                position: Position { line: 1, column: 1 },
            },
            Token {
                token_type: TokenType::OpenCurly,
                content: "{".to_owned(),
                position: Position { line: 1, column: 6 },
            },
            Token {
                token_type: TokenType::CloseCurly,
                content: "}".to_owned(),
                position: Position { line: 1, column: 7 },
            },
        ]
    );
}

#[test]
fn id_program() {
    let mut source = "main {\n".to_owned();
    source.push_str("\t1: if zero X then goto 0 else goto 2\n");
    source.push_str("\t2: do inc Y goto 3\n");
    source.push_str("\t3: do dec X goto 1\n");
    source.push_str("}\n");
    assert_eq!(
        generate_tokens(source),
        vec![
            Token {
                token_type: TokenType::Main,
                content: "main".to_owned(),
                position: Position { line: 1, column: 1 },
            },
            Token {
                token_type: TokenType::OpenCurly,
                content: "{".to_owned(),
                position: Position { line: 1, column: 6 },
            },
            Token {
                token_type: TokenType::Number,
                content: "1".to_owned(),
                position: Position { line: 2, column: 2 },
            },
            Token {
                token_type: TokenType::Colon,
                content: ":".to_owned(),
                position: Position { line: 2, column: 3 },
            },
            Token {
                token_type: TokenType::If,
                content: "if".to_owned(),
                position: Position { line: 2, column: 5 },
            },
            Token {
                token_type: TokenType::Zero,
                content: "zero".to_owned(),
                position: Position { line: 2, column: 8 },
            },
            Token {
                token_type: TokenType::Register,
                content: "X".to_owned(),
                position: Position { line: 2, column: 13 },
            },
            Token {
                token_type: TokenType::Then,
                content: "then".to_owned(),
                position: Position { line: 2, column: 15 },
            },
            Token {
                token_type: TokenType::Goto,
                content: "goto".to_owned(),
                position: Position { line: 2, column: 20 },
            },
            Token {
                token_type: TokenType::Number,
                content: "0".to_owned(),
                position: Position { line: 2, column: 25 },
            },
            Token {
                token_type: TokenType::Else,
                content: "else".to_owned(),
                position: Position { line: 2, column: 22 },
            },
            Token {
                token_type: TokenType::Goto,
                content: "goto".to_owned(),
                position: Position { line: 2, column: 27 },
            },
            Token {
                token_type: TokenType::Number,
                content: "2".to_owned(),
                position: Position { line: 2, column: 32 },
            },
            Token {
                token_type: TokenType::CloseCurly,
                content: "}".to_owned(),
                position: Position { line: 5, column: 1 },
            },
        ]
    );
}
