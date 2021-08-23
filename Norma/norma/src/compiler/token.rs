use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position {
    pub line: u64,
    pub column: u64,
}

impl Position {
    pub fn update_for_newline(&mut self) {
        self.line = self.line + 1;
        self.column = 1;
    }

    pub fn update_column(&mut self) {
        self.column = self.column + 1;
    }

    pub fn update(&mut self, character: char) {
        if character == '\n' {
            self.update_for_newline();
        } else {
            self.update_column()
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "linha {} e coluna {}", self.line, self.column)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Span {
    pub start: Position,
    /// Inclusive
    pub end: Position,
    pub length: u64,
}

impl Span {
    pub fn from_start(start: Position) -> Self {
        Self {
            start,
            end: Position { line: start.line - 1, column: start.column - 1 },
            length: 0,
        }
    }
}

impl fmt::Display for Span {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        if self.length == 1 {
            write!(formatter, "{}", self.start)
        } else if self.start.line == self.end.line {
            write!(
                formatter,
                "de {}, até coluna {}",
                self.start, self.end.column
            )
        } else {
            write!(formatter, "de {}, até {}", self.start, self.end)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(dead_code)]
pub enum TokenType {
    Do,
    Else,
    Goto,
    If,
    Main,
    Operation,
    Test,
    Then,
    Zero,
    Inc,
    Dec,
    Add,
    Sub,
    Cmp,
    Register,
    Number,
    String,
    Colon,
    Comma,
    OpenParen,
    CloseParen,
    OpenCurly,
    CloseCurly,
    SingleSlash,
    Comment,
    None,
}

// Deixei apenas Add, Sub e Cmp em TokenType por que na escrita da norma pode
// ter mais de uma função com o mesmo nome desde que com parâmetros diferentes,
// aí na hora da tokenização marcamos apenas como Add, Sub ou Cmp e
// depois no parser vemos qual função chamar com base nos parâmetros.

// SingleSlash, Comment e None são apenas enums auxiliares.

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Token {
    pub token_type: TokenType,
    pub content: String,
    pub span: Span,
}
