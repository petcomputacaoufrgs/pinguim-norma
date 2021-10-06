pub trait MacroCallExpansor<'ast> {
    type InstructionKind;
    
    fn expand_label(
        &self, 
        inner_precomp: &PreCompiled<'ast>,
        inner_next_label: &str,
        outer_label: &'ast ast::Symbol,
        outer_instr_kind: &'ast Self::InstructionKind
    ) -> String;

    fn macro_type(&self) -> ast::MacroType;
}

pub struct OperMacroCall;

impl<'ast> MacroCallExpansor<'ast> for OperMacroCall {
    type InstructionKind = ast::Operation;

    fn expand_label(
        &self, 
        inner_precomp: &PreCompiled<'ast>,
        inner_next_label: &str,
        outer_label: &'ast ast::Symbol,
        outer_operation: &'ast Self::InstructionKind
    ) -> String {
        if inner_precomp.program.is_label_valid(inner_next_label) {
            format!("{}.{}.{}", outer_label.content, inner_precomp.macro_data.name.content, inner_next_label)
        } else {
            // ver se eh true ou false
            outer_operation.next_label.content.clone()
        }
    }

    fn macro_type(&self) -> ast::MacroType {
        ast::MacroType::Operation
    }
}