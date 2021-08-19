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
    None
}

// Deixei apenas Add, Sub e Cmp em TokenType por que na escrita da norma pode ter mais de uma função com o mesmo nome
// desde que com parâmetros diferentes, aí na hora da tokenização marcamos apenas como Add, Sub ou Cmp e 
// depois no parser vemos qual função chamar com base nos parâmetros.

// SingleSlash, Comment e None são apenas enums auxiliares.

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Token {
    pub token_type: TokenType,
    pub content: String,
    pub position: Position,
}
