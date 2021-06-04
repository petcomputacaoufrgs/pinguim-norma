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

/// The interpreter of a Norma program.
#[derive(Debug, Clone)]
pub struct Interpreter {
    registers: RegisterBank,
    program: Program,
    program_counter: Label,
}

impl Interpreter {
    /// Initializes the interpreter given the register bank with all registers
    /// (and their initial values), a program (all the instructions), and
    /// initial program counter (position of the current instruction).
    pub fn new(
        registers: RegisterBank,
        program: Program,
        program_counter: Label,
    ) -> Self {
        Self { registers, program, program_counter }
    }

    /// Returns an immutable reference to the register bank of this interpreter.
    pub fn registers(&self) -> &RegisterBank {
        &self.registers
    }

    /// Returns a mutable reference to the register bank of this interpreter.
    pub fn registers_mut(&mut self) -> &mut RegisterBank {
        &mut self.registers
    }

    /// Returns an immutable reference to the program being executed by this
    /// interpreter.
    pub fn program(&self) -> &Program {
        &self.program
    }

    /// Returns the program counter, i.e. the index of the next instruction,
    /// possibly invalid (which means execution ended).
    pub fn program_counter(&self) -> Label {
        self.program_counter
    }

    /// Executes a single instruction if not yet halted. Returns whether an
    /// instruction is executed.
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

    fn exec_instruction(&mut self, instruction: InstructionKind) {
        match instruction {
            InstructionKind::Operation(oper) => self.exec_operation(oper),
            InstructionKind::Test(test) => self.exec_test(test),
        }
    }

    fn exec_operation(&mut self, operation: Operation) {
        match operation.kind() {
            OperationKind::Inc => self.exec_inc(operation.register()),
            OperationKind::Dec => self.exec_dec(operation.register()),
        }
        self.program_counter = operation.destiny();
    }

    fn exec_inc(&mut self, register_index: RegisterIndex) {
        self.registers.register_mut(register_index).inc();
    }

    fn exec_dec(&mut self, register_index: RegisterIndex) {
        self.registers.register_mut(register_index).dec();
    }

    fn exec_test(&mut self, test: Test) {
        let success = match test.kind() {
            TestKind::IsZero => self.test_is_zero(test.register()),
        };
        self.program_counter = test.destiny(success);
    }

    fn test_is_zero(&self, register_index: RegisterIndex) -> bool {
        self.registers.register(register_index).is_zero()
    }
}
