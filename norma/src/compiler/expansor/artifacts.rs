use crate::{compiler::parser::ast, interpreter::program::Program};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct ExpansionRequired<'ast> {
    pub working_macro: WorkingMacro<'ast>,
}

#[derive(Clone, Debug)]
pub struct PreCompiled<'ast> {
    pub macro_data: &'ast ast::Macro,
    pub program: Program, // Program do interpretador
}

impl<'ast> PreCompiled<'ast> {
    pub fn new(macro_data: &'ast ast::Macro) -> Self {
        PreCompiled { macro_data, program: Program::empty() }
    }
}

#[derive(Clone, Debug)]
pub struct WorkingCode {
    pub program: Program,
    pub expanded_labels: HashMap<String, String>,
}

impl WorkingCode {
    pub fn new() -> Self {
        Self { program: Program::empty(), expanded_labels: HashMap::new() }
    }

    pub fn insert_expansion(&mut self, old_label: String, new_label: String) {
        self.expanded_labels.insert(old_label, new_label);
    }

    pub fn finish(self) -> Program {
        let mut program = self.program;
        let expanded_labels = self.expanded_labels;

        for instruction in &mut program {
            instruction.kind.rename_labels(|label: &mut String| {
                if let Some(new_label) = expanded_labels.get(label) {
                    label.clone_from(new_label);
                }
            })
        }

        program
    }
}

#[derive(Clone, Debug)]
pub struct WorkingMacro<'ast> {
    pub code: WorkingCode,
    pub macro_data: &'ast ast::Macro,
    pub instr_index: usize,
}

impl<'ast> WorkingMacro<'ast> {
    pub fn new(macro_data: &'ast ast::Macro) -> Self {
        WorkingMacro { code: WorkingCode::new(), macro_data, instr_index: 0 }
    }

    pub fn finish(self) -> PreCompiled<'ast> {
        let program = self.code.finish();
        PreCompiled { program, macro_data: self.macro_data }
    }
}
