use crate::compiler::parser::ast;
use std::{error::Error, fmt};

#[derive(Clone, Debug)]
pub struct UndefinedMacro {
    pub macro_name: String,
}

impl fmt::Display for UndefinedMacro {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Macro \"{}\" não existe", self.macro_name)
    }
}

impl Error for UndefinedMacro {}

#[derive(Clone, Debug)]
pub struct RecursiveMacro {
    pub macro_name: String,
}

impl fmt::Display for RecursiveMacro {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "Macro \"{}\" contém chamada recursiva",
            self.macro_name
        )
    }
}

impl Error for RecursiveMacro {}

#[derive(Clone, Debug)]
pub struct IncompatibleMacroType {
    pub macro_name: String,
    pub expected_type: ast::MacroType,
    pub found_type: ast::MacroType,
}

impl fmt::Display for IncompatibleMacroType {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "Esperada macro do tipo {}, mas macro \"{}\" é do tipo {}",
            self.expected_type, self.macro_name, self.found_type
        )
    }
}

impl Error for IncompatibleMacroType {}

#[derive(Clone, Debug)]
pub struct MismatchedArgsNumber {
    pub macro_name: String,
    pub expected_num: usize,
    pub found_num: usize,
}

impl fmt::Display for MismatchedArgsNumber {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "Macro \"{}\" requer que sejam passados {} argumentos, mas foram \
             fornecidos {}",
            self.macro_name, self.expected_num, self.found_num
        )
    }
}

impl Error for MismatchedArgsNumber {}

#[derive(Clone, Debug)]
pub struct MismatchedArgType {
    pub macro_name: String,
    pub expected_type: ast::MacroArgumentType,
    pub found_type: ast::MacroArgumentType,
    pub index: usize,
}

impl fmt::Display for MismatchedArgType {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "Macro \"{}\" requer que argumento do índice {} (a partir do 0) \
             seja do tipo {}, mas foram tipo {} foi encontrado",
            self.macro_name, self.index, self.expected_type, self.found_type
        )
    }
}

impl Error for MismatchedArgType {}

#[derive(Clone, Debug)]
pub struct InvalidLabelForMain {
    pub label: String,
}

impl fmt::Display for InvalidLabelForMain {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Rótulo \"{}\" é inválido para main", self.label)
    }
}

impl Error for InvalidLabelForMain {}

#[derive(Clone, Debug)]
pub struct InvalidLabelForOperMacro {
    pub label: String,
}

impl fmt::Display for InvalidLabelForOperMacro {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "Rótulo \"{}\" é inválido para macro de operação",
            self.label
        )
    }
}

impl Error for InvalidLabelForOperMacro {}

#[derive(Clone, Debug)]
pub struct InvalidLabelForTestMacro {
    pub label: String,
}

impl fmt::Display for InvalidLabelForTestMacro {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "Rótulo \"{}\" é inválido para macro de teste",
            self.label
        )
    }
}

impl Error for InvalidLabelForTestMacro {}
