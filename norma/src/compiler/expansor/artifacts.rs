use crate::{
    compiler::parser::ast,
    interpreter::program::{Instruction, Program},
};
use std::{collections::HashMap, fmt};

#[derive(Clone, Debug)]
pub struct ExpansionRequired<'ast> {
    ///
    /// - `working_macro`: macro que necessita de expansão
    pub working_macro: WorkingMacro<'ast>,
}

#[derive(Clone, Debug)]
pub struct PreCompiled<'ast> {
    ///
    /// - `macro_data`: informações originais da macro
    macro_data: &'ast ast::Macro,
    ///
    /// - `program`: conjunto de instruções e labels da macro
    program: Program,
}

impl<'ast> PreCompiled<'ast> {
    /// Cria uma nova estrutura PreCompiled
    /// 
    /// - `macro_data`: informações originais da macro
    pub fn new(macro_data: &'ast ast::Macro) -> Self {
        PreCompiled { macro_data, program: Program::empty() }
    }

    /// Retorna as informações originais da macro
    pub fn macro_data(&self) -> &'ast ast::Macro {
        self.macro_data
    }

    /// Retorna o conjunto de instruções e labels da macro
    pub fn program(&self) -> &Program {
        &self.program
    }
}

#[derive(Clone, Debug)]
pub struct WorkingCode {
    ///
    /// - `program`: conjunto de instruções do código que está sendo precompilado
    program: Program,
    ///
    /// - `expanded_labels`: mapeamentos dos labels originais para os novos labels expandidos do programa
    expanded_labels: HashMap<String, String>,
}

impl WorkingCode {
    /// Cria uma nova estrutura WorkingCode
    pub fn new() -> Self {
        Self { program: Program::empty(), expanded_labels: HashMap::new() }
    }

    /// Insere uma nova instrução precompilada no programa
    /// 
    /// - `instruction`: instrução precompilada
    pub fn insert_instr(&mut self, instruction: Instruction) {
        self.program.insert(instruction)
    }

    /// Insere novo conjunto chave-valor de label expandida
    /// 
    /// - `old_label`: label antigo
    /// - `new_label`: label expandido
    pub fn insert_expansion(&mut self, old_label: String, new_label: String) {
        self.expanded_labels.insert(old_label, new_label);
    }

    /// Finaliza o programa da estrutura, expandindo definitivamente todos os labels do programa a partir do mapeamento
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
    ///
    /// - `code`: programa da macro com mapeamento de labels expandidos
    code: WorkingCode,
    ///
    /// - `macro_data`: informações originais da macro
    macro_data: &'ast ast::Macro,
    ///
    /// - `instr_index`: índice da instrução na qual a precompilação da macro foi pausada em prol de outra
    instr_index: usize,
}

impl<'ast> WorkingMacro<'ast> {
    /// Cria uma nova estrutura WorkingCode
    /// 
    /// - `macro_data`: informações originais da macro
    pub fn new(macro_data: &'ast ast::Macro) -> Self {
        WorkingMacro { code: WorkingCode::new(), macro_data, instr_index: 0 }
    }

    /// Retorna uma instância imutável do código da WorkingMacro
    pub fn code(&self) -> &WorkingCode {
        &self.code
    }

    /// Retorna uma instância mutável do código da WorkingMacro
    pub fn code_mut(&mut self) -> &mut WorkingCode {
        &mut self.code
    }

    /// Retorna uma instância imutável das informações originais da macro
    pub fn macro_data(&self) -> &'ast ast::Macro {
        self.macro_data
    }

    /// Retorna a instrução em que a precompilação foi pausada
    pub fn curr_instr(&self) -> Option<&'ast ast::Instruction> {
        self.macro_data
            .instr
            .get_index(self.instr_index)
            .map(|(_, instr)| instr)
    }

    /// Avança para a próxima instrução a ser precompilada
    pub fn next_instr(&mut self) {
        self.instr_index += 1;
    }

    /// Termina a precompilação e retorna uma estrutura PreCompiled
    pub fn finish(self) -> PreCompiled<'ast> {
        let program = self.code.finish();
        PreCompiled { program, macro_data: self.macro_data }
    }
}
