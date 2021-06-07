use wasm_bindgen::prelude::*;
use num_bigint::BigUint;

mod norma;


// Import javascript functions
#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn test(input: &str) {
    let mut registers = norma::RegisterBank::new(BigUint::parse_bytes(input.as_bytes(),10).unwrap());
    registers.insert("J");
    for _i in 1..=3 {
        registers.apply("X", |reg| {
            reg.dec();
        });
        registers.apply("Y", |reg| {
            reg.inc();
        });
        registers.apply("J", |reg| {
            reg.inc();
            reg.inc();
        });
    }

    if registers.is_zero("X") {
        registers.inc("Y");
        registers.dec("J");   
    }
    
    let x_value = registers.get_value("X");
    let y_value = registers.get_value("Y");
    let j_value = registers.get_value("J");
    let counter = registers.get_counter();

    alert(&format!("X: {} \nY: {} \nJ: {}", x_value, y_value, j_value));
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}