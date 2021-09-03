use super::token::{BuiltInOperation, BuiltInTest, Span};
use indexmap::IndexMap;
use num_bigint::BigUint;

#[derive(Clone, Debug)]
pub struct Instruction {
    pub label: Symbol,
    pub instruction_type: InstructionType,
}

impl Instruction {
    pub fn new(label: Symbol, typ: InstructionType) -> Self {
        Instruction { label, instruction_type: typ }
    }
}

#[derive(Clone, Debug)]
pub enum OperationType {
    BuiltIn(BuiltInOperation, Symbol),
    Macro(Symbol, Vec<MacroParam>),
}

#[derive(Clone, Debug)]
pub enum TestType {
    BuiltIn(BuiltInTest, Symbol),
    Macro(Symbol, Vec<MacroParam>),
}

#[derive(Clone, Debug)]
pub enum InstructionType {
    Operation(Operation),
    Test(Test),
}

#[derive(Clone, Debug)]
pub struct Operation {
    pub oper_type: OperationType,
    pub next_label: Symbol,
}

#[derive(Clone, Debug)]
pub struct Test {
    pub test_type: TestType,
    pub next_true_label: Symbol,
    pub next_false_label: Symbol,
}

#[derive(Clone, Debug)]
pub struct Macro {
    pub macro_type: MacroType,
    pub name: Symbol,
    pub parameters: Vec<Symbol>,
    pub instr: IndexMap<String, Instruction>,
}

#[derive(Clone, Debug)]
pub enum MacroType {
    Operation,
    Test,
}

#[derive(Clone, Debug)]
pub struct Symbol {
    pub content: String,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub enum MacroParam {
    Register(Symbol),
    Number(BigUint),
}

#[derive(Clone, Debug)]
pub struct Main {
    pub code: IndexMap<String, Instruction>,
}

pub struct Program {
    pub main: Main,
    pub macros: IndexMap<String, Macro>,
}
