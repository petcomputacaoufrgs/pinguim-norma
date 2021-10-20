use crate::{
    compiler::{error::Diagnostics, parser::ast},
    interpreter::program::Program,
};

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
    program: &Program,
    diagnostics: &mut Diagnostics,
) {
    validate_for_oper_macro(label, program, diagnostics);
}

pub fn validate_for_oper_macro(
    label: &ast::Symbol,
    _program: &Program,
    diagnostics: &mut Diagnostics,
) {
    if is_false(&label.content) || is_true(&label.content) {
        panic!("aaa")
    }
}

pub fn validate_for_test_macro(
    label: &ast::Symbol,
    program: &Program,
    diagnostics: &mut Diagnostics,
) {
    let is_boolean_label = is_false(&label.content) || is_true(&label.content);
    if !is_boolean_label && !program.is_label_valid(label) {
        panic!("aaaa")
    }
}
