use crate::machine::{Machine, RegisterName};
use indexmap::IndexMap;
use num_bigint::BigUint;

// ("1.add.2", "do inc X goto 1.add.3")

#[derive(Debug, Clone)]
pub struct Program {
    current: Label,
    instructions: IndexMap<Label, Instruction>,
    machine: Machine,
}

impl Program {
    pub fn new(
        instructions: IndexMap<Label, Instruction>,
        machine: Machine,
    ) -> Self {
        let (current, _) =
            instructions.first().expect("One instruction required");
        Self { current: current.clone(), instructions, machine }
    }

    pub fn run_step(&mut self) -> bool {
        let entry = self.instructions.get(&self.current).cloned();
        match entry {
            Some(instruction) => {
                self.run_instruction(instruction);
                true
            },
            None => false,
        }
    }

    pub fn run_steps(&mut self, max_steps: u64) -> bool {
        for _ in 0 .. max_steps {
            if !self.run_step() {
                return false;
            }
        }
        true
    }

    pub fn run_all(&mut self) {
        while self.run_step() {}
    }

    fn run_instruction(&mut self, instruction: Instruction) {
        match instruction.kind {
            InstructionKind::Test(test) => self.run_test(test),
            InstructionKind::Operation(operation) => {
                self.run_operation(operation)
            },
        }
    }

    fn run_operation(&mut self, operation: Operation) {
        match operation.kind {
            OperationKind::Inc(register) => self.run_inc(&register),
            OperationKind::Dec(register) => self.run_dec(&register),
            OperationKind::Add(reg_left, reg_right, reg_tmp) => {
                self.run_add(&reg_left, &reg_right, &reg_tmp)
            },
        }
        self.current = operation.next;
    }

    fn run_inc(&mut self, reg_name: &RegisterName) {
        self.machine.inc(reg_name);
    }

    fn run_dec(&mut self, reg_name: &RegisterName) {
        self.machine.dec(reg_name);
    }

    fn run_test(&mut self, test: Test) {
        let success = match test.kind {
            TestKind::Zero(register) => self.test_zero(&register),
            TestKind::Equals(reg_left, reg_right) => {
                self.test_equals(reg_left, reg_right)
            },
            TestKind::LessThan(reg_left, reg_right) => {
                self.test_less_than(reg_left, reg_right)
            },
        };
        self.current = if success { test.next_then } else { test.next_else };
    }

    fn test_zero(&mut self, reg_name: &RegisterName) -> bool {
        self.machine.is_zero(reg_name)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Label {
    content: String,
}

#[derive(Debug, Clone)]
pub struct Instruction {
    pub kind: InstructionKind,
}

#[derive(Debug, Clone)]
pub enum InstructionKind {
    Operation(Operation),
    Test(Test),
}

#[derive(Debug, Clone)]
pub struct Operation {
    pub kind: OperationKind,
    pub next: Label,
}

#[derive(Debug, Clone)]
pub enum OperationKind {
    Inc(RegisterName),
    Dec(RegisterName),
    AddConst(BigUint, RegisterName),
    Add(RegisterName, RegisterName, RegisterName),
    SubConst(BigUint, RegisterName),
    Sub(RegisterName, RegisterName, RegisterName),
}

#[derive(Debug, Clone)]
pub struct Test {
    pub kind: TestKind,
    pub next_then: Label,
    pub next_else: Label,
}

#[derive(Debug, Clone)]
pub enum TestKind {
    Zero(RegisterName),
    Equals(RegisterName, RegisterName),
    LessThan(RegisterName, RegisterName),
}
