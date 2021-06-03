pub mod register;
pub mod instruction;
pub mod machine;

use instruction::{
    Instruction,
    InstructionKind,
    Location,
    Operation,
    OperationKind,
    Program,
    Test,
    TestKind,
};
use machine::Machine;
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
        ]);

        let reg_x_index = reg_bank.symbol_to_index("X").expect("I put X there");
        let reg_y_index = reg_bank.symbol_to_index("Y").expect("I put Y there");

        let location0 = Location::new("main", "1");
        let location1 = Location::new("main", "2");
        let location2 = Location::new("main", "3");

        let instruction0 = InstructionKind::Test(Test::new(
            TestKind::IsZero,
            reg_x_index,
            3,
            1,
        ));
        let instruction1 = InstructionKind::Operation(Operation::new(
            OperationKind::Dec,
            reg_x_index,
            2,
        ));
        let instruction2 = InstructionKind::Operation(Operation::new(
            OperationKind::Inc,
            reg_y_index,
            0,
        ));

        let program = Program::new(vec![
            Instruction::new(instruction0, location0),
            Instruction::new(instruction1, location1),
            Instruction::new(instruction2, location2),
        ]);

        let machine = Machine::new(reg_bank, program, 0);
        while machine.step() {}

        let reg_bank = machine.registers();

        alert(&register_bank.register(reg_y_index).to_string());
    }
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, norma!");
}
