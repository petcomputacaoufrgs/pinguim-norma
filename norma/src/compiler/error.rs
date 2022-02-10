use crate::compiler::position::Span;
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
