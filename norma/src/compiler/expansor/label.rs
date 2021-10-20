use super::error::{
    InvalidLabelForMain,
    InvalidLabelForOperMacro,
    InvalidLabelForTestMacro,
};
use crate::compiler::{
    error::{Diagnostics, Error},
    parser::ast,
};
use indexmap::IndexMap;
use std::fmt;

/*
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CodeType {
    Main,
    Macro(ast::MacroType),
}

impl fmt::Display for CodeType {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CodeType::Main => write!(formatter, "main"),
            CodeType::Macro(macro_type) => {
                write!(formatter, "{} macro", macro_type)
            },
        }
    }
}

impl CodeType {
    pub fn validate_label(
        self,
        label: &ast::Symbol,
        code: &IndexMap<String, ast::Instruction>,
        diagnostics: &mut Diagnostics,
    ) {
        match self {
            CodeType::Main => validate_for_main(label, code, diagnostics),
            CodeType::Macro(ast::MacroType::Operation) => {
                validate_for_oper_macro(label, code, diagnostics)
            },
            CodeType::Macro(ast::MacroType::Test) => {
                validate_for_test_macro(label, code, diagnostics)
            },
        }
    }
}
*/

/// Testa se o content do label é "true"
///
/// - `label`: conteúdo de um label
pub fn is_true(label: &str) -> bool {
    label == "true"
}

/// Testa se o content do label é "false"
///
/// - `label`: conteúdo de um label
pub fn is_false(label: &str) -> bool {
    label == "false"
}

pub fn validate_for_main(
    label: &ast::Symbol,
    code: &IndexMap<String, ast::Instruction>,
    diagnostics: &mut Diagnostics,
) {
    if is_false(&label.content) || is_true(&label.content) {
        let cause = InvalidLabelForMain { label: label.content.clone() };
        diagnostics.raise(Error::new(cause, label.span));
    }
}

pub fn validate_for_oper_macro(
    label: &ast::Symbol,
    _code: &IndexMap<String, ast::Instruction>,
    diagnostics: &mut Diagnostics,
) {
    if is_false(&label.content) || is_true(&label.content) {
        let cause = InvalidLabelForOperMacro { label: label.content.clone() };
        diagnostics.raise(Error::new(cause, label.span));
    }
}

pub fn validate_for_test_macro(
    label: &ast::Symbol,
    code: &IndexMap<String, ast::Instruction>,
    diagnostics: &mut Diagnostics,
) {
    let is_boolean_label = is_false(&label.content) || is_true(&label.content);
    if !is_boolean_label && !code.contains_key(&label.content) {
        let cause = InvalidLabelForTestMacro { label: label.content.clone() };
        diagnostics.raise(Error::new(cause, label.span));
    }
}
