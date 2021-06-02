use wasm_bindgen::prelude::*;
use num_bigint::BigUint;
use num_traits::identities::{Zero, One};

mod norma;


// Import javascript functions
#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn test(input: &str) {
    let mut registers = norma::RegisterBank::new(BigUint::parse_bytes(input.as_bytes(),10).unwrap());
    for i in 1..=3 {
        registers.dec("X");
        registers.inc("Y");
        registers.inc("J");
        registers.inc("J");
    }
    let x_value = registers.get_value("X");
    let y_value = registers.get_value("Y");
    let j_value = registers.get_value("J");

    alert(&format!("X: {} \nY: {} \nJ: {}", x_value, y_value, j_value));
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}