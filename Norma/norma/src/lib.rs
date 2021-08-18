use wasm_bindgen::prelude::*;
use std::collections::HashMap;
use num_bigint::BigUint;
use serde::{Serialize, Deserialize};

mod norma;
mod lexer;
mod token;

// Import javascript functions
#[wasm_bindgen]
extern {
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
    pub counter: String
}

impl ExportableMachine {
    pub fn from_machine(mut machine: norma::Machine) -> ExportableMachine {
        return ExportableMachine {
            registers: machine.get_registers_exportable(),
            counter: machine.get_counter().to_str_radix(10).to_string()
        }
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
    let mut registers = norma::Machine::new(BigUint::parse_bytes(input.as_bytes(),10).unwrap());
    registers.insert("VAL");
    registers.insert("TMP");
    registers.insert("CNT");

    while !registers.is_zero("X").unwrap() {
        registers.dec("X");
        registers.inc("VAL");
        registers.inc("CNT");
    }

    while !registers.is_zero("CNT").unwrap() {
        registers.dec("CNT");

        while !registers.is_zero("VAL").unwrap() {
            registers.inc("Y");
            registers.inc("TMP");
            registers.dec("VAL");
        }

        while !registers.is_zero("TMP").unwrap() {
            registers.inc("VAL");
            registers.dec("TMP");
        }
    }


    JsValue::from_serde(&ExportableMachine::from_machine(registers)).unwrap()
}

#[wasm_bindgen]
pub fn test(input: &str) -> JsValue {
    let mut registers = norma::Machine::new(BigUint::parse_bytes(input.as_bytes(),10).unwrap());
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

    if registers.is_zero("X").unwrap() {
        registers.inc("Y");
        registers.dec("J");   
    }

    JsValue::from_serde(&ExportableMachine::from_machine(registers)).unwrap()
}
