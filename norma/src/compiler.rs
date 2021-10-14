pub mod position;
pub mod error;
pub mod lexer;
pub mod parser;
pub mod expansor;

#[cfg(test)]
mod test;

use crate::interpreter::program::Program;
use error::Diagnostics;
use expansor::expand;
use lexer::generate_tokens;
use parser::parse;

pub fn compile(source: &str) -> Result<Program, Diagnostics> {
    let mut diagnostics = Diagnostics::new();
    let tokens = generate_tokens(source, &mut diagnostics);
    let maybe_ast = parse(tokens, &mut diagnostics);
    let maybe_prog = maybe_ast.and_then(|ast| expand(&ast, &mut diagnostics));

    match maybe_prog {
        Some(runtime_program) if diagnostics.is_ok() => Ok(runtime_program),
        _ => Err(diagnostics),
    }
}
