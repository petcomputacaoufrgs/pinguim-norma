use crate::compiler::{
    lexer::token::{BuiltInOperation, BuiltInTest},
    position::Span,
};
use indexmap::IndexMap;
use num_bigint::BigUint;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Instruction {
    pub label: Symbol,
    pub instruction_type: InstructionType,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OperationType {
    BuiltIn(BuiltInOperation, Symbol),
    Macro(Symbol, Vec<MacroArgument>),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TestType {
    BuiltIn(BuiltInTest, Symbol),
    Macro(Symbol, Vec<MacroArgument>),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InstructionType {
    Operation(Operation),
    Test(Test),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Operation {
    pub oper_type: OperationType,
    pub next_label: Symbol,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Test {
    pub test_type: TestType,
    pub next_true_label: Symbol,
    pub next_false_label: Symbol,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Macro {
    pub macro_type: MacroType,
    pub name: Symbol,
    pub parameters: Vec<Symbol>,
    pub instr: IndexMap<String, Instruction>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MacroType {
    Operation,
    Test,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Symbol {
    pub content: String,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MacroArgument {
    Register(Symbol),
    Number(BigUint),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Main {
    pub code: IndexMap<String, Instruction>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Program {
    pub main: Main,
    pub macros: IndexMap<String, Macro>,
}
