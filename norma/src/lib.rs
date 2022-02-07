pub mod compiler;
pub mod machine;
pub mod interpreter;

use compiler::error::Diagnostics;
use interpreter::run_once;
use num_bigint::BigUint;

pub fn run(source: &str, input: BigUint) -> Result<BigUint, Diagnostics> {
    compiler::compile(source).map(|program| run_once(input, program))
}
