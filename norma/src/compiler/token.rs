use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position {
    pub utf8_index: usize,
    pub utf16_index: usize,
    pub line: u64,
    pub column: u64,
}

impl Default for Position {
    fn default() -> Self {
        Position { utf8_index: 0, utf16_index: 0, line: 1, column: 1 }
    }
}

impl Position {
    fn update_newline(&mut self) {
        self.line += 1;
        self.column = 1;
    }

    fn update_column(&mut self) {
        self.column += 1;
    }

    fn update_indices(&mut self, character: char) {
        self.utf8_index += character.len_utf8();
        self.utf16_index += character.len_utf16();
    }

    pub fn update(&mut self, character: char) {
        self.update_indices(character);
        if character == '\n' {
            self.update_newline();
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
    /// Exclusive
    pub end: Position,
}

impl Default for Span {
    fn default() -> Self {
        Self::from_start(Position::default())
    }
}

impl Span {
    pub fn from_start(start: Position) -> Self {
        Self { start, end: start }
    }

    pub fn update(&mut self, character: char) {
        self.end.update(character);
    }

    pub fn finish(&mut self) {
        self.start = self.end;
    }
}

impl fmt::Display for Span {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let end = Position { column: self.end.column - 1, ..self.end };

        if self.start.line != self.end.line {
            write!(formatter, "de {}, até {}", self.start, end)
        } else if self.start.column + 1 == self.end.column {
            write!(formatter, "na {}", self.start)
        } else {
            write!(formatter, "de {}, até coluna {}", self.start, end.column)
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
    BuiltInOper(BuiltInOperation),
    BuiltInTest(BuiltInTest),
    Number,
    Identifier,
    Colon,
    Comma,
    OpenParen,
    CloseParen,
    OpenCurly,
    CloseCurly,
}

impl fmt::Display for TokenType {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenType::Do => write!(formatter, "do"),
            TokenType::Else => write!(formatter, "else"),
            TokenType::Goto => write!(formatter, "goto"),
            TokenType::If => write!(formatter, "if"),
            TokenType::Main => write!(formatter, "main"),
            TokenType::Operation => write!(formatter, "operation"),
            TokenType::Test => write!(formatter, "test"),
            TokenType::Then => write!(formatter, "then"),
            TokenType::BuiltInOper(builtin_oper) => write!(formatter, "{}", builtin_oper),
            TokenType::BuiltInTest(builtin_test) => write!(formatter, "{}", builtin_test),
            TokenType::Number => write!(formatter, "<número>"),
            TokenType::Identifier => write!(formatter, "<identificador>"),
            TokenType::Colon => write!(formatter, ";"),
            TokenType::Comma => write!(formatter, ","),
            TokenType::OpenParen => write!(formatter, "("),
            TokenType::CloseParen => write!(formatter, ")"),
            TokenType::OpenCurly => write!(formatter, "{{"),
            TokenType::CloseCurly => write!(formatter, "}}"),
        }

    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Token {
    pub token_type: TokenType,
    pub content: String,
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BuiltInOperation {
    Inc,
    Dec,
}

impl fmt::Display for BuiltInOperation {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BuiltInOperation::Inc => write!(formatter, "inc"),
            BuiltInOperation::Dec => write!(formatter, "dec"),
        }

    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BuiltInTest {
    Zero,
}

impl fmt::Display for BuiltInTest {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BuiltInTest::Zero => write!(formatter, "zero"),
        }

    }
}
