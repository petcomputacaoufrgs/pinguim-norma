use crate::compiler::ast;
use std::collections::HashMap;
use indexmap::IndexSet;
use crate::interpreter::program::{Program, Instruction, InstructionKind, Operation, OperationKind, Test, TestKind};
use crate::compiler::error::Diagnostics;

pub fn expand(ast: &ast::Program, diagnostics: &mut Diagnostics) -> Option<Program> {
    
    todo!()
}

struct Expansor<'ast> {
    precompileds: HashMap<String, PreCompiled>,     // macros prontas
    target_macros: IndexSet<String>,                // macros untouched
    working_macros: Vec<WorkingMacro>,              // macros em progresso
    ast: &'ast ast::Program,
}

impl<'ast> Expansor<'ast> {
    fn new(ast: &'ast ast::Program) -> Self {
        let target_macros = ast.macros.keys().cloned().collect();

        Expansor {
            precompileds: HashMap::new(),
            target_macros,
            working_macros: Vec::new(),
            ast,
        }
    }

    fn expand_program(&mut self) -> Option<Program> {
        self.expand_macros();
        self.expand_main()
    }

    fn expand_main(&mut self) -> Option<Program> {
        todo!()
    }

    fn expand_macros(&mut self) {
        while let Some(macro_name) = self.pop_target() {
            self.expand_target(macro_name);
            self.expand_working_macros();  
        }    
    }

    fn expand_target(&mut self, macro_name: String) {
        if let Some(macro_def) = self.get_macro(&macro_name) {
            let precompiled = PreCompiled::new(macro_def.name.clone(), macro_def.macro_type);
            let working_macro = WorkingMacro::new(precompiled);

            self.push_working_macro(working_macro);       
        }
    }

    // pega o nome da próxima macro a ser expandida
    fn pop_target(&mut self) -> Option<String> {
        self.target_macros.pop()
    }

    // pega uma macro da ast através do seu nome e retorna-a
    fn get_macro(&mut self, macro_name: &str) -> Option<&ast::Macro> {
        match self.ast.macros.get(macro_name) {
            Some(macro_def) => {
                Some(macro_def)
            },
            None => {
                panic!("Erro");
                None
            }
        }
    }

    fn push_working_macro(&mut self, working_macro: WorkingMacro) {
        self.working_macros.push(working_macro);
    }

    fn expand_working_macro(&mut self, mut working_macro: WorkingMacro) {
        let macro_def = self.ast.macros.get(&working_macro.precompiled.name.content).expect("Macro deve existir");

        while let Some((_, instr)) = macro_def.instr.get_index(working_macro.instr_index) {
            match self.expand_instruction(instr) {
                Ok(resulting_instr) => {
                    working_macro.precompiled.program.insert(resulting_instr);
                    working_macro.instr_index += 1;
                },
                Err(request) => {
                    self.push_working_macro(working_macro);
                    self.target_macros.remove(&request.macro_name);     // mover para expanding_instruction????
                    self.expand_target(request.macro_name);
                    break;
                } 
            }
        }
    }

    fn expand_working_macros(&mut self) {
        while let Some(working_macro) = self.pop_working_macro() {
            self.expand_working_macro(working_macro);
        } 
    }

    fn pop_working_macro(&mut self) -> Option<WorkingMacro> {
        self.working_macros.pop()
    }

    fn expand_instruction(&mut self, instr: &'ast ast::Instruction) -> Result<Instruction, ExpansionRequired> {
        let instr_kind = match &instr.instruction_type {
            ast::InstructionType::Operation(operation) => {
                InstructionKind::Operation(self.expand_operation(operation)?)
            },
            ast::InstructionType::Test(test) => {
                InstructionKind::Test(self.expand_test(test)?)
            }
        };

        Ok(Instruction {
            label: instr.label.content.clone(),
            kind: instr_kind,
        })
    }
    
    fn expand_operation(&mut self, operation: &'ast ast::Operation) -> Result<Operation, ExpansionRequired> {
        todo!()
    }

    fn expand_test(&mut self, test: &'ast ast::Test) -> Result<Test, ExpansionRequired> {
        todo!()
    }
}

struct ExpansionRequired {
    macro_name: String,
}

struct PreCompiled {
    name: ast::Symbol,
    macro_type: ast::MacroType,
    program: Program,  // Program do interpretador
}

impl PreCompiled {
    fn new(name: ast::Symbol, macro_type: ast::MacroType) -> Self {
        PreCompiled {
            name,
            macro_type,
            program: Program::empty(),
        }
    }
}

struct WorkingMacro {
    precompiled: PreCompiled,
    instr_index: usize,
}

impl WorkingMacro {
    fn new(precompiled: PreCompiled) -> Self {
        WorkingMacro {
            precompiled,
            instr_index: 0,
        }
    }
}


// se for outra chamada de macro, ve sem tem ela no precompiled
// se já tiver la faz a expansao direto
// se nao estiver, monta o expanding_macro com a macro atual e coloca na pilha
// tenta fazer a expansao do macro que dependemos (retira da do target_macros)
// quando chegar no final, coloca-o no precompiled
// enquanto tiver macros no expanding_macros e a medida que precisa compila os macros que são dependencias
// quando nao tiver mais nada no expanding_macros, tenta tirar outra do target_macros
// quando não tiver mais nada no target_macro, faz a expansão da main que é o mesmo algoritmo porém:

// se uma macro nao tiver precompilada, é por que nao existe aquela macro (por que todas ja foram precompiladas)

// expansao:
// 1- traduzir instruções
// 2- prefixar os rótulos dos macros aninhados (internos)
// 3- mapear rótulos de saída (true, false, invalidos) para rótulos externos