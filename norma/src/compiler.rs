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
    let ast = parse(tokens, &mut diagnostics);
    let maybe_program = ast.and_then(|ast| expand(&ast, &mut diagnostics));

    match maybe_program {
        Some(runtime_program) if diagnostics.is_ok() => Ok(runtime_program),
        _ => Err(diagnostics),
    }
}
