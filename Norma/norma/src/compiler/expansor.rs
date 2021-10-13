mod macro_call;

use crate::{
    compiler::{
        ast,
        error::Diagnostics,
        token::{BuiltInOperation, BuiltInTest},
    },
    interpreter::program::{
        Instruction,
        InstructionKind,
        Operation,
        OperationKind,
        Program,
        Test,
        TestKind,
    },
};
use indexmap::IndexSet;
use std::collections::HashMap;
use macro_call::{MacroCallExpansor, OperMacroCallExpansor, TestMacroCallExpansor};

pub fn expand(
    ast: &ast::Program,
    diagnostics: &mut Diagnostics,
) -> Option<Program> {
    let mut expansor = Expansor::new(ast);
    expansor.expand_program()
}

struct Expansor<'ast> {
    precompileds: HashMap<String, PreCompiledCode<'ast>>, // macros prontas
    target_macros: IndexSet<String>,                  // macros untouched
    working_macros: Vec<WorkingCode<'ast>>,          // macros em progresso
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
        // criar Program vazio ~"working macro"
        let mut main_code = self.create_working_code("main");

        // iterar pelas instrucoes da ast da main
        for (_, instr) in &self.ast.main.code {

            // para cada instrucao chamar precompile_instruction() com .expect() caso retorne ExpansionRequired
            self.precompile_instruction(&instr, &mut main_code).expect("Macros existentes já foram compiladas");
        }

        // retornar o programa no final
        Some(main_code.precompiled.program)
    }

    // precompila todas as macros
    fn precompile_macros(&mut self) {
        while let Some(macro_name) = self.pop_target_macro() {
            let working_macro = self.create_working_code(&macro_name);
            self.push_working_macro(working_macro);
            self.precompile_working_macros();
        }
    }

    // pega uma nova macro target e coloca na pilha de macros a serem
    // trabalhadas
    fn create_working_code(&mut self, macro_name: &str) -> WorkingCode<'ast> {
        let macro_def = self.get_macro(macro_name);
        let precompiled = PreCompiledCode::new(macro_def);
        WorkingCode::new(precompiled)
    }

    // pega o nome da próxima macro a ser expandida
    fn pop_target_macro(&mut self) -> Option<String> {
        self.target_macros.pop()
    }

    // pega uma macro da ast através do seu nome e retorna-a
    fn get_macro(&mut self, macro_name: &str) -> &'ast ast::Macro {
        self.ast
            .macros
            .get(macro_name)
            .expect("Macro em target macros deveria existir na ast")
    }

    // coloca um macro a ser trabalhado na pilha
    fn push_working_macro(&mut self, working_macro: WorkingCode<'ast>) {
        self.working_macros.push(working_macro);
    }

    // expande o macro a ser trabalhado até que ele termine ou que precise ser
    // pausado
    fn precompile_working_macro(
        &mut self,
        mut working_macro: WorkingCode<'ast>,
    ) {
        let macro_def =
            self.get_macro(&working_macro.precompiled.macro_data.name.content);
        while let Some((_, instr)) =
            macro_def.instr.get_index(working_macro.instr_index)
        {
            match self.precompile_instruction(instr, &mut working_macro) {
                Ok(()) => working_macro.instr_index += 1,
                Err(request) => {
                    self.push_working_macro(working_macro);
                    self.push_working_macro(request.working_code);
                    break;
                },
            }
        }
    }

    // expande todas as macros na pilha de macros a serem trabalhadas
    fn precompile_working_macros(&mut self) {
        while let Some(working_macro) = self.pop_working_macros() {
            self.precompile_working_macro(working_macro);
        }
    }

    // tira uma macro da pilha de macros a serem trabalhadas
    fn pop_working_macros(&mut self) -> Option<WorkingCode<'ast>> {
        self.working_macros.pop()
    }

    // expande uma instrução colocando o resultado na working macro
    fn precompile_instruction(
        &mut self,
        instr: &'ast ast::Instruction,
        working_code: &mut WorkingCode<'ast>,
    ) -> Result<(), ExpansionRequired<'ast>> {
        match &instr.instruction_type {
            ast::InstructionType::Operation(operation) => {
                self.precompile_operation(
                    &instr.label,
                    operation,
                    working_code,
                )?;
            },
            ast::InstructionType::Test(test) => {
                self.precompile_test(&instr.label, test, working_code)?;
            },
        }

        Ok(())
    }

    // expande uma instrução do tipo operação
    fn precompile_operation(
        &mut self,
        label: &'ast ast::Symbol,
        operation: &'ast ast::Operation,
        working_code: &mut WorkingCode<'ast>,
    ) -> Result<(), ExpansionRequired<'ast>> {
        match &operation.oper_type {
            ast::OperationType::BuiltIn(builtin_oper, param) => {
                let oper_kind =
                    self.precompile_builtin_oper(*builtin_oper, param);
                let runtime_oper = Operation {
                    kind: oper_kind,
                    next: operation.next_label.content.clone(),
                };

                let instruction = Instruction {
                    kind: InstructionKind::Operation(runtime_oper),
                    label: label.content.clone(),
                };

                working_code.insert_instruction(instruction);

                Ok(())
            },
            ast::OperationType::Macro(macro_name, params) => self
                .precompile_macro_call(
                    label,
                    operation,
                    &OperMacroCallExpansor,
                    macro_name,
                    params,
                    working_code,
                ),
        }
    }
    
    // expande uma operação builtin
    fn precompile_builtin_oper(
        &mut self,
        builtin_oper: BuiltInOperation,
        param: &'ast ast::Symbol,
    ) -> OperationKind {
        match builtin_oper {
            BuiltInOperation::Inc => OperationKind::Inc(param.content.clone()),
            BuiltInOperation::Dec => OperationKind::Dec(param.content.clone()),
        }
    }

    fn precompile_test(
        &mut self,
        label: &'ast ast::Symbol,
        test: &'ast ast::Test,
        working_code: &mut WorkingCode<'ast>,
    ) -> Result<(), ExpansionRequired<'ast>> {
        match &test.test_type {
            ast::TestType::BuiltIn(builtin_test, param) => {
                let test_kind = self.precompile_builtin_test(*builtin_test, param);
                let runtime_test = Test {
                    kind: test_kind,
                    next_then: test.next_true_label.content.clone(),
                    next_else: test.next_false_label.content.clone(),
                };

                let instruction = Instruction {
                    kind: InstructionKind::Test(runtime_test),
                    label: label.content.clone(),
                };

                working_code.insert_instruction(instruction);

                Ok(())
            },
            ast::TestType::Macro(macro_name, params) => self
                .precompile_macro_call(
                    label,
                    test,
                    &TestMacroCallExpansor,
                    macro_name,
                    params,
                    working_code,
                ),
        }
    }

    fn precompile_builtin_test(
        &mut self,
        builtin_test: BuiltInTest,
        param: &'ast ast::Symbol,
    ) -> TestKind {
        match builtin_test {
            BuiltInTest::Zero => TestKind::Zero(param.content.clone()),
        }
    }

    // expande uma operação que é outra chamada de macro
    fn precompile_macro_call<E>(
        &mut self,
        label: &'ast ast::Symbol,
        instr_kind: &'ast E::InstructionKind,
        call_expansor: &E,
        macro_name: &'ast ast::Symbol,
        arguments: &'ast [ast::MacroArgument],
        working_code: &mut WorkingCode<'ast>,
    ) -> Result<(), ExpansionRequired<'ast>> 
    where
        E: MacroCallExpansor<'ast>
    {
        if let Some(precompiled_macro) =
            self.precompileds.get(&macro_name.content).cloned()
        {
            if call_expansor.macro_type() == precompiled_macro.macro_data.macro_type {
                self.expand_macro(precompiled_macro, label, instr_kind, call_expansor, arguments, working_code);
            } else {
                panic!("Erro")
            }

            Ok(())

        } else if self.target_macros.remove(&macro_name.content) {
            let working_code = self.create_working_code(&macro_name.content);
            Err(ExpansionRequired { working_code })
        } else if self.ast.macros.contains_key(&macro_name.content) {
            panic!("Recursãaaaaaaaaaaaao /o\\")
        } else {
            panic!("Macro não existe")
        }
    }

    fn expand_macro<E>(
        &mut self,
        inner_precomp: PreCompiledCode<'ast>,
        outer_label: &'ast ast::Symbol,
        outer_instr_kind: &'ast E::InstructionKind,
        call_expansor: &E,
        arguments: &'ast [ast::MacroArgument],
        working_code: &mut WorkingCode<'ast>,
    ) 
    where
        E: MacroCallExpansor<'ast>
    {
        let params_map = self.map_params_to_args(
            &inner_precomp.macro_data.parameters,
            arguments,
        );

        for instr in inner_precomp.program.instructions() {
            working_code.insert_instruction(self.expand_instr(
                &params_map,
                instr,
                outer_label,
                outer_instr_kind,
                call_expansor,
                &inner_precomp,
            ));
        }
    }

    /// Expande uma instrução de um macro pré-compilado para dentro de um macro
    /// a ser compilado.
    ///
    /// - `params_map` mapeia parâmetros formais do macro pré-compilado para os
    ///   argumentos passados. Os Argumentos se encontram na chamada de fora.
    /// - `instr` é a instrução a ser compilada.
    /// - `outer_label` é o label da instrução que chama a macro de dentro. Será
    ///   prefixado a todos labels da macro de dentro.
    /// - `outer_next_label` é o label após o `goto` na instrução da macro de
    ///   fora. Ròtulos de saída da macro de dentro serão remapeados para esse
    ///   label.
    /// - `inner_precomp` é a precompilação da macro interna.
    ///
    /// TODO: por enquanto só considera chamadas de macro operação.
    fn expand_instr<E>(
        &mut self,
        params_map: &HashMap<&'ast str, &'ast str>,
        instr: &Instruction,
        outer_label: &'ast ast::Symbol,
        outer_instr_kind: &'ast E::InstructionKind,
        call_expansor: &E,
        inner_precomp: &PreCompiledCode<'ast>,
    ) -> Instruction 
    where
        E: MacroCallExpansor<'ast>
    {
        let instr_kind = match &instr.kind {
            InstructionKind::Operation(operation) => {
                let expanded_operation = self.expand_oper_instr(
                    params_map,
                    operation,
                    outer_label,
                    outer_instr_kind,
                    call_expansor,
                    inner_precomp,
                );

                InstructionKind::Operation(expanded_operation)
            },

            InstructionKind::Test(test) => {
                let expanded_test = self.expand_test_instr(
                    params_map,
                    test,
                    outer_label,
                    outer_instr_kind,
                    call_expansor,
                    inner_precomp,
                );

                InstructionKind::Test(expanded_test)
            },
        };

        Instruction {
            label: self.expand_label(inner_precomp, &instr.label, outer_label, outer_instr_kind, call_expansor),
            kind: instr_kind
        }
    }

    fn expand_oper_instr<E>(
        &mut self,
        params_map: &HashMap<&'ast str, &'ast str>,
        operation: &Operation,
        outer_label: &'ast ast::Symbol,
        outer_instr_kind: &'ast E::InstructionKind,
        call_expansor: &E,
        inner_precomp: &PreCompiledCode<'ast>,
    ) -> Operation 
    where
        E: MacroCallExpansor<'ast>
    {
        Operation {
            kind: self.expand_oper_kind(&operation.kind, params_map),
            next: self.expand_label(inner_precomp, &operation.next, outer_label, outer_instr_kind, call_expansor),
        }
    }

    fn expand_test_instr<E>(
        &mut self,
        params_map: &HashMap<&'ast str, &'ast str>,
        test: &Test,
        outer_label: &'ast ast::Symbol,
        outer_instr_kind: &'ast E::InstructionKind,
        call_expansor: &E,
        inner_precomp: &PreCompiledCode<'ast>,
    ) -> Test 
    where
        E: MacroCallExpansor<'ast>
    {
        Test {
            kind: self.expand_test_kind(&test.kind, params_map),
            next_then: self.expand_label(inner_precomp, &test.next_then, outer_label, outer_instr_kind, call_expansor),
            next_else: self.expand_label(inner_precomp, &test.next_else, outer_label, outer_instr_kind, call_expansor)
        }
    }

    fn expand_label<E>(
        &self, 
        inner_precomp: &PreCompiledCode<'ast>, 
        inner_next_label: &str,
        outer_label: &'ast ast::Symbol, 
        outer_instr_kind: &'ast E::InstructionKind,
        call_expansor: &E,
    ) -> String 
    where
        E: MacroCallExpansor<'ast>
    {
        if inner_precomp.program.is_label_valid(inner_next_label) {
            format!("{}.{}.{}", outer_label.content, inner_precomp.macro_data.name.content, inner_next_label)
        } else if self.is_true_label(inner_next_label) {
            call_expansor.expand_true_label(outer_instr_kind)
        } else if self.is_false_label(inner_next_label) {
            call_expansor.expand_false_label(outer_instr_kind)
        } else {
            call_expansor.expand_invalid_label(outer_instr_kind)
        }
    }

    fn expand_oper_kind(&self, operation_kind: &OperationKind, params_map: &HashMap<&'ast str, &'ast str>) -> OperationKind {
        operation_kind.map_registers(|register| self.map_param_to_arg(params_map, register))
    }

    fn expand_test_kind(&self, test_kind: &TestKind, params_map: &HashMap<&'ast str, &'ast str>) -> TestKind {
        test_kind.map_registers(|register| self.map_param_to_arg(params_map, register))
    }

    fn map_param_to_arg(
        &self, 
        params_map: &HashMap<&'ast str, &'ast str>, 
        register: &str
    ) -> String {
        params_map.get(register).map_or_else(|| register.to_string(), |arg| arg.to_string())
    }

    /// Produz mapeamento de nomes de registradores em parâmetros formais
    /// (chave) para registradores passados como argumentos (valor).
    fn map_params_to_args(
        &self,
        def_params: &'ast [ast::Symbol],
        args: &'ast [ast::MacroArgument],
    ) -> HashMap<&'ast str, &'ast str> {
        if def_params.len() > args.len() {
            panic!("Missing arguments")
        } else if def_params.len() < args.len() {
            panic!("Too much arguments")
        } else {
            def_params
                .iter()
                .zip(args)
                .map(|(param, arg)| (param.content.as_str(), self.expect_register_arg(arg)))
                .collect()
        }
    }

    fn expect_register_arg(&self, macro_argument: &'ast ast::MacroArgument) -> &'ast str {
        match macro_argument {
            ast::MacroArgument::Register(register) => &register.content,
            ast::MacroArgument::Number(constant) => panic!("aaaaaa") 
        }
    }

    fn is_true_label(&self, label: &str) -> bool {
        label == "true"
    }

    fn is_false_label(&self, label: &str) -> bool {
        label == "false"
    }    
}

#[derive(Clone, Debug)]
struct ExpansionRequired<'ast> {
    working_code: WorkingCode<'ast>,
}

#[derive(Clone, Debug)]
struct PreCompiledCode<'ast> {
    macro_data: &'ast ast::Macro,
    program: Program,
}

impl<'ast> PreCompiledCode<'ast> {
    fn new(macro_data: &'ast ast::Macro) -> Self {
        PreCompiledCode { macro_data, program: Program::empty() }
    }
}

#[derive(Clone, Debug)]
struct WorkingCode<'ast> {
    precompiled: PreCompiledCode<'ast>,
    instr_index: usize,
}

impl<'ast> WorkingCode<'ast> {
    fn new(precompiled: PreCompiledCode<'ast>) -> Self {
        WorkingCode { precompiled, instr_index: 0 }
    }

    fn insert_instruction(&mut self, instruction: Instruction) {
        self.precompiled.program.insert(instruction);
    }
}