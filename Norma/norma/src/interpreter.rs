use crate::machine::Machine;
use indexmap::IndexMap;
use num_bigint::BigUint;
use std::ops::AddAssign;

// ("1.add.2", "do inc X goto 1.add.3")

#[derive(Debug, Clone)]
pub struct Program {
    current: String,
    instructions: IndexMap<String, Instruction>,
    machine: Machine,
    steps: BigUint,
}

impl Program {
    pub fn new(
        instructions: IndexMap<String, Instruction>,
        machine: Machine,
        steps: BigUint,
    ) -> Self {
        let (current, _) =
            instructions.first().expect("One instruction required");
        Self { current: current.clone(), instructions, machine, steps }
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

    /// `inc A`
    ///
    /// `1` step
    fn run_inc(&mut self, reg_name: &str) {
        self.count_steps(1u8);
        self.machine.inc(reg_name);
    }

    /// `dec A`
    ///
    /// `1` step
    fn run_dec(&mut self, reg_name: &str) {
        self.count_steps(1u8);
        self.machine.dec(reg_name);
    }

    /// ```
    /// operation add_const N (A) {
    ///     1: do inc A goto 2
    ///     2: do inc A goto 3
    ///     ...
    ///     N: do inc A goto done
    /// }
    /// ```
    ///
    /// `N` steps
    fn run_add_const(&mut self, reg_name: &str, constant: &BigUint) {
        self.count_steps(constant);
        self.machine.add_const(reg_name, constant);
    }

    /// ```
    /// operation add (L, R, Tmp) {
    ///     cleanup: if zero Tmp then goto start else clear_Tmp
    ///     clear_Tmp: do dec Tmp goto cleanup
    ///
    ///     start: if zero R then goto restore else goto next_L
    ///     next_L: do inc L goto next_R
    ///     next_R: do dec R goto save_Tmp
    ///     save_Tmp: do inc Tmp goto start
    ///
    ///     restore: if zero Tmp then goto done else goto undo_R
    ///     undo_R: do inc R goto undo_Tmp
    ///     undo_Tmp: do dec Tmp goto restore
    /// }
    /// ```
    ///
    /// (Tmp * 2 + 1) + (R * 4 + 1) + (R * 3 + 1)
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

    /// `zero A`
    ///
    /// `1` step
    fn test_zero(&mut self, reg_name: &str) -> bool {
        self.count_steps(1u8);
        self.machine.is_zero(reg_name)
    }

    /// ```
    /// test eq_const N (A) {
    ///     1: if zero A then goto false else goto 1_dec
    ///     1_dec: do dec A goto 2
    ///     2: if zero A then goto false else goto 2_dec
    ///     2_dec: do dec A goto 3
    ///     ...
    ///     Nplus1: if zero A then goto 1_restore_true else goto 1_restore_false
    ///
    ///     1_restore_true: do inc A goto 2_restore_true
    ///     2_restore_true: do inc A goto 3_restore_true
    ///     ...
    ///     N_restore_true: do inc A goto true
    ///
    ///     1_restore_false: do inc A goto 2_restore_false
    ///     2_restore_false: do inc A goto 3_restore_false
    ///     ...
    ///     N_restore_false: do inc A goto false
    /// }
    /// ```
    ///
    /// `(min(A, N)) * 2 + 1 + min(A, N)` steps
    fn test_eq_const(&mut self, register: &str, constant: &BigUint) -> bool {
        self.count_steps(constant.min(register, constant) * 3 + 1);
        self.machine.eq_const(register, constant)
    }

    /// ```
    /// test eq (L, R, Tmp) {
    ///     cleanup: if zero Tmp then goto check_L else clear_Tmp
    ///     clear_Tmp: do dec Tmp goto cleanup
    ///
    ///     check_L: if zero L then goto check_LR else goto check_R
    ///     check_LR: if zero R then goto restore_true else goto restore_false
    ///     check_R: if zero R then goto restore_false else goto next_L
    ///     next_L: do dec L goto next_R
    ///     next_R: do dec R goto save_Tmp
    ///     save_Tmp: do inc Tmp goto check_L
    ///
    ///     restore: if zero Tmp then goto true else goto restore_true_Tmp
    ///     restore_true_Tmp: do dec Tmp goto restore_true_L
    ///     restore_true_L: do inc L goto restore_true_R
    ///     restore_true_R: do inc R goto restore_true
    ///
    ///     restore: if zero Tmp then goto false else goto restore_false_Tmp
    ///     restore_false_Tmp: do dec Tmp goto restore_false_L
    ///     restore_false_L: do inc L goto restore_false_R
    ///     restore_false_R: do inc R goto restore_false
    /// }
    /// ```
    ///
    /// (Tmp * 2 + 1) + (min(L, R) * 9 + 3)
    fn test_eq(
        &mut self,
        reg_left: &str,
        reg_right: &str,
        reg_tmp: &str,
    ) -> bool {
        todo!()
    }

    fn count_steps<T>(&mut self, amount: T)
    where
        BigUint: AddAssign<T>,
    {
        self.steps += amount;
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
