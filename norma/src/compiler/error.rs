use crate::compiler::token::{Span, TokenType};
use std::{error::Error as StdError, fmt, slice, vec};

#[derive(Debug)]
pub struct Error {
    cause: Box<dyn StdError + Send + Sync>,
    span: Option<Span>,
}

impl Error {
    pub fn new<E>(cause: E, span: Span) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        Self { cause: Box::new(cause), span: Some(span) }
    }

    pub fn with_no_span<E>(cause: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        Self { cause: Box::new(cause), span: None }
    }

    pub fn span(&self) -> Option<Span> {
        self.span
    }

    pub fn cause(&self) -> &(dyn StdError + Send + Sync + 'static) {
        &*self.cause
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.cause)?;
        if let Some(span) = self.span() {
            write!(formatter, ", {}", span)?;
        }
        Ok(())
    }
}

impl StdError for Error {}

#[derive(Debug, Default)]
pub struct Diagnostics {
    errors: Vec<Error>,
}

impl Diagnostics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn raise(&mut self, error: Error) {
        self.errors.push(error);
    }

    pub fn is_ok(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn is_err(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn count_errors(&self) -> usize {
        self.errors.len()
    }

    pub fn iter(&self) -> Iter {
        Iter { inner: self.errors.iter() }
    }
}

impl IntoIterator for Diagnostics {
    type Item = Error;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { inner: self.errors.into_iter() }
    }
}

#[derive(Debug)]
pub struct IntoIter {
    inner: vec::IntoIter<Error>,
}

impl Iterator for IntoIter {
    type Item = Error;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl DoubleEndedIterator for IntoIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back()
    }
}

impl ExactSizeIterator for IntoIter {}

#[derive(Debug)]
pub struct Iter<'diagnostics> {
    inner: slice::Iter<'diagnostics, Error>,
}

impl<'diagnostics> Iterator for Iter<'diagnostics> {
    type Item = &'diagnostics Error;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'diagnostics> DoubleEndedIterator for Iter<'diagnostics> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back()
    }
}

impl<'diagnostics> ExactSizeIterator for Iter<'diagnostics> {}

#[derive(Debug, Clone)]
pub struct InvalidChar {
    pub character: char,
}

impl fmt::Display for InvalidChar {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Caracter {:?} é inválido", self.character)
    }
}

impl StdError for InvalidChar {}

#[derive(Debug, Clone)]
pub struct BadCommentStart;

impl fmt::Display for BadCommentStart {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Começo inválido de comentário")
    }
}

impl StdError for BadCommentStart {}

// erros do parser
#[derive(Clone, Debug)]
pub struct MainAlreadyDeclared;

impl fmt::Display for MainAlreadyDeclared {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Main já foi declarada neste programa")
    }
}

impl StdError for MainAlreadyDeclared {}

#[derive(Clone, Debug)]
pub struct MainNotDeclared;

impl fmt::Display for MainNotDeclared {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Main não foi declarada neste programa")
    }
}

impl StdError for MainNotDeclared {}

#[derive(Clone, Debug)]
pub struct MacroAlreadyDeclared {
    pub macro_name: String
}

impl fmt::Display for MacroAlreadyDeclared {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Macro \"{}\" já foi declarada neste programa", self.macro_name)
    }
}

impl StdError for MacroAlreadyDeclared {}

#[derive(Clone, Debug)]
pub struct LabelAlreadyDeclared {
    pub label_name: String
}

impl fmt::Display for LabelAlreadyDeclared {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Macro \"{}\" já foi declarada neste programa", self.label_name)
    }
}

impl StdError for LabelAlreadyDeclared {}

#[derive(Clone, Debug)]
pub struct UnexpectedToken {
    pub expected_types: Vec<TokenType>
}

impl fmt::Display for UnexpectedToken {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Token inesperado encontrado, esperava-se um ")?;

        let count = self.expected_types.len();

        if count > 1 {
            for expected_type in &self.expected_types[..count - 2] {
                write!(formatter, "\"{}\", ", expected_type)?;
            }

            write!(formatter, "\"{}\" ou ", self.expected_types[count-2])?;
        }

        if count > 0 {
            write!(formatter, "\"{}\"", self.expected_types[count-1])?;
        }

        Ok(())
    }
}

impl StdError for UnexpectedToken {}

#[derive(Clone, Debug)]
pub struct UnexpectedEndOfInput;

impl fmt::Display for UnexpectedEndOfInput {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Fim inesperado do código")
    }
}

impl StdError for UnexpectedEndOfInput {}