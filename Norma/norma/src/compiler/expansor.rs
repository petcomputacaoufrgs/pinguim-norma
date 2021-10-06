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

pub fn expand(
    ast: &ast::Program,
    diagnostics: &mut Diagnostics,
) -> Option<Program> {
    todo!()
}

struct Expansor<'ast> {
    precompileds: HashMap<String, PreCompiled<'ast>>, // macros prontas
    target_macros: IndexSet<String>,                  // macros untouched
    working_macros: Vec<WorkingMacro<'ast>>,          // macros em progresso
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

    // pega uma nova macro target e coloca na pilha de macros a serem
    // trabalhadas
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
        self.ast
            .macros
            .get(macro_name)
            .expect("Macro em target macros deveria existir na ast")
    }

    // coloca um macro a ser trabalhado na pilha
    fn push_working_macro(&mut self, working_macro: WorkingMacro<'ast>) {
        self.working_macros.push(working_macro);
    }

    // expande o macro a ser trabalhado até que ele termine ou que precise ser
    // pausado
    fn precompile_working_macro(
        &mut self,
        mut working_macro: WorkingMacro<'ast>,
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
                    self.push_working_macro(request.working_macro);
                    break;
                },
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
    fn precompile_instruction(
        &mut self,
        instr: &'ast ast::Instruction,
        working_macro: &mut WorkingMacro<'ast>,
    ) -> Result<(), ExpansionRequired<'ast>> {
        match &instr.instruction_type {
            ast::InstructionType::Operation(operation) => {
                self.precompile_operation(
                    &instr.label,
                    operation,
                    working_macro,
                )?;
            },
            ast::InstructionType::Test(test) => {
                self.precompile_test(&instr.label, test, working_macro)?;
            },
        }

        Ok(())
    }

    // ----------------- INSTRUCTION OPERATION (do) ---------------------

    // expande uma instrução do tipo operação
    fn precompile_operation(
        &mut self,
        label: &'ast ast::Symbol,
        operation: &'ast ast::Operation,
        working_macro: &mut WorkingMacro<'ast>,
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

                working_macro.insert_instruction(instruction);

                Ok(())
            },
            ast::OperationType::Macro(macro_name, params) => self
                .precompile_oper_macro_call(
                    label,
                    operation,
                    macro_name,
                    params,
                    working_macro,
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

    // expande uma operação que é outra chamada de macro
    fn precompile_oper_macro_call(
        &mut self,
        label: &'ast ast::Symbol,
        operation: &'ast ast::Operation,
        macro_name: &'ast ast::Symbol,
        arguments: &'ast [ast::MacroArgument],
        working_macro: &mut WorkingMacro<'ast>,
    ) -> Result<(), ExpansionRequired<'ast>> {
        if let Some(precompiled_macro) =
            self.precompileds.get(&macro_name.content).cloned()
        {
            match precompiled_macro.macro_data.macro_type {
                ast::MacroType::Operation => self.expand_oper_macro(
                    precompiled_macro,
                    label,
                    operation,
                    arguments,
                    working_macro,
                ),
                ast::MacroType::Test => panic!("erro dps"),
            }

            Ok(())

        } else if self.target_macros.remove(&macro_name.content) {
            let working_macro = self.make_working_macro(&macro_name.content);
            Err(ExpansionRequired { working_macro })
        } else if self.ast.macros.contains_key(&macro_name.content) {
            panic!("Recursãaaaaaaaaaaaao /o\\")
        } else {
            panic!("Macro não existe")
        }
    }

    fn expand_oper_macro(
        &mut self,
        inner_precomp: PreCompiled<'ast>,
        outer_label: &'ast ast::Symbol,
        outer_operation: &'ast ast::Operation,
        arguments: &'ast [ast::MacroArgument],
        working_macro: &mut WorkingMacro<'ast>,
    ) {
        let params_map = self.map_params_to_args(
            &inner_precomp.macro_data.parameters,
            arguments,
        );

        for instr in inner_precomp.program.instructions() {
            working_macro.insert_instruction(self.expand_oper_macro_instr(
                &params_map,
                instr,
                outer_label,
                outer_operation,
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
    fn expand_oper_macro_instr(
        &mut self,
        params_map: &HashMap<&'ast str, &'ast str>,
        instr: &Instruction,
        outer_label: &'ast ast::Symbol,
        outer_operation: &'ast ast::Operation,
        inner_precomp: &PreCompiled<'ast>,
    ) -> Instruction {
        let instr_kind = match &instr.kind {
            InstructionKind::Operation(operation) => {
                let expanded_operation = self.expand_oper_macro_oper_instr(
                    params_map,
                    operation,
                    outer_label,
                    outer_operation,
                    inner_precomp,
                );

                InstructionKind::Operation(expanded_operation)
            },

            InstructionKind::Test(test) => {
                let expanded_test = self.expand_oper_macro_test_instr(
                    params_map,
                    test,
                    outer_label,
                    outer_operation,
                    inner_precomp,
                );

                InstructionKind::Test(expanded_test)
            },
        };

        Instruction {
            label: self.expand_oper_macro_label(inner_precomp, &instr.label, outer_label, outer_operation),
            kind: instr_kind
        }
    }

    fn expand_oper_macro_oper_instr(
        &mut self,
        params_map: &HashMap<&'ast str, &'ast str>,
        operation: &Operation,
        outer_label: &'ast ast::Symbol,
        outer_operation: &'ast ast::Operation,
        inner_precomp: &PreCompiled<'ast>,
    ) -> Operation {

        Operation {
            kind: self.expand_oper_kind(&operation.kind, params_map),
            next: self.expand_oper_macro_label(inner_precomp, &operation.next, outer_label, outer_operation),
        }
    }

    fn expand_oper_macro_test_instr(
        &mut self,
        params_map: &HashMap<&'ast str, &'ast str>,
        test: &Test,
        outer_label: &'ast ast::Symbol,
        outer_operation: &'ast ast::Operation,
        inner_precomp: &PreCompiled<'ast>,
    ) -> Test {

        Test {
            kind: self.expand_test_kind(&test.kind, params_map),
            next_then: self.expand_oper_macro_label(inner_precomp, &test.next_then, outer_label, outer_operation),
            next_else: self.expand_oper_macro_label(inner_precomp, &test.next_else, outer_label, outer_operation)
        }
    }

    fn expand_oper_kind(&self, operation_kind: &OperationKind, params_map: &HashMap<&'ast str, &'ast str>) -> OperationKind {
        operation_kind.map_registers(|register| self.map_param_to_arg(params_map, register))
    }

    fn expand_test_kind(&self, test_kind: &TestKind, params_map: &HashMap<&'ast str, &'ast str>) -> TestKind {
        test_kind.map_registers(|register| self.map_param_to_arg(params_map, register))
    }

    fn expand_oper_macro_label(
        &self, 
        inner_precomp: &PreCompiled<'ast>, 
        inner_next_label: &str, 
        outer_label: &'ast ast::Symbol, 
        outer_operation: &'ast ast::Operation
    ) -> String {
        if inner_precomp.program.is_label_valid(inner_next_label) {
            format!("{}.{}.{}", outer_label.content, inner_precomp.macro_data.name.content, inner_next_label)
        } else {
            outer_operation.next_label.content.clone()
        }
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

// ----------------- INSTRUCTION TEST (if) ---------------------

    fn precompile_test(
        &mut self,
        label: &'ast ast::Symbol,
        test: &'ast ast::Test,
        working_macro: &mut WorkingMacro<'ast>,
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

                working_macro.insert_instruction(instruction);

                Ok(())
            },
            ast::TestType::Macro(macro_name, params) => self
                .precompile_test_macro_call(
                    label,
                    test,
                    macro_name,
                    params,
                    working_macro,
                ),
        }
    }

    fn precompile_test_macro_call(
        &mut self, 
        label: &'ast ast::Symbol, 
        test: &'ast ast::Test,
        macro_name: &'ast ast::Symbol,
        arguments: &'ast [ast::MacroArgument],
        working_macro: &mut WorkingMacro<'ast>,
    ) -> Result<(), ExpansionRequired<'ast>> {
        if let Some(precompiled_macro) = 
            self.precompileds.get(&macro_name.content).cloned() 
        {
            match precompiled_macro.macro_data.macro_type {
                ast::MacroType::Operation => panic!("erro"),
                ast::MacroType::Test => {
                    self.expand_test_macro(
                        precompiled_macro,
                        label,
                        test,
                        arguments,
                        working_macro,
                    )
                }
            }

            Ok(())

        } else if self.target_macros.remove(&macro_name.content) {
            let working_macro = self.make_working_macro(&macro_name.content);
            Err(ExpansionRequired { working_macro })
        } else if self.ast.macros.contains_key(&macro_name.content) {
            panic!("rEcURsÃoO")
        } else {
            panic!("panico :O")
        }
    }

    fn expand_test_macro(
        &self,
        inner_precomp: PreCompiled<'ast>,
        outer_label: &'ast ast::Symbol,
        outer_test: &'ast ast::Test,
        arguments: &'ast [ast::MacroArgument],
        working_macro: &mut WorkingMacro<'ast>,
    ) {
        let params_map = self.map_params_to_args(
            &inner_precomp.macro_data.parameters,
            arguments
        );

        for instr in inner_precomp.program.instructions() {
            working_macro.insert_instruction(self.expand_test_macro_instr(
                &params_map,
                instr,
                outer_label,
                outer_test,
                &inner_precomp,
            ));
        }
    }

    fn expand_test_macro_instr(
        &self,
        params_map: &HashMap<&'ast str, &'ast str>,
        instr: &Instruction,
        outer_label: &'ast ast::Symbol,
        outer_test: &'ast ast::Test,
        inner_precomp: &PreCompiled<'ast>,
    ) -> Instruction {
        let instr_kind = match &instr.kind {
            InstructionKind::Operation(operation) => {
                let expanded_operation = self.expand_test_macro_oper_instr(
                    params_map,
                    operation,
                    outer_label,
                    outer_test,
                    inner_precomp,
                );

                InstructionKind::Operation(expanded_operation)
            },

            InstructionKind::Test(test) => {
                let expanded_test = self.expand_test_macro_test_instr(
                    params_map,
                    test,
                    outer_label,
                    outer_test,
                    inner_precomp,
                );

                InstructionKind::Test(expanded_test)
            },
        };

        Instruction {
            label: self.expand_test_macro_label(inner_precomp, &instr.label, outer_label, outer_test),
            kind: instr_kind
        }
    }

    fn expand_test_macro_oper_instr(
        &self,
        params_map: &HashMap<&'ast str, &'ast str>,
        operation: &Operation,
        outer_label: &'ast ast::Symbol,
        outer_test: &'ast ast::Test,
        inner_precomp: &PreCompiled<'ast>,
    ) -> Operation {

        Operation {
            kind: self.expand_oper_kind(&operation.kind, params_map),
            next: self.expand_test_macro_label(inner_precomp, &operation.next, outer_label, outer_test)
        }
    }

    fn expand_test_macro_test_instr(
        &self,
        params_map: &HashMap<&'ast str, &'ast str>,
        test: &Test,
        outer_label: &'ast ast::Symbol,
        outer_test: &'ast ast::Test,
        inner_precomp: &PreCompiled<'ast>,
    ) -> Test {

        Test {
            kind: self.expand_test_kind(&test.kind, params_map),
            next_then: self.expand_test_macro_label(inner_precomp, &test.next_then, outer_label, outer_test),
            next_else: self.expand_test_macro_label(inner_precomp, &test.next_else, outer_label, outer_test)
        }
    }


    fn expand_test_macro_label(
        &self, 
        inner_precomp: &PreCompiled<'ast>, 
        inner_next_label: &str,
        outer_label: &'ast ast::Symbol, 
        outer_test: &'ast ast::Test
    ) -> String {
        if inner_precomp.program.is_label_valid(inner_next_label) {
            format!("{}.{}.{}", outer_label.content, inner_precomp.macro_data.name.content, inner_next_label)
        } else if self.is_label_true(inner_next_label) {
            outer_test.next_true_label.content.clone()
        } else if self.is_label_false(inner_next_label) {
            outer_test.next_false_label.content.clone()
        } else {
            panic!("where to go??? :'(")
        }
    }

    fn is_label_true(&self, label: &str) -> bool {
        label == "true"
    }

    fn is_label_false(&self, label: &str) -> bool {
        label == "false"
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
}

#[derive(Clone, Debug)]
struct ExpansionRequired<'ast> {
    working_macro: WorkingMacro<'ast>,
}

#[derive(Clone, Debug)]
struct PreCompiled<'ast> {
    macro_data: &'ast ast::Macro,
    program: Program, // Program do interpretador
}

impl<'ast> PreCompiled<'ast> {
    fn new(macro_data: &'ast ast::Macro) -> Self {
        PreCompiled { macro_data, program: Program::empty() }
    }
}

#[derive(Clone, Debug)]
struct WorkingMacro<'ast> {
    precompiled: PreCompiled<'ast>,
    instr_index: usize,
}

impl<'ast> WorkingMacro<'ast> {
    fn new(precompiled: PreCompiled<'ast>) -> Self {
        WorkingMacro { precompiled, instr_index: 0 }
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
// enquanto tiver macros no expanding_macros e a medida que precisa compila os
// macros que são dependencias quando nao tiver mais nada no expanding_macros,
// tenta tirar outra do target_macros quando não tiver mais nada no
// target_macro, faz a expansão da main que é o mesmo algoritmo porém:

// se uma macro nao tiver precompilada, é por que nao existe aquela macro (por
// que todas ja foram precompiladas)

// expansao:
// 1- traduzir instruções
// 2- prefixar os rótulos dos macros aninhados (internos)
// 3- mapear rótulos de saída (true, false, invalidos) para rótulos externos
