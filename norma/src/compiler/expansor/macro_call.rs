use crate::compiler::parser::ast;

pub trait MacroCallExpansor<'ast> {
    type InstructionKind;

    /// Expande o label "true"
    ///
    /// - `outer_instr_kind`: tipo de instrução que chamou uma outra macro
    fn expand_true_label(
        &self,
        outer_instr_kind: &'ast Self::InstructionKind,
    ) -> String;

    /// Expande o label "false"
    ///
    /// - `outer_instr_kind`: tipo de instrução que chamou uma outra macro
    fn expand_false_label(
        &self,
        outer_instr_kind: &'ast Self::InstructionKind,
    ) -> String;

    /// Expande labels inválidos
    ///
    /// - `outer_instr_kind`: tipo de instrução que chamou uma outra macro
    fn expand_invalid_label(
        &self,
        outer_instr_kind: &'ast Self::InstructionKind,
    ) -> String;

    fn macro_type(&self) -> ast::MacroType;
}

#[derive(Clone, Debug, Copy, Default)]
pub struct OperMacroCallExpansor;

impl<'ast> MacroCallExpansor<'ast> for OperMacroCallExpansor {
    type InstructionKind = ast::Operation;

    /// Expande o label "true" para operações
    ///
    /// - `outer_instr_kind`: tipo de instrução que chamou uma outra macro
    fn expand_true_label(
        &self,
        _outer_instr_kind: &'ast Self::InstructionKind,
    ) -> String {
        String::from("true")
    }

    /// Expande o label "false" para operações
    ///
    /// - `outer_instr_kind`: tipo de instrução que chamou uma outra macro
    fn expand_false_label(
        &self,
        _outer_instr_kind: &'ast Self::InstructionKind,
    ) -> String {
        String::from("false")
    }

    /// Expande labels inválidos para fora do programa
    ///
    /// - `outer_instr_kind`: tipo de instrução que chamou uma outra macro
    fn expand_invalid_label(
        &self,
        outer_instr_kind: &'ast Self::InstructionKind,
    ) -> String {
        outer_instr_kind.next_label.content.clone()
    }

    /// Retorna o tipo da macro
    fn macro_type(&self) -> ast::MacroType {
        ast::MacroType::Operation
    }
}

#[derive(Clone, Debug, Copy, Default)]
pub struct TestMacroCallExpansor;

impl<'ast> MacroCallExpansor<'ast> for TestMacroCallExpansor {
    type InstructionKind = ast::Test;

    /// Expande o label "true" para testes
    ///
    /// - `outer_instr_kind`: tipo de instrução que chamou uma outra macro
    fn expand_true_label(
        &self,
        outer_instr_kind: &'ast Self::InstructionKind,
    ) -> String {
        outer_instr_kind.next_true_label.content.clone()
    }

    /// Expande o label "false" para testes
    ///
    /// - `outer_instr_kind`: tipo de instrução que chamou uma outra macro
    fn expand_false_label(
        &self,
        outer_instr_kind: &'ast Self::InstructionKind,
    ) -> String {
        outer_instr_kind.next_false_label.content.clone()
    }

    /// Expande labels inválidos para testes
    ///
    /// - `outer_instr_kind`: tipo de instrução que chamou uma outra macro
    fn expand_invalid_label(
        &self,
        _outer_instr_kind: &'ast Self::InstructionKind,
    ) -> String {
        String::from("?")
    }

    /// Retorna o tipo da macro
    fn macro_type(&self) -> ast::MacroType {
        ast::MacroType::Test
    }
}
