use crate::register::RegisterIndex;
use std::{collections::HashMap, fmt};

pub type Label = usize;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Location {
    routine: String,
    symbol: String,
}

impl Location {
    pub fn new(routine: &str, symbol: &str) -> Self {
        Self { routine: routine.to_string(), symbol: symbol.to_string() }
    }

    pub fn routine(&self) -> &str {
        &self.routine
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }
}

impl fmt::Display for Location {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}.{}", self.routine(), self.symbol())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum OperationKind {
    Inc,
    Dec,
}

#[derive(Debug, Clone, Copy)]
pub struct Operation {
    kind: OperationKind,
    register: RegisterIndex,
    destiny: Label,
}

impl Operation {
    pub fn new(
        kind: OperationKind,
        register: RegisterIndex,
        destiny: Label,
    ) -> Self {
        Self { kind, register, destiny }
    }

    pub fn kind(self) -> OperationKind {
        self.kind
    }

    pub fn register(self) -> RegisterIndex {
        self.register
    }

    pub fn destiny(self) -> Label {
        self.destiny
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TestKind {
    IsZero,
}

#[derive(Debug, Clone, Copy)]
pub struct Test {
    kind: TestKind,
    register: RegisterIndex,
    true_dest: Label,
    false_dest: Label,
}

impl Test {
    pub fn new(
        kind: TestKind,
        register: RegisterIndex,
        true_dest: Label,
        false_dest: Label,
    ) -> Self {
        Self { kind, register, true_dest, false_dest }
    }

    pub fn kind(self) -> TestKind {
        self.kind
    }

    pub fn register(self) -> RegisterIndex {
        self.register
    }

    pub fn true_dest(self) -> Label {
        self.true_dest
    }

    pub fn false_dest(self) -> Label {
        self.false_dest
    }

    pub fn destiny(self, success: bool) -> Label {
        if success {
            self.true_dest
        } else {
            self.false_dest
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum InstructionKind {
    Operation(Operation),
    Test(Test),
}

#[derive(Debug, Clone)]
pub struct Instruction {
    kind: InstructionKind,
    location: Location,
}

impl Instruction {
    pub fn new(kind: InstructionKind, location: Location) -> Self {
        Self { kind, location }
    }

    pub fn kind(&self) -> InstructionKind {
        self.kind
    }

    pub fn location(&self) -> &Location {
        &self.location
    }
}

#[derive(Debug, Clone)]
pub struct Program {
    instructions: Vec<Instruction>,
    location_table: HashMap<Location, Label>,
}

impl Program {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        let location_table = instructions
            .iter()
            .enumerate()
            .map(|(index, instruction)| (instruction.location().clone(), index))
            .collect();
        Self { instructions, location_table }
    }

    pub fn location_to_label(&self, location: &Location) -> Option<Label> {
        self.location_table.get(location).copied()
    }

    pub fn instruction(&self, label: Label) -> Option<&Instruction> {
        self.instructions.get(label)
    }
}
