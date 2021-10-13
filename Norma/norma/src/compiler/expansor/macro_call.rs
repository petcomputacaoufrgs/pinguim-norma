use crate::compiler::ast;

pub trait MacroCallExpansor<'ast> {
    type InstructionKind;

    fn expand_true_label(
        &self,
        outer_instr_kind: &'ast Self::InstructionKind,
    ) -> String;

    fn expand_false_label(
        &self,
        outer_instr_kind: &'ast Self::InstructionKind,
    ) -> String;

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

    fn expand_true_label(
        &self,
        outer_instr_kind: &'ast Self::InstructionKind,
    ) -> String {
        panic!("Erro de compilação");
    }

    fn expand_false_label(
        &self,
        outer_instr_kind: &'ast Self::InstructionKind,
    ) -> String {
        panic!("Erro de compilação");
    }

    fn expand_invalid_label(
        &self,
        outer_instr_kind: &'ast Self::InstructionKind,
    ) -> String {
        outer_instr_kind.next_label.content.clone()
    }

    fn macro_type(&self) -> ast::MacroType {
        ast::MacroType::Operation
    }
}

#[derive(Clone, Debug, Copy, Default)]
pub struct TestMacroCallExpansor;

impl<'ast> MacroCallExpansor<'ast> for TestMacroCallExpansor {
    type InstructionKind = ast::Test;

    fn expand_true_label(
        &self,
        outer_instr_kind: &'ast Self::InstructionKind,
    ) -> String {
        outer_instr_kind.next_true_label.content.clone()
    }

    fn expand_false_label(
        &self,
        outer_instr_kind: &'ast Self::InstructionKind,
    ) -> String {
        outer_instr_kind.next_false_label.content.clone()
    }

    fn expand_invalid_label(
        &self,
        outer_instr_kind: &'ast Self::InstructionKind,
    ) -> String {
        panic!("Erro de compilação");    
    }

    fn macro_type(&self) -> ast::MacroType {
        ast::MacroType::Test
    }
}