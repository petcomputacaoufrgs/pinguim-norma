use wasm_bindgen::prelude::*;
#[warn(unused_imports)]
use num_bigint::BigUint;

mod norma;


// Import javascript functions
#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub struct Exportable_Machine {
    registers: Vec<String>,
    num_of_registers: usize,
    counter: String
}

#[wasm_bindgen]
impl Exportable_Machine {
    pub fn get_register(&self, index: usize) -> String {
        self.registers[index].clone()
    }

    pub fn get_counter(&self) -> String {
        self.counter.clone()
    }
}

#[wasm_bindgen]
pub fn test(input: &str) -> Exportable_Machine {
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

    let exp_regs = vec![x_value.to_str_radix(10), y_value.to_str_radix(10), j_value.to_str_radix(10)];

    return Exportable_Machine {
        registers: exp_regs,
        num_of_registers: 3,
        counter: counter.to_str_radix(10)
    };
}