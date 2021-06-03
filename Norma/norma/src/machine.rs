use crate::{
    instruction::{
        InstructionKind,
        Label,
        Operation,
        OperationKind,
        Program,
        Test,
        TestKind,
    },
    register::{RegisterBank, RegisterIndex},
};

#[derive(Debug, Clone)]
pub struct Machine {
    registers: RegisterBank,
    program: Program,
    program_counter: Label,
}

impl Machine {
    pub fn new(
        registers: RegisterBank,
        program: Program,
        program_counter: Label,
    ) -> Self {
        Self { registers, program, program_counter }
    }

    pub fn registers(&self) -> &RegisterBank {
        &self.registers
    }

    pub fn registers_mut(&mut self) -> &mut RegisterBank {
        &mut self.registers
    }

    pub fn program(&self) -> &Program {
        &self.program
    }

    pub fn program_counter(&self) -> Label {
        self.program_counter
    }

    pub fn step(&mut self) -> bool {
        match self.program.instruction(self.program_counter) {
            Some(instruction) => {
                let kind = instruction.kind();
                self.exec_instruction(kind);
                true
            },
            None => false,
        }
    }

    pub fn exec_instruction(&mut self, instruction: InstructionKind) {
        match instruction {
            InstructionKind::Operation(oper) => self.exec_operation(oper),
            InstructionKind::Test(test) => self.exec_test(test),
        }
    }

    pub fn exec_operation(&mut self, operation: Operation) {
        match operation.kind() {
            OperationKind::Inc => self.exec_inc(operation.register()),
            OperationKind::Dec => self.exec_dec(operation.register()),
        }
        self.program_counter = operation.destiny();
    }

    pub fn exec_inc(&mut self, register_index: RegisterIndex) {
        self.registers.register_mut(register_index).inc();
    }

    pub fn exec_dec(&mut self, register_index: RegisterIndex) {
        self.registers.register_mut(register_index).dec();
    }

    pub fn exec_test(&mut self, test: Test) {
        let success = match test.kind() {
            TestKind::IsZero => self.test_is_zero(test.register()),
        };
        self.program_counter = test.destiny(success);
    }

    pub fn test_is_zero(&self, register_index: RegisterIndex) -> bool {
        self.registers.register(register_index).is_zero()
    }
}
