use crate::compiler::ast;
use std::collections::HashMap;
use indexmap::IndexSet;
use crate::interpreter::program::{Program, Instruction, InstructionKind, Operation, OperationKind, Test, TestKind};
use crate::compiler::token::{BuiltInOperation, BuiltInTest};
use crate::compiler::error::Diagnostics;

pub fn expand(ast: &ast::Program, diagnostics: &mut Diagnostics) -> Option<Program> {
    
    todo!()
}

struct Expansor<'ast> {
    precompileds: HashMap<String, PreCompiled<'ast>>,     // macros prontas
    target_macros: IndexSet<String>,                      // macros untouched
    working_macros: Vec<WorkingMacro<'ast>>,              // macros em progresso
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

    // compila o programa todo
    fn expand_program(&mut self) -> Option<Program> {
        self.precompile_macros();
        self.expand_main()
    }

    // compila a main depois de precompilar os macros
    fn expand_main(&mut self) -> Option<Program> {
        todo!()
    }

    // precompila todas as macros
    fn precompile_macros(&mut self) {
        while let Some(macro_name) = self.pop_target_macro() {
            let working_macro = self.make_working_macro(&macro_name);
            self.push_working_macro(working_macro);
            self.precompile_working_macros();  
        }    
    }

    // pega uma nova macro target e coloca na pilha de macros a serem trabalhadas
    fn make_working_macro(&mut self, macro_name: &str) -> WorkingMacro<'ast> {
        let macro_def = self.get_macro(macro_name);
        let precompiled = PreCompiled::new(macro_def);
        WorkingMacro::new(precompiled)
    }

    // pega o nome da próxima macro a ser expandida
    fn pop_target_macro(&mut self) -> Option<String> {
        self.target_macros.pop()
    }

    // pega uma macro da ast através do seu nome e retorna-a
    fn get_macro(&mut self, macro_name: &str) -> &'ast ast::Macro {
        self.ast.macros.get(macro_name).expect("Macro em target macros deveria existir na ast")
    }

    // coloca um macro a ser trabalhado na pilha
    fn push_working_macro(&mut self, working_macro: WorkingMacro<'ast>) {
        self.working_macros.push(working_macro);
    }

    // expande o macro a ser trabalhado até que ele termine ou que precise ser pausado
    fn precompile_working_macro(&mut self, mut working_macro: WorkingMacro<'ast>) {
        let macro_def = self.get_macro(&working_macro.precompiled.macro_data.name.content);
        while let Some((_, instr)) = macro_def.instr.get_index(working_macro.instr_index) {
            match self.precompile_instruction(instr, &mut working_macro) {
                Ok(()) => working_macro.instr_index += 1,
                Err(request) => {
                    self.push_working_macro(working_macro);
                    self.push_working_macro(request.working_macro);
                    break;
                } 
            }
        }
    }

    // expande todas as macros na pilha de macros a serem trabalhadas
    fn precompile_working_macros(&mut self) {
        while let Some(working_macro) = self.pop_working_macro() {
            self.precompile_working_macro(working_macro);
        } 
    }

    // tira uma macro da pilha de macros a serem trabalhadas
    fn pop_working_macro(&mut self) -> Option<WorkingMacro<'ast>> {
        self.working_macros.pop()
    }

    // expande uma instrução colocando o resultado na working macro
    fn precompile_instruction(&mut self, instr: &'ast ast::Instruction, working_macro: &mut WorkingMacro<'ast>) -> Result<(), ExpansionRequired<'ast>> {
        match &instr.instruction_type {
            ast::InstructionType::Operation(operation) => {
                self.precompile_operation(&instr.label, operation, working_macro)?;
            },
            ast::InstructionType::Test(test) => {
                self.precompile_test(&instr.label, test, working_macro)?;
            }
        }

        Ok(())
    }
    
    // expande uma instrução do tipo operação
    fn precompile_operation(&mut self, label: &'ast ast::Symbol, operation: &'ast ast::Operation, working_macro: &mut WorkingMacro<'ast>) -> Result<(), ExpansionRequired<'ast>> {
        match &operation.oper_type {
            ast::OperationType::BuiltIn(builtin_oper, param) => {
                let oper_kind = self.precompile_builtin_oper(*builtin_oper, param);
                let runtime_oper = Operation {
                    kind: oper_kind,
                    next: operation.next_label.content.clone(),
                };

                let instruction = Instruction {
                    kind: InstructionKind::Operation(runtime_oper),
                    label: label.content.clone(),
                };

                working_macro.insert_instruction(instruction);

                Ok(())
            },
            ast::OperationType::Macro(macro_name, params) => {
                todo!()
            }
        }
    }

    // expande uma operação builtin
    fn precompile_builtin_oper(&mut self, builtin_oper: BuiltInOperation, param: &'ast ast::Symbol) -> OperationKind {
        match builtin_oper {
            BuiltInOperation::Inc => OperationKind::Inc(param.content.clone()),
            BuiltInOperation::Dec => OperationKind::Dec(param.content.clone()),
        }
    }

    // expande uma operação que é outra chamada de macro
    fn precompile_oper_macro_call(&mut self, macro_name: &'ast ast::Symbol, params: &'ast [ast::MacroParam], working_macro: &mut WorkingMacro<'ast>) -> Result<(), ExpansionRequired<'ast>> {
        if let Some(precompiled_macro) = self.precompileds.get(&macro_name.content).cloned() {
            self.expand_oper_macro(precompiled_macro, params, working_macro)

        } else if self.target_macros.remove(&macro_name.content) {
            let working_macro = self.make_working_macro(&macro_name.content);  
            Err(ExpansionRequired { working_macro })    
<
        } else if self.ast.macros.contains_key(&macro_name.content) {
            panic!("Recusãaaaaaaaaaaaao /o\\")
        } else {
            panic!("Macro não existe")
        }
    }

    fn expand_oper_macro(&mut self, precompiled_macro: PreCompiled<'ast>, params: &'ast [ast::MacroParam], working_macro: &mut WorkingMacro<'ast>) -> Result<(), ExpansionRequired<'ast>> {
        for instr in precompiled_macro.program.instructions() {
            working_macro.insert_instruction(self.expand_instruction(instr, label_outer, label_next_outer, inner_precomp));
        }

        todo!()
    }

    fn expand_instruction(&mut self, instr: &Instruction, outer_label: &'ast ast::Symbol, outer_next_label: &'ast ast::Symbol, inner_precomp: &PreCompiled<'ast>) -> Instruction {
        todo!()
    }  

    fn make_parameters(&mut self, call_params: &'ast [ast::MacroParam], def_params: &'ast [ast::Symbol]) -> HashMap<&'ast ast::Symbol, &'ast ast::Symbol> {
        todo!()
    }

    fn precompile_test(&mut self, label: &'ast ast::Symbol, test: &'ast ast::Test, working_macro: &mut WorkingMacro<'ast>) -> Result<(), ExpansionRequired<'ast>> {
        todo!()
    }

    fn precompile_builtin_test(&mut self, builtin_test: BuiltInTest, param: &'ast ast::Symbol) -> TestKind {
        match builtin_test {
            BuiltInTest::Zero => TestKind::Zero(param.content.clone()),
        }
    }
}

#[derive(Clone, Debug)]
struct ExpansionRequired<'ast> {
    working_macro: WorkingMacro<'ast>,
}

#[derive(Clone, Debug)]
struct PreCompiled<'ast> {
    macro_data: &'ast ast::Macro,
    program: Program,  // Program do interpretador
}

impl<'ast> PreCompiled<'ast> {
    fn new(macro_data: &'ast ast::Macro) -> Self {
        PreCompiled {
            macro_data,
            program: Program::empty(),
        }
    }
}

#[derive(Clone, Debug)]
struct WorkingMacro<'ast> {
    precompiled: PreCompiled<'ast>,
    instr_index: usize,
}

impl<'ast> WorkingMacro<'ast> {
    fn new(precompiled: PreCompiled<'ast>) -> Self {
        WorkingMacro {
            precompiled,
            instr_index: 0,
        }
    }

    fn insert_instruction(&mut self, instruction: Instruction) {
        self.precompiled.program.insert(instruction);
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