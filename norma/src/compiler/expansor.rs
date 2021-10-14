#[cfg(test)]
mod test;

pub mod error;

mod artifacts;
mod macro_call;

use crate::{
    compiler::{
        error::{Diagnostics, Error},
        lexer::token::{BuiltInOperation, BuiltInTest},
        parser::ast,
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
use error::{UndefinedMacro, RecursiveMacro, IncompatibleMacroType, MismatchedArgsNumber, MismatchedArgType};
use artifacts::{ExpansionRequired, PreCompiled, WorkingCode, WorkingMacro};
use indexmap::IndexSet;
use macro_call::{
    MacroCallExpansor,
    OperMacroCallExpansor,
    TestMacroCallExpansor,
};
use std::collections::HashMap;

pub fn expand(
    ast: &ast::Program,
    diagnostics: &mut Diagnostics,
) -> Option<Program> {
    Expansor::new(ast).expand_program(diagnostics)
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
    fn expand_program(&mut self, diagnostics: &mut Diagnostics) -> Option<Program> {
        self.precompile_macros(diagnostics);
        self.expand_main(diagnostics)
    }

    // compila a main depois de precompilar os macros
    fn expand_main(&mut self, diagnostics: &mut Diagnostics) -> Option<Program> {
        let mut code = WorkingCode::new();
        for instruction in self.ast.main.code.values() {
            self.precompile_instruction(instruction, &mut code, diagnostics).expect(
                "All existing macros should already have been precompiled",
            );
        }
        Some(code.finish())
    }

    // precompila todas as macros
    fn precompile_macros(&mut self, diagnostics: &mut Diagnostics) {
        while let Some(macro_name) = self.pop_target_macro() {
            let working_macro = self.make_working_macro(&macro_name);
            self.push_working_macro(working_macro);
            self.precompile_working_macros(diagnostics);
        }
    }

    // pega uma nova macro target e coloca na pilha de macros a serem
    // trabalhadas
    fn make_working_macro(&mut self, macro_name: &str) -> WorkingMacro<'ast> {
        let macro_def = self.get_macro(macro_name);
        WorkingMacro::new(macro_def)
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
        diagnostics: &mut Diagnostics
    ) {
        loop {
            if let Some(instr) = working_macro.curr_instr() {
                let precomp_result = self
                    .precompile_instruction(instr, working_macro.code_mut(), diagnostics);

                match precomp_result {
                    Ok(()) => working_macro.next_instr(),
                    Err(request) => {
                        self.push_working_macro(working_macro);
                        self.push_working_macro(request.working_macro);
                        break;
                    },
                }
            } else {
                self.finish_working_macro(working_macro);
                break;
            }
        }
    }

    /// Acaba a pré-compilação de um macro, inserindo o trabalho feito por
    /// `WorkingMacro` no registro de macros pré-compilados.
    fn finish_working_macro(&mut self, working_macro: WorkingMacro<'ast>) {
        let precompiled = working_macro.finish();
        let name = precompiled.macro_data().name.content.clone();
        self.precompileds.insert(name, precompiled);
    }

    // expande todas as macros na pilha de macros a serem trabalhadas
    fn precompile_working_macros(&mut self, diagnostics: &mut Diagnostics) {
        while let Some(working_macro) = self.pop_working_macro() {
            self.precompile_working_macro(working_macro, diagnostics);
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
        working_code: &mut WorkingCode,
        diagnostics: &mut Diagnostics
    ) -> Result<(), ExpansionRequired<'ast>> {
        match &instr.instruction_type {
            ast::InstructionType::Operation(operation) => {
                self.precompile_operation(
                    &instr.label,
                    operation,
                    working_code,
                    diagnostics
                )?;
            },
            ast::InstructionType::Test(test) => {
                self.precompile_test(
                    &instr.label, 
                    test, 
                    working_code, 
                    diagnostics
                )?;
            },
        }

        Ok(())
    }

    // expande uma instrução do tipo operação
    fn precompile_operation(
        &mut self,
        label: &'ast ast::Symbol,
        operation: &'ast ast::Operation,
        working_code: &mut WorkingCode,
        diagnostics: &mut Diagnostics
    ) -> Result<(), ExpansionRequired<'ast>> {
        match &operation.oper_type {
            ast::OperationType::BuiltIn(builtin_oper, param) => {
                let oper_kind =
                    self.precompile_builtin_oper(*builtin_oper, param);
                let runtime_oper = Operation {
                    kind: oper_kind,
                    next: operation.next_label.content.clone(),
                };

                let instruction = Instruction::new(
                    label.content.clone(),
                    InstructionKind::Operation(runtime_oper),
                );

                working_code.insert_instr(instruction);

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
                    diagnostics
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
        working_code: &mut WorkingCode,
        diagnostics: &mut Diagnostics
    ) -> Result<(), ExpansionRequired<'ast>> {
        match &test.test_type {
            ast::TestType::BuiltIn(builtin_test, param) => {
                let test_kind =
                    self.precompile_builtin_test(*builtin_test, param);
                let runtime_test = Test {
                    kind: test_kind,
                    next_then: test.next_true_label.content.clone(),
                    next_else: test.next_false_label.content.clone(),
                };

                let instruction = Instruction::new(
                    label.content.clone(),
                    InstructionKind::Test(runtime_test),
                );

                working_code.insert_instr(instruction);

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
                    diagnostics
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
        working_code: &mut WorkingCode,
        diagnostics: &mut Diagnostics
    ) -> Result<(), ExpansionRequired<'ast>>
    where
        E: MacroCallExpansor<'ast>,
    {
        if let Some(precompiled_macro) =
            self.precompileds.get(&macro_name.content).cloned()
        {
            if call_expansor.macro_type()
                == precompiled_macro.macro_data().macro_type
            {
                self.expand_macro(
                    macro_name,
                    precompiled_macro,
                    label,
                    instr_kind,
                    call_expansor,
                    arguments,
                    working_code,
                    diagnostics,
                );
            } else {
                let error_cause = IncompatibleMacroType {
                    macro_name: macro_name.content.clone(),
                    expected_type: call_expansor.macro_type(),
                    found_type: precompiled_macro.macro_data().macro_type
                };

                diagnostics.raise(Error::new(error_cause, macro_name.span));
            }

            Ok(())
        } else if self.target_macros.remove(&macro_name.content) {
            let working_macro = self.make_working_macro(&macro_name.content);
            Err(ExpansionRequired { working_macro })
        } else if self.ast.macros.contains_key(&macro_name.content) {
            let error_cause = RecursiveMacro { macro_name: macro_name.content.clone() };
            diagnostics.raise(Error::new(error_cause, macro_name.span));
            Ok(())
        } else {
            let error_cause = UndefinedMacro { macro_name: macro_name.content.clone() };
            diagnostics.raise(Error::new(error_cause, macro_name.span));
            Ok(())
        }
    }

    fn expand_macro<E>(
        &mut self,
        call_macro_name: &'ast ast::Symbol,
        inner_precomp: PreCompiled<'ast>,
        outer_label: &'ast ast::Symbol,
        outer_instr_kind: &'ast E::InstructionKind,
        call_expansor: &E,
        arguments: &'ast [ast::MacroArgument],
        working_code: &mut WorkingCode,
        diagnostics: &mut Diagnostics,
    ) where
        E: MacroCallExpansor<'ast>,
    {
        let first_label = inner_precomp.program().first_label();
        let expanded_first_label = self.expand_label(
            &inner_precomp,
            first_label,
            outer_label,
            outer_instr_kind,
            call_expansor,
        );
        working_code.insert_expansion(
            outer_label.content.clone(),
            expanded_first_label,
        );

        let params_map = self.map_params_to_args(
            call_macro_name,
            &inner_precomp.macro_data().parameters,
            arguments,
            diagnostics
        );

        for instr in inner_precomp.program().instructions() {
            working_code.insert_instr(self.expand_instr(
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
        inner_precomp: &PreCompiled<'ast>,
    ) -> Instruction
    where
        E: MacroCallExpansor<'ast>,
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

        Instruction::new(
            self.expand_label(
                inner_precomp,
                instr.label(),
                outer_label,
                outer_instr_kind,
                call_expansor,
            ),
            instr_kind,
        )
    }

    fn expand_oper_instr<E>(
        &mut self,
        params_map: &HashMap<&'ast str, &'ast str>,
        operation: &Operation,
        outer_label: &'ast ast::Symbol,
        outer_instr_kind: &'ast E::InstructionKind,
        call_expansor: &E,
        inner_precomp: &PreCompiled<'ast>,
    ) -> Operation
    where
        E: MacroCallExpansor<'ast>,
    {
        Operation {
            kind: self.expand_oper_kind(&operation.kind, params_map),
            next: self.expand_label(
                inner_precomp,
                &operation.next,
                outer_label,
                outer_instr_kind,
                call_expansor,
            ),
        }
    }

    fn expand_test_instr<E>(
        &mut self,
        params_map: &HashMap<&'ast str, &'ast str>,
        test: &Test,
        outer_label: &'ast ast::Symbol,
        outer_instr_kind: &'ast E::InstructionKind,
        call_expansor: &E,
        inner_precomp: &PreCompiled<'ast>,
    ) -> Test
    where
        E: MacroCallExpansor<'ast>,
    {
        Test {
            kind: self.expand_test_kind(&test.kind, params_map),
            next_then: self.expand_label(
                inner_precomp,
                &test.next_then,
                outer_label,
                outer_instr_kind,
                call_expansor,
            ),
            next_else: self.expand_label(
                inner_precomp,
                &test.next_else,
                outer_label,
                outer_instr_kind,
                call_expansor,
            ),
        }
    }

    fn expand_label<E>(
        &self,
        inner_precomp: &PreCompiled<'ast>,
        inner_next_label: &str,
        outer_label: &'ast ast::Symbol,
        outer_instr_kind: &'ast E::InstructionKind,
        call_expansor: &E,
    ) -> String
    where
        E: MacroCallExpansor<'ast>,
    {
        if inner_precomp.program().is_label_valid(inner_next_label) {
            format!(
                "{}.{}.{}",
                outer_label.content,
                inner_precomp.macro_data().name.content,
                inner_next_label
            )
        } else if self.is_true_label(inner_next_label) {
            call_expansor.expand_true_label(outer_instr_kind)
        } else if self.is_false_label(inner_next_label) {
            call_expansor.expand_false_label(outer_instr_kind)
        } else {
            call_expansor.expand_invalid_label(outer_instr_kind)
        }
    }

    fn expand_oper_kind(
        &self,
        operation_kind: &OperationKind,
        params_map: &HashMap<&'ast str, &'ast str>,
    ) -> OperationKind {
        operation_kind.map_registers(|register| {
            self.map_param_to_arg(params_map, register)
        })
    }

    fn expand_test_kind(
        &self,
        test_kind: &TestKind,
        params_map: &HashMap<&'ast str, &'ast str>,
    ) -> TestKind {
        test_kind.map_registers(|register| {
            self.map_param_to_arg(params_map, register)
        })
    }

    fn map_param_to_arg(
        &self,
        params_map: &HashMap<&'ast str, &'ast str>,
        register: &str,
    ) -> String {
        params_map
            .get(register)
            .map_or_else(|| register.to_string(), |arg| arg.to_string())
    }

    /// Produz mapeamento de nomes de registradores em parâmetros formais
    /// (chave) para registradores passados como argumentos (valor).
    fn map_params_to_args(
        &self,
        call_macro_name: &'ast ast::Symbol,
        def_params: &'ast [ast::Symbol],
        args: &'ast [ast::MacroArgument],
        diagnostics: &mut Diagnostics,
    ) -> HashMap<&'ast str, &'ast str> {
        if def_params.len() != args.len() {
            let error_cause = MismatchedArgsNumber { 
                macro_name: call_macro_name.content.clone(),
                expected_num: def_params.len(),
                found_num: args.len()
            };
            diagnostics.raise(Error::new(error_cause, call_macro_name.span));
        }

        def_params
            .iter()
            .zip(args)
            .enumerate()
            .map(|(index, (param, arg))| {
                (param.content.as_str(), self.expect_register_arg(call_macro_name, arg, index, diagnostics))
            })
            .collect()
    }

    fn expect_register_arg(
        &self,
        call_macro_name: &'ast ast::Symbol,
        macro_argument: &'ast ast::MacroArgument,
        index: usize,
        diagnostics: &mut Diagnostics,
    ) -> &'ast str {
        match macro_argument {
            ast::MacroArgument::Register(register) => &register.content,
            _ => {
                let found_type = macro_argument.arg_type();
                let expected_type = ast::MacroArgumentType::Register;
                let error_cause = MismatchedArgType {
                    macro_name: call_macro_name.content.clone(),
                    expected_type,
                    found_type,
                    index
                };
                diagnostics.raise(Error::new(error_cause, call_macro_name.span));
                "?"
            },
        }
    }

    fn is_true_label(&self, label: &str) -> bool {
        label == "true"
    }

    fn is_false_label(&self, label: &str) -> bool {
        label == "false"
    }
}
