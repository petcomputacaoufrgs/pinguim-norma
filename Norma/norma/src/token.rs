#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position {
    pub line: u64,
    pub column: u64,
}

impl Position {
    pub fn update_for_newline(mut self) {
        self.line += 1;
        self.column = 1;
    }

    pub fn update_column(mut self) {
        self.column += 1;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TokenType {
    Do,
    Else,
    Goto,
    If,
    Main,
    Operation,
    Test,
    Then,
    Register,
    Number,
    String,
    Colon,
    Comma,
    Semicolon,
    OpenParen,
    CloseParen,
    OpenCurly,
    CloseCurly,
    SingleSlash,
    Comment,
    None
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Token {
    pub token_type: TokenType,
    pub content: String,
    pub position: Position,
}
