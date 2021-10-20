use super::label;
use crate::{
    compiler::{error::Diagnostics, parser::ast},
    interpreter::program::{Instruction, Program},
};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct ExpansionRequired<'ast> {
    pub working_macro: WorkingMacro<'ast>,
}

#[derive(Clone, Debug)]
pub struct PreCompiled<'ast> {
    macro_data: &'ast ast::Macro,
    program: Program, // Program do interpretador
}

impl<'ast> PreCompiled<'ast> {
    pub fn new(macro_data: &'ast ast::Macro) -> Self {
        PreCompiled { macro_data, program: Program::empty() }
    }

    pub fn macro_data(&self) -> &'ast ast::Macro {
        self.macro_data
    }

    pub fn program(&self) -> &Program {
        &self.program
    }
}

#[derive(Clone, Debug)]
pub struct WorkingCode {
    program: Program,
    expanded_labels: HashMap<String, String>,
}

impl WorkingCode {
    pub fn new() -> Self {
        Self { program: Program::empty(), expanded_labels: HashMap::new() }
    }

    pub fn insert_instr(&mut self, instruction: Instruction) {
        self.program.insert(instruction)
    }

    pub fn insert_expansion(&mut self, old_label: String, new_label: String) {
        self.expanded_labels.insert(old_label, new_label);
    }

    pub fn finish_main(self, diagnostics: &mut Diagnostics) -> Program {
        self.finish(diagnostics, label::validate_for_main)
    }

    pub fn finish_oper_macro(self, diagnostics: &mut Diagnostics) -> Program {
        self.finish(diagnostics, label::validate_for_oper_macro)
    }

    pub fn finish_test_macro(self, diagnostics: &mut Diagnostics) -> Program {
        self.finish(diagnostics, label::validate_for_test_macro)
    }

    fn finish<F>(
        self,
        diagnostics: &mut Diagnostics,
        mut validate_label: F,
    ) -> Program
    where
        F: FnMut(&str, &Program, &mut Diagnostics),
    {
        let mut program = self.program;
        let expanded_labels = self.expanded_labels;

        program.collect_labels(|label| {
            validate_label(label, &program, diagnostics)
        });

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
    code: WorkingCode,
    macro_data: &'ast ast::Macro,
    instr_index: usize,
}

impl<'ast> WorkingMacro<'ast> {
    pub fn new(macro_data: &'ast ast::Macro) -> Self {
        WorkingMacro { code: WorkingCode::new(), macro_data, instr_index: 0 }
    }

    pub fn code(&self) -> &WorkingCode {
        &self.code
    }

    pub fn code_mut(&mut self) -> &mut WorkingCode {
        &mut self.code
    }

    pub fn macro_data(&self) -> &'ast ast::Macro {
        self.macro_data
    }

    pub fn curr_instr(&self) -> Option<&'ast ast::Instruction> {
        self.macro_data
            .instr
            .get_index(self.instr_index)
            .map(|(_, instr)| instr)
    }

    pub fn next_instr(&mut self) {
        self.instr_index += 1;
    }

    pub fn finish(self, diagnostics: &mut Diagnostics) -> PreCompiled<'ast> {
        let program = match self.macro_data.macro_type {
            ast::MacroType::Operation => {
                self.code.finish_oper_macro(diagnostics)
            },
            ast::MacroType::Test => self.code.finish_test_macro(diagnostics),
        };
        PreCompiled { program, macro_data: self.macro_data }
    }
}
