use num_bigint::BigUint;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

mod machine;
mod interpreter;

use machine::Machine;

// Import javascript functions
#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);

    #[wasm_bindgen(js_namespace= console , js_name = log)]
    fn print_wasm(a: &str);

    #[wasm_bindgen(js_namespace= console , js_name = log)]
    fn print_number(a: usize);
}

/// A Norma machine adapted to be exported for JS
/// registers -> k: name of register, v: value
/// counter -> machine counter
#[derive(Serialize, Deserialize)]
pub struct ExportableMachine {
    pub registers: HashMap<String, String>,
    pub counter: String,
}

impl ExportableMachine {
    pub fn from_machine(mut machine: Machine) -> ExportableMachine {
        return ExportableMachine {
            registers: machine.export_registers(),
            counter: machine.get_counted_steps().to_str_radix(10).to_string(),
        };
    }
}

#[wasm_bindgen]
pub fn vetor(input: &[usize]) {
    for &number in input.iter() {
        print_number(number);
    }
}

#[wasm_bindgen]
pub fn square(input: &str) -> JsValue {
    let mut registers =
        Machine::new(BigUint::parse_bytes(input.as_bytes(), 10).unwrap());
    registers.insert("VAL");
    registers.insert("TMP");
    registers.insert("CNT");

    while !registers.is_zero("X") {
        registers.dec("X");
        registers.inc("VAL");
        registers.inc("CNT");
    }

    while !registers.is_zero("CNT") {
        registers.dec("CNT");

        while !registers.is_zero("VAL") {
            registers.inc("Y");
            registers.inc("TMP");
            registers.dec("VAL");
        }

        while !registers.is_zero("TMP") {
            registers.inc("VAL");
            registers.dec("TMP");
        }
    }

    JsValue::from_serde(&ExportableMachine::from_machine(registers)).unwrap()
}

#[wasm_bindgen]
pub fn test(input: &str) -> JsValue {
    let mut registers =
        Machine::new(BigUint::parse_bytes(input.as_bytes(), 10).unwrap());
    registers.insert("J");
    for _i in 1 ..= 3 {
        registers.dec("X");
        registers.inc("X");
        registers.inc("J");
        registers.inc("J");
    }

    if registers.is_zero("X") {
        registers.inc("Y");
        registers.dec("J");
    }

    JsValue::from_serde(&ExportableMachine::from_machine(registers)).unwrap()
}
