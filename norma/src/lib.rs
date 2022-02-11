pub mod compiler;
pub mod machine;
pub mod interpreter;

use interpreter::run_once;
use num_bigint::BigUint;
use pinguim_language::error::Diagnostics;

pub fn run(source: &str, input: BigUint) -> Result<BigUint, Diagnostics> {
    compiler::compile(source).map(|program| run_once(input, program))
}
