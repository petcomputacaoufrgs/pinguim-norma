use super::token::{Span, BuiltInOperation, BuiltInTest};
use indexmap::IndexMap;
use num_bigint::BigUint;

#[derive(Clone, Debug)]
pub struct Instruction {
    pub label: Symbol,
    pub instruction_type: InstructionType,
    pub parameters: Parameters,
}

impl Instruction {
    pub fn new(label: Symbol, typ: InstructionType, parameters: Parameters) -> Self {
        Instruction {
            label,
            instruction_type: typ,
            parameters,
        }
    }
}

#[derive(Clone, Debug)]
pub enum OperationType {
    BuiltIn(BuiltInOperation),
    Macro(Symbol)
}

#[derive(Clone, Debug)]
pub enum TestType {
    BuiltIn(BuiltInTest),
    Macro(Symbol)
}

#[derive(Clone, Debug)]
pub enum InstructionType {
    Operation(Operation),
    Test(Test)
}

#[derive(Clone, Debug)]
pub struct Operation {
    oper_type: OperationType,
    next_label: Symbol,
}

#[derive(Clone, Debug)]
pub struct Test {
    test_type: TestType,
    next_true_label: Symbol,
    next_false_label: Symbol,
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
pub struct Parameters {
    registers: Vec<String>,
    constant: Option<BigUint>,
}

#[derive(Clone, Debug)]
pub struct Main {
    pub code: IndexMap<String, Instruction>,
}

pub struct Program {
    pub main: Main,
    pub macros: IndexMap<String, Macro>
}
