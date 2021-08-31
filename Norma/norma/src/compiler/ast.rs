use super::token::Span;
use indexmap::IndexMap;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Instruction {
    label: String,
    instruction_type: InstructionType,
    registers: Vec<Symbol>,
    constant: Option<usize>,
}

impl Instruction {
    pub fn new(label: String, typ: InstructionType, regs: Vec<Symbol>, constant: Option<usize>) -> Self {
        Instruction {
            label,
            instruction_type: typ,
            registers: regs,
            constant
        }
    }

    pub fn label(&self) -> &str {
        &self.label
    }
}

#[derive(Clone, Debug)]
pub enum OperationType {
    Inc,
    Dec,
    AddConst,
    SubConst,
    AddRegs,
    SubRegs,
    Macro(Symbol)
}

#[derive(Clone, Debug)]
pub enum TestType {
    Zero,
    CmpConst,
    CmpRegs,
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

pub struct Macro {
    macro_type: MacroType,
    name: Symbol,
    parameters: Vec<Symbol>,
    instr: IndexMap<String, Instruction>,
}

pub enum MacroType {
    Operation,
    Test,
}

#[derive(Clone, Debug)]
pub struct Symbol {
    content: String,
    span: Span,
}

#[derive(Clone, Debug)]
pub struct Main {
    code: IndexMap<String, Instruction>,
}

pub struct Program {
    main: Main,
    macros: HashMap<String, Macro>
}


// struct das macros
// struct da main

// main -> lesse uma macro ia la na struct das macros e ia fazer um append das linhas

// 1. intrs.... index (0)
// 2. instr index (1)

// expansor
// 1.macro1.1. asasushuas (2)
// 1.macro1.2. ahsuahsuhahu (3)


// parser pt1: expansao das macros com macros 

// operation add(A,B) {
//     intsr1: ...
//     instr2: ...
// }

// parser pt2: expansao das instrucoes da main em indexmap

// main {
//     1: .... (1)
//     2: .... (2)
//     3. do add(A,B) ... 
//     intsr1: ...
//     instr2: ...
// }

// Instruction {
//     InstructionType::Operation::OperationType::Macro
//     name: 
//     regs:
//     ...
// }

// parser pt3: juntar tudo 

// itera indexmap_main, cada vez q acha macro 