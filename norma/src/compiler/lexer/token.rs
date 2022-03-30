use pinguim_language::position::Span;
use std::fmt;

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
    BuiltInOper(BuiltInOperation),
    BuiltInTest(BuiltInTest),
    ShortcutOper(ShortcutOperation),
    ShortcutTest(ShortcutTest),
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
            TokenType::BuiltInOper(builtin_oper) => {
                write!(formatter, "{}", builtin_oper)
            }
            TokenType::BuiltInTest(builtin_test) => {
                write!(formatter, "{}", builtin_test)
            }
            TokenType::ShortcutOper(shortcut_oper) => {
                write!(formatter, "{}", shortcut_oper)
            }
            TokenType::ShortcutTest(shortcut_test) => {
                write!(formatter, "{}", shortcut_test)
            }
            TokenType::Number => write!(formatter, "<número>"),
            TokenType::Identifier => write!(formatter, "<identificador>"),
            TokenType::Colon => write!(formatter, ":"),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ShortcutOperation {
    Clear,
    Load,
    AddConst,
    Add,
    SubConst,
    Sub,
}

impl fmt::Display for ShortcutOperation {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ShortcutOperation::Clear => write!(formatter, "clear"),
            ShortcutOperation::Load => write!(formatter, "load"),
            ShortcutOperation::AddConst => write!(formatter, "addc"),
            ShortcutOperation::Add => write!(formatter, "add"),
            ShortcutOperation::SubConst => write!(formatter, "subc"),
            ShortcutOperation::Sub => write!(formatter, "sub"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ShortcutTest {
    EqualsConst,
    Equals,
    LessThanConst,
    LessThan,
}

impl fmt::Display for ShortcutTest {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ShortcutTest::EqualsConst => write!(formatter, "cmpc"),
            ShortcutTest::Equals => write!(formatter, "cmp"),
            ShortcutTest::LessThanConst => write!(formatter, "lessthanc"),
            ShortcutTest::LessThan => write!(formatter, "lessthan"),
        }
    }
}
