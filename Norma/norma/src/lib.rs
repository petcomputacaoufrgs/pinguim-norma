use wasm_bindgen::prelude::*;
use std::collections::HashMap;
use num_bigint::BigUint;
use serde::{Serialize, Deserialize};

mod norma;


// Import javascript functions
#[wasm_bindgen]
extern {
    pub fn alert(s: &str);

    #[wasm_bindgen(js_namespace= console , js_name = log)]
    fn printf(a: &str);
}

/// A Norma machine adapted to be exported for JS
/// registers -> k: name of register, v: value
/// counter -> count
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

    if registers.is_zero("X") {
        registers.inc("Y");
        registers.dec("J");   
    }
    
    let x_value = registers.get_value("X");
    let y_value = registers.get_value("Y");
    let j_value = registers.get_value("J");
    let counter = registers.get_counter();

    alert(&format!("X: {} \nY: {} \nJ: {} \nCounter: {}", x_value, y_value, j_value, counter));

    JsValue::from_serde(&ExportableMachine::from_machine(registers)).unwrap()
}