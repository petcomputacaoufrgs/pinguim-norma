use crate::machine::Machine;
use indexmap::IndexMap;
use num_bigint::BigUint;

// ("1.add.2", "do inc X goto 1.add.3")

#[derive(Debug, Clone)]
pub struct Program {
    current: String,
    instructions: IndexMap<String, Instruction>,
    machine: Machine,
}

impl Program {
    pub fn new(
        instructions: IndexMap<String, Instruction>,
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
            OperationKind::AddConst(register, constant) => {
                self.run_add_const(&register, &constant)
            },
            OperationKind::Add(reg_left, reg_right, reg_tmp) => {
                self.run_add(&reg_left, &reg_right, &reg_tmp)
            },
        }
        self.current = operation.next;
    }

    fn run_inc(&mut self, reg_name: &str) {
        self.machine.inc(reg_name);
    }

    fn run_dec(&mut self, reg_name: &str) {
        self.machine.dec(reg_name);
    }

    fn run_add_const(&mut self, reg_name: &str, constant: &BigUint) {
        self.machine.add_const(reg_name, constant);
    }

    fn run_add(&mut self, reg_left: &str, reg_right: &str, reg_tmp: &str) {
        todo!()
    }

    fn run_test(&mut self, test: Test) {
        let success = match test.kind {
            TestKind::Zero(register) => self.test_zero(&register),
            TestKind::EqConst(register, constant) => {
                self.test_eq_const(&register, &constant)
            },
            TestKind::Eq(reg_left, reg_right, reg_tmp) => {
                self.test_eq(&reg_left, &reg_right, &reg_tmp)
            },
        };
        self.current = if success { test.next_then } else { test.next_else };
    }

    fn test_zero(&mut self, reg_name: &str) -> bool {
        self.machine.is_zero(reg_name)
    }

    fn test_eq_const(&mut self, register: &str, constant: &BigUint) -> bool {
        self.machine.eq_const(register, constant)
    }

    fn test_eq(
        &mut self,
        reg_left: &str,
        reg_right: &str,
        reg_tmp: &str,
    ) -> bool {
        todo!()
    }
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
    pub next: String,
}

#[derive(Debug, Clone)]
pub enum OperationKind {
    Inc(String),
    Dec(String),
    AddConst(String, BigUint),
    Add(String, String, String),
}

#[derive(Debug, Clone)]
pub struct Test {
    pub kind: TestKind,
    pub next_then: String,
    pub next_else: String,
}

#[derive(Debug, Clone)]
pub enum TestKind {
    Zero(String),
    EqConst(String, BigUint),
    Eq(String, String, String),
}
