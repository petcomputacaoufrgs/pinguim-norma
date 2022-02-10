use crate::compiler::parser::ast;
use std::{error::Error, fmt};

#[derive(Clone, Debug)]
pub struct UndefinedMacro {
    ///
    /// - `macro_name`: nome da macro que não foi definida no código
    pub macro_name: String,
}

/// Implementa a trait Display para mensagens de erro
impl fmt::Display for UndefinedMacro {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Macro \"{}\" não existe", self.macro_name)
    }
}

/// Implementa a trait Error para poder ser colocado no diagnóstico
impl Error for UndefinedMacro {}

#[derive(Clone, Debug)]
pub struct RecursiveMacro {
    ///
    /// - `macro_name`: nome da macro que executa chamadas recursivas
    pub macro_names: Vec<String>,
}

/// Implementa a trait Display para mensagens de erro
impl fmt::Display for RecursiveMacro {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Recursão detectada entre as macros: ")?;

        let count = self.macro_names.len();

        if count > 1 {
            for expected_type in &self.macro_names[..count - 2] {
                write!(formatter, "\"{}\", ", expected_type)?;
            }

            write!(formatter, "\"{}\" e ", self.macro_names[count - 2])?;
        }

        if count > 0 {
            write!(formatter, "\"{}\"", self.macro_names[count - 1])?;
        }

        Ok(())
    }
}

/// Implementa a trait Error para poder ser colocado no diagnóstico
impl Error for RecursiveMacro {}

#[derive(Clone, Debug)]
pub struct IncompatibleMacroType {
    pub macro_name: String,
    pub expected_type: ast::MacroType,
    pub found_type: ast::MacroType,
}

/// Implementa a trait Display para mensagens de erro
impl fmt::Display for IncompatibleMacroType {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "Esperada macro do tipo {}, mas macro \"{}\" é do tipo {}",
            self.expected_type, self.macro_name, self.found_type
        )
    }
}

/// Implementa a trait Error para poder ser colocado no diagnóstico
impl Error for IncompatibleMacroType {}

#[derive(Clone, Debug)]
pub struct MismatchedArgsNumber {
    ///
    /// - `macro_name`: nome da macro cujos argumentos não estão de acordo com os parâmetros formais
    pub macro_name: String,
    ///
    /// - `expected_num`: número de parâmetros formais
    pub expected_num: usize,
    ///
    /// - `found_num`: número de argumentos passados na chamada da macro
    pub found_num: usize,
}

/// Implementa a trait Display para mensagens de erro
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

/// Implementa a trait Error para poder ser colocado no diagnóstico
impl Error for MismatchedArgsNumber {}

#[derive(Clone, Debug)]
pub struct MismatchedArgType {
    ///
    /// - `macro_name`: nome da macro cujo argumento passado não é do tipo esperado
    pub macro_name: String,
    ///
    /// - `expected_type`: tipo do parâmetro formal
    pub expected_type: ast::MacroArgumentType,
    ///
    /// - `found_type`: tipo do argumento passado na chamada da macro
    pub found_type: ast::MacroArgumentType,
    ///
    /// - `index`: índice do argumento do tipo errado na chamada
    pub index: usize,
}

/// Implementa a trait Display para mensagens de erro
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

/// Implementa a trait Error para poder ser colocado no diagnóstico
impl Error for MismatchedArgType {}

#[derive(Clone, Debug)]
pub struct InvalidLabelForMain {
    ///
    /// - `label`: nome do label inválido
    pub label: String,
}

/// Implementa a trait Display para mensagens de erro
impl fmt::Display for InvalidLabelForMain {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Rótulo \"{}\" é inválido para main", self.label)
    }
}

/// Implementa a trait Error para poder ser colocado no diagnóstico
impl Error for InvalidLabelForMain {}

#[derive(Clone, Debug)]
pub struct InvalidLabelForOperMacro {
    ///
    /// - `label`: nome do label inválido
    pub label: String,
}

/// Implementa a trait Display para mensagens de erro
impl fmt::Display for InvalidLabelForOperMacro {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "Rótulo \"{}\" é inválido para macro de operação",
            self.label
        )
    }
}

/// Implementa a trait Error para poder ser colocado no diagnóstico
impl Error for InvalidLabelForOperMacro {}

#[derive(Clone, Debug)]
pub struct InvalidLabelForTestMacro {
    ///
    /// - `label`: nome do label inválido
    pub label: String,
}

/// Implementa a trait Display para mensagens de erro
impl fmt::Display for InvalidLabelForTestMacro {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "Rótulo \"{}\" é inválido para macro de teste",
            self.label
        )
    }
}

/// Implementa a trait Error para poder ser colocado no diagnóstico
impl Error for InvalidLabelForTestMacro {}
