use crate::compiler::lexer::token::TokenType;
use std::{error::Error, fmt};

#[derive(Clone, Debug)]
pub struct MainAlreadyDeclared;

impl fmt::Display for MainAlreadyDeclared {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Main já foi declarada neste programa")
    }
}

impl Error for MainAlreadyDeclared {}

#[derive(Clone, Debug)]
pub struct MainNotDeclared;

impl fmt::Display for MainNotDeclared {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Main não foi declarada neste programa")
    }
}

impl Error for MainNotDeclared {}

#[derive(Clone, Debug)]
pub struct MacroAlreadyDeclared {
    pub macro_name: String,
}

impl fmt::Display for MacroAlreadyDeclared {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "Macro \"{}\" já foi declarada neste programa",
            self.macro_name
        )
    }
}

impl Error for MacroAlreadyDeclared {}

#[derive(Clone, Debug)]
pub struct LabelAlreadyDeclared {
    pub label_name: String,
}

impl fmt::Display for LabelAlreadyDeclared {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "Macro \"{}\" já foi declarada neste programa",
            self.label_name
        )
    }
}

impl Error for LabelAlreadyDeclared {}

#[derive(Clone, Debug)]
pub struct UnexpectedToken {
    pub expected_types: Vec<TokenType>,
}

impl fmt::Display for UnexpectedToken {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Token inesperado encontrado, esperava-se um ")?;

        let count = self.expected_types.len();

        if count > 1 {
            for expected_type in &self.expected_types[.. count - 2] {
                write!(formatter, "\"{}\", ", expected_type)?;
            }

            write!(formatter, "\"{}\" ou ", self.expected_types[count - 2])?;
        }

        if count > 0 {
            write!(formatter, "\"{}\"", self.expected_types[count - 1])?;
        }

        Ok(())
    }
}

impl Error for UnexpectedToken {}

#[derive(Clone, Debug)]
pub struct UnexpectedEndOfInput;

impl fmt::Display for UnexpectedEndOfInput {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Fim inesperado do código")
    }
}

impl Error for UnexpectedEndOfInput {}
