use crate::compiler::{
    lexer::token::{BuiltInOperation, BuiltInTest},
    position::Span,
};
use indexmap::IndexMap;
use num_bigint::BigUint;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Instruction {
    ///
    /// - `label`: rótulo da instrução
    pub label: Symbol,
    ///
    /// - `instruction_type`: tipo da instrução
    pub instruction_type: InstructionType,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OperationType {
    ///
    /// - `BuiltIn`: referente a operações da própria norma
    BuiltIn(BuiltInOperation, Symbol),
    ///
    /// - `Macro`: operações escritas pelo usuário
    Macro(Symbol, Vec<MacroArgument>),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TestType {
    ///
    /// - `BuiltIn`: referente a testes da própria norma
    BuiltIn(BuiltInTest, Symbol),
    ///
    /// - `Macro`: testes escritas pelo usuário
    Macro(Symbol, Vec<MacroArgument>),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InstructionType {
    ///
    /// - `Operation`: referente a instruções que executam operações (macros ou builtin) e guarda consigo tal operação
    Operation(Operation),
    ///
    /// - `Test`: referente a instruções que executam testes (macros ou builtin) e guarda consigo tal teste
    Test(Test),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Operation {
    ///
    /// - `oper_type`: tipo de operação
    pub oper_type: OperationType,
    ///
    /// - `next_label`: rótulo para o qual essa instrução manda após executar sua operação
    pub next_label: Symbol,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Test {
    ///
    /// - `test_type`: tipo de teste
    pub test_type: TestType,
    ///
    /// - `next_true_label`: rótulo para o qual essa instrução manda após executar seu teste e ele der verdadeiro
    pub next_true_label: Symbol,
    ///
    /// - `next_false_label`: rótulo para o qual essa instrução manda após executar seu teste e ele der falso
    pub next_false_label: Symbol,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Macro {
    ///
    /// - `macro_type`: tipo de macro
    pub macro_type: MacroType,
    ///
    /// - `name`: nome da macro
    pub name: Symbol,
    ///
    /// - `parameters`: paramêtros formais da macro
    pub parameters: Vec<Symbol>,
    ///
    /// - `instr`: mapeamento das instruções com seus labels (código da macro)
    pub instr: IndexMap<String, Instruction>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MacroType {
    ///
    /// - `Operation`: macro que realiza uma operação com o conteúdo dos registradores
    Operation,
    /// - `Teste`: macro que realiza um teste com o conteúdo dos registradores
    Test,
}

/// Implementa a trait Display, útil para formatar mensagens de erro
impl fmt::Display for MacroType {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MacroType::Operation => write!(formatter, "Operation"),
            MacroType::Test => write!(formatter, "Test"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Symbol {
    ///
    /// - `content`: palavra do código, mas que não é necessariamente código em si
    pub content: String,
    ///
    /// - `span`: localização dessa palavra no código
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MacroArgument {
    ///
    /// - `Register`: referente ao argumento registrador e carrega consigo o símbolo referente a essa registrador
    Register(Symbol),
    ///
    /// - `Number`: referente ao argumento número e carrega consigo o valor em BigUInt consigo
    Number(BigUint),
}

impl MacroArgument {
    /// Retorna o tipo de MacroArgument
    pub fn arg_type(&self) -> MacroArgumentType {
        match self {
            MacroArgument::Register(_) => MacroArgumentType::Register,
            MacroArgument::Number(_) => MacroArgumentType::Number,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Main {
    ///
    /// - `code`: instruções do corpo da função main
    pub code: IndexMap<String, Instruction>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Program {
    ///
    /// - `main`: código da função principal do programa
    pub main: Main,
    ///
    /// - `macros`: mapeamento dos nomes das macros declaradas para suas informações e código
    pub macros: IndexMap<String, Macro>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MacroArgumentType {
    ///
    /// - `Register`: quando o argumento passado for um registrador
    Register,
    /// - `Number`: quando o argumento passado for um número
    Number,
}

/// Implementa a trait Display, útil para formatar mensagens de erro
impl fmt::Display for MacroArgumentType {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MacroArgumentType::Register => write!(formatter, "registrador"),
            MacroArgumentType::Number => write!(formatter, "número"),
        }
    }
}
