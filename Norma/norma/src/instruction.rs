use crate::register::RegisterIndex;
use std::{collections::HashMap, fmt};

/// Type alias for labels, i.e. instruction indices. Defined for clarity.
pub type Label = usize;

/// A location in the source code of an instruction.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Location {
    routine: String,
    symbol: String,
}

impl Location {
    /// Initializes a location from routine name and source code label (the
    /// "symbol").
    pub fn new(routine: &str, symbol: &str) -> Self {
        Self { routine: routine.to_string(), symbol: symbol.to_string() }
    }

    /// Returns the name of the routine where an instruction is in the source
    /// code.
    pub fn routine(&self) -> &str {
        &self.routine
    }

    /// Returns the symbol in a routine (the label) identifying where an
    /// instruction is in the source code.
    pub fn symbol(&self) -> &str {
        &self.symbol
    }
}

impl fmt::Display for Location {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}.{}", self.routine(), self.symbol())
    }
}

/// The kind of an operation over a register.
#[derive(Debug, Clone, Copy)]
pub enum OperationKind {
    /// Increments the register.
    Inc,
    /// Decrements the register.
    Dec,
}

/// Data of an operation over a register.
#[derive(Debug, Clone, Copy)]
pub struct Operation {
    kind: OperationKind,
    register: RegisterIndex,
    destiny: Label,
}

impl Operation {
    /// Initializes this operation from its kind, the register on which
    /// operates, and the destiny label.
    pub fn new(
        kind: OperationKind,
        register: RegisterIndex,
        destiny: Label,
    ) -> Self {
        Self { kind, register, destiny }
    }

    /// The kind of this operation.
    pub fn kind(self) -> OperationKind {
        self.kind
    }

    /// The register on which this operation operates.
    pub fn register(self) -> RegisterIndex {
        self.register
    }

    /// The label to which this operation will jump after finished.
    pub fn destiny(self) -> Label {
        self.destiny
    }
}

/// The kind of a test for a register.
#[derive(Debug, Clone, Copy)]
pub enum TestKind {
    /// Tests if the register is zero.
    IsZero,
}

/// Data of a test for a register.
#[derive(Debug, Clone, Copy)]
pub struct Test {
    kind: TestKind,
    register: RegisterIndex,
    true_dest: Label,
    false_dest: Label,
}

impl Test {
    /// Initializes this test from its kind, the register tested, and the
    /// destiny labels for both true and false cases.
    pub fn new(
        kind: TestKind,
        register: RegisterIndex,
        true_dest: Label,
        false_dest: Label,
    ) -> Self {
        Self { kind, register, true_dest, false_dest }
    }

    /// The kind of this test.
    pub fn kind(self) -> TestKind {
        self.kind
    }

    /// The register being tested.
    pub fn register(self) -> RegisterIndex {
        self.register
    }

    /// Destination jumped to after the test if the test is successfull.
    pub fn true_dest(self) -> Label {
        self.true_dest
    }

    /// Destination jumped to after the test if the test fails.
    pub fn false_dest(self) -> Label {
        self.false_dest
    }

    /// The destination for the test given if it succeeded or not.
    pub fn destiny(self, success: bool) -> Label {
        if success {
            self.true_dest
        } else {
            self.false_dest
        }
    }
}

/// The kind of instruction data.
#[derive(Debug, Clone, Copy)]
pub enum InstructionKind {
    /// This is an operation instruction.
    Operation(Operation),
    /// This is a test instruction.
    Test(Test),
}

/// A Norma's instruction data.
#[derive(Debug, Clone)]
pub struct Instruction {
    kind: InstructionKind,
    location: Location,
}

impl Instruction {
    /// Initializes this instruction given its kind and the location in the
    /// source code.
    pub fn new(kind: InstructionKind, location: Location) -> Self {
        Self { kind, location }
    }

    /// The kind of this instruction.
    pub fn kind(&self) -> InstructionKind {
        self.kind
    }

    /// Location in the source code of the instruction.
    pub fn location(&self) -> &Location {
        &self.location
    }
}

/// A Norma program, i.e., a list of instructions.
#[derive(Debug, Clone)]
pub struct Program {
    instructions: Vec<Instruction>,
    location_table: HashMap<Location, Label>,
}

impl Program {
    /// Initializes this program given a list of instructions.
    pub fn new(instructions: Vec<Instruction>) -> Self {
        let location_table = instructions
            .iter()
            .enumerate()
            .map(|(index, instruction)| (instruction.location().clone(), index))
            .collect();
        Self { instructions, location_table }
    }

    /// Queries for a label whose instruction has the given location.
    pub fn location_to_label(&self, location: &Location) -> Option<Label> {
        self.location_table.get(location).copied()
    }

    /// Queries for an instruction given its label. Returns `None` if the label
    /// is out of program bounds.
    pub fn instruction(&self, label: Label) -> Option<&Instruction> {
        self.instructions.get(label)
    }
}
