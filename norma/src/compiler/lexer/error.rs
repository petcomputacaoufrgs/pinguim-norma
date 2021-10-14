use std::{error::Error, fmt};

#[derive(Debug, Clone)]
pub struct InvalidChar {
    pub character: char,
}

impl fmt::Display for InvalidChar {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Caracter {:?} é inválido", self.character)
    }
}

impl Error for InvalidChar {}

#[derive(Debug, Clone)]
pub struct BadCommentStart;

impl fmt::Display for BadCommentStart {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Começo inválido de comentário")
    }
}

impl Error for BadCommentStart {}
