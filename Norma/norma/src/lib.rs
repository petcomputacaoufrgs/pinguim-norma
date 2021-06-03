pub mod register;

use num::BigUint;
use register::{Register, RegisterBank};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn test(input: &str) {
    if let Some(input_number) = BigUint::parse_bytes(input.as_bytes(), 10) {
        let mut reg_bank = RegisterBank::new(vec![
            Register::with_value("X", input_number),
            Register::zeroed("Y"),
            Register::zeroed("B"),
        ]);

        let reg_x_index = reg_bank.symbol_to_index("X").expect("I put X there");
        let reg_y_index = reg_bank.symbol_to_index("Y").expect("I put Y there");

        let reg_x = reg_bank.register_mut(reg_x_index);
        reg_x.inc();
        reg_x.inc();
        reg_x.inc();
        let reg_y = reg_bank.register_mut(reg_y_index);
        reg_y.inc();
        reg_y.inc();
        reg_y.dec();

        alert(&reg_bank.to_string());
    }
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, norma!");
}
