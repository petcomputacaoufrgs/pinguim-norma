#[cfg(test)]
mod test;

pub mod error;

mod label;
mod artifacts;
mod macro_call;

use crate::{
    compiler::{
        lexer::token::{BuiltInOperation, BuiltInTest},
        parser::ast,
    },
    interpreter::program::{
        Instruction, InstructionKind, Operation, OperationKind, Program, Test,
        TestKind,
    },
};
use artifacts::{ExpansionRequired, PreCompiled, WorkingCode, WorkingMacro};
use error::{
    IncompatibleMacroType, MismatchedArgType, MismatchedArgsNumber,
    RecursiveMacro, UndefinedMacro,
};
use indexmap::{IndexMap, IndexSet};
use macro_call::{
    MacroCallExpansor, OperMacroCallExpansor, TestMacroCallExpansor,
};
use pinguim_language::error::{Diagnostics, Error};
use std::collections::HashMap;

/// Cria um Expansor e expande o programa a partir da `ast` fornecida
///
/// - `ast`: árvore sintática abstrata, programa oriundo do parser
/// - `diagnostics`: vetor que armazena erros coletados durante a compilação
pub fn expand(
    ast: &ast::Program,
    diagnostics: &mut Diagnostics,
) -> Option<Program> {
    Expansor::new(ast).expand_program(diagnostics)
}

struct Expansor<'ast> {
    ///
    /// - `precompileds`: macros já expandidas e prontas
    precompileds: HashMap<String, PreCompiled<'ast>>,
    ///
    /// - `target_macros`: macros a serem expandidas
    target_macros: IndexSet<String>,
    ///
    /// - `working_macros`: macros em progresso de expansão e que podem ser
    ///   pausadas se necessário
    working_macros: Vec<WorkingMacro<'ast>>,
    ///
    /// - `ast`: árvore sintática abstrata, programa oriundo do parser
    ast: &'ast ast::Program,
}

impl<'ast> Expansor<'ast> {
    /// Construtor da estrutura Expansor
    ///
    /// - `ast`: árvore sintática abstrata, programa oriundo do parser
    fn new(ast: &'ast ast::Program) -> Self {
        let target_macros = ast.macros.keys().rev().cloned().collect();

        Expansor {
            precompileds: HashMap::new(),
            target_macros,
            working_macros: Vec::new(),
            ast,
        }
    }

    /// Expande o programa inteiro, macros e main e retorna um possível programa
    /// na AST do Interpreter
    ///
    /// - `diagnostics`: vetor que armazena erros coletados durante a compilação
    fn expand_program(
        &mut self,
        diagnostics: &mut Diagnostics,
    ) -> Option<Program> {
        self.precompile_macros(diagnostics);
        self.expand_main(diagnostics)
    }

    /// Expande tudo que estiver na main do programa do parser, caso todas as
    /// macros estiverem expandidas. Caso contrário, retorna None
    ///
    /// - `diagnostics`: vetor que armazena erros coletados durante a compilação
    ///
    /// # Panics
    /// Invoca panic caso exista macro que não tenha sido precompilada
    fn expand_main(
        &mut self,
        diagnostics: &mut Diagnostics,
    ) -> Option<Program> {
        let mut code = WorkingCode::new();
        for instruction in self.ast.main.code.values() {
            let result = self.precompile_instruction(
                "main",
                instruction,
                &mut code,
                &self.ast.main.code,
                diagnostics,
                label::validate_for_main,
            );
            result.expect(
                "All existing macros should already have been precompiled",
            );
        }

        Some(code.finish())
    }

    /// Precompila todas as macros declaradas no programa
    ///
    /// - `diagnostics`: vetor que armazena erros coletados durante a compilação
    fn precompile_macros(&mut self, diagnostics: &mut Diagnostics) {
        while let Some(macro_name) = self.pop_target_macro() {
            let working_macro = self.make_working_macro(&macro_name);
            self.push_working_macro(working_macro);
            self.precompile_working_macros(diagnostics);
        }
    }

    /// Cria e retorna uma working macro para uma dada macro da ast
    ///
    /// - `macro_name`: Nome da macro cujo os dados devem ser pegos da ast e
    ///   utilizados para criar uma working macro
    fn make_working_macro(&mut self, macro_name: &str) -> WorkingMacro<'ast> {
        let macro_def = self.get_macro(macro_name);
        WorkingMacro::new(macro_def)
    }

    /// Retira uma macro da pilha de macros intocadas a serem precompiladas
    fn pop_target_macro(&mut self) -> Option<String> {
        self.target_macros.pop()
    }

    /// Pega uma dada macro da ast a partir de seu nome
    ///
    /// - `macro_name`: Nome da macro cujo os dados devem ser pegos da ast
    ///
    /// # Panics
    /// Invoca panic se for requisitado macro que não existe na ast
    fn get_macro(&mut self, macro_name: &str) -> &'ast ast::Macro {
        self.ast
            .macros
            .get(macro_name)
            .expect("Macro em target macros deveria existir na ast")
    }

    /// Coloca uma macro em precompilação na pilha de macros a serem trabalhadas
    ///
    /// - `working_macro`: Macro a ser colocada na pilha
    fn push_working_macro(&mut self, working_macro: WorkingMacro<'ast>) {
        self.working_macros.push(working_macro);
    }

    /// Expande o macro a ser trabalhado até que ele termine ou que precise ser
    /// pausado
    ///
    /// - `working_macro`: Macro em processo de precompilação
    /// - `diagnostics`: vetor que armazena erros coletados durante a compilação
    fn precompile_working_macro(
        &mut self,
        mut working_macro: WorkingMacro<'ast>,
        diagnostics: &mut Diagnostics,
    ) {
        loop {
            if let Some(instr) = working_macro.curr_instr() {
                let macro_data = working_macro.macro_data();
                let validate_label = match macro_data.macro_type {
                    ast::MacroType::Operation => label::validate_for_oper_macro,
                    ast::MacroType::Test => label::validate_for_test_macro,
                };

                let precomp_result = self.precompile_instruction(
                    &macro_data.name.content,
                    instr,
                    working_macro.code_mut(),
                    &macro_data.instr,
                    diagnostics,
                    validate_label,
                );

                match precomp_result {
                    Ok(()) => working_macro.next_instr(),
                    Err(request) => {
                        self.push_working_macro(working_macro);
                        self.push_working_macro(request.working_macro);
                        break;
                    }
                }
            } else {
                self.finish_working_macro(working_macro);
                break;
            }
        }
    }

    /// Encerra a precompilação de uma working macro e a insere como precompiled
    /// macro
    ///
    /// - `working_macro`: macro que estava em precompilação e foi será
    /// finalizada e colocada na estrutura de macros terminadas (precompileds macros)
    fn finish_working_macro(&mut self, working_macro: WorkingMacro<'ast>) {
        let precompiled = working_macro.finish();
        let name = precompiled.macro_data().name.content.clone();
        self.precompileds.insert(name, precompiled);
    }

    /// Expande todas as macros a serem trabalhadas enquanto houver macro nessa
    /// pilha
    ///
    /// - `diagnostics`: vetor que armazena erros coletados durante a compilação
    fn precompile_working_macros(&mut self, diagnostics: &mut Diagnostics) {
        while let Some(working_macro) = self.pop_working_macro() {
            self.precompile_working_macro(working_macro, diagnostics);
        }
    }

    /// Retira uma macro da pilha de macros a serem precompiladas
    fn pop_working_macro(&mut self) -> Option<WorkingMacro<'ast>> {
        self.working_macros.pop()
    }

    /// Coleta todas as macros chamadas atualmente por uma macro cujo nome
    /// é passado por parâmetro, incluindo ela mesma.
    /// - `macro_name`: nome da macro de onde a pilha de chamadas começa
    fn stack_of(&self, macro_name: &str) -> Option<Vec<String>> {
        let index = self.working_macros.iter().rposition(|working_macro| {
            working_macro.macro_data().name.content == macro_name
        })?;

        let macro_names = self.working_macros[index..]
            .iter()
            .map(|working_macro| {
                working_macro.macro_data().name.content.clone()
            })
            .collect();

        Some(macro_names)
    }

    /// Expande uma instrução conforme se ela for teste ou operação
    ///
    /// - `caller_name`: nome da macro (ou main) sendo atualmente processada, que causa a chamada desse método
    /// - `instr`: instrução a ser expandida
    /// - `working_code`: macro em precompilação
    /// - `ast_code`: programa da macro, estrutura com todas as instruções
    /// - `diagnostics`: vetor que armazena erros coletados durante a compilação
    /// - `validate_label`: função genérica que valida próximo label da instrução
    fn precompile_instruction<F>(
        &mut self,
        caller_name: &str,
        instr: &'ast ast::Instruction,
        working_code: &mut WorkingCode,
        ast_code: &'ast IndexMap<String, ast::Instruction>,
        diagnostics: &mut Diagnostics,
        validate_label: F,
    ) -> Result<(), ExpansionRequired<'ast>>
    where
        F: FnMut(
            &'ast ast::Symbol,
            &'ast IndexMap<String, ast::Instruction>,
            &mut Diagnostics,
        ),
    {
        match &instr.instruction_type {
            ast::InstructionType::Operation(operation) => {
                self.precompile_operation(
                    caller_name,
                    &instr.label,
                    operation,
                    working_code,
                    ast_code,
                    diagnostics,
                    validate_label,
                )?;
            }

            ast::InstructionType::Test(test) => {
                self.precompile_test(
                    caller_name,
                    &instr.label,
                    test,
                    working_code,
                    ast_code,
                    diagnostics,
                    validate_label,
                )?;
            }
        }

        Ok(())
    }

    /// Expande uma instrução do tipo operação
    ///
    /// - `caller_name`: nome da macro (ou main) sendo atualmente processada, que causa a chamada desse método
    /// - `label`: rótulo da instrução
    /// - `operation`: operação que a instrução executa
    /// - `working_code`: macro a qual a instrução pertence
    /// - `ast_code`: programa da macro, estrutura com todas as instruções
    /// - `diagnostics`: vetor que armazena erros coletados durante a compilação
    /// - `validate_label`: função genérica que valida próximo label da instrução
    fn precompile_operation<F>(
        &mut self,
        caller_name: &str,
        label: &'ast ast::Symbol,
        operation: &'ast ast::Operation,
        working_code: &mut WorkingCode,
        ast_code: &'ast IndexMap<String, ast::Instruction>,
        diagnostics: &mut Diagnostics,
        mut validate_label: F,
    ) -> Result<(), ExpansionRequired<'ast>>
    where
        F: FnMut(
            &'ast ast::Symbol,
            &'ast IndexMap<String, ast::Instruction>,
            &mut Diagnostics,
        ),
    {
        validate_label(&operation.next_label, ast_code, diagnostics);

        match &operation.oper_type {
            ast::OperationType::BuiltIn(builtin_oper, argument) => {
                let oper_kind =
                    self.precompile_builtin_oper(*builtin_oper, argument);
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
            }
            ast::OperationType::Macro(macro_name, params) => self
                .precompile_macro_call(
                    caller_name,
                    label,
                    operation,
                    &OperMacroCallExpansor,
                    macro_name,
                    params,
                    working_code,
                    diagnostics,
                ),
        }
    }

    /// Expande uma operação builtin em uma dada instrução
    ///
    /// - `builtin_oper`: operação builtin a ser precompilada
    /// - `arg`: argumento da operação builtin
    fn precompile_builtin_oper(
        &mut self,
        builtin_oper: BuiltInOperation,
        arg: &'ast ast::Symbol,
    ) -> OperationKind {
        match builtin_oper {
            BuiltInOperation::Inc => OperationKind::Inc(arg.content.clone()),
            BuiltInOperation::Dec => OperationKind::Dec(arg.content.clone()),
        }
    }

    /// Expande uma instrução do tipo teste
    ///
    /// - `caller_name`: nome da macro (ou main) sendo atualmente processada, que causa a chamada desse método
    /// - `label`: rótulo da instrução
    /// - `test`: teste que a instrução executa
    /// - `working_code`: macro a qual a instrução pertence
    /// - `ast_code`: programa da macro, estrutura com todas as instruções
    /// - `diagnostics`: vetor que armazena erros coletados durante a compilação
    /// - `validate_label`: função genérica que valida próximo label da instrução
    fn precompile_test<F>(
        &mut self,
        caller_name: &str,
        label: &'ast ast::Symbol,
        test: &'ast ast::Test,
        working_code: &mut WorkingCode,
        ast_code: &'ast IndexMap<String, ast::Instruction>,
        diagnostics: &mut Diagnostics,
        mut validate_label: F,
    ) -> Result<(), ExpansionRequired<'ast>>
    where
        F: FnMut(
            &'ast ast::Symbol,
            &'ast IndexMap<String, ast::Instruction>,
            &mut Diagnostics,
        ),
    {
        validate_label(&test.next_true_label, ast_code, diagnostics);
        validate_label(&test.next_false_label, ast_code, diagnostics);

        match &test.test_type {
            ast::TestType::BuiltIn(builtin_test, argument) => {
                let test_kind =
                    self.precompile_builtin_test(*builtin_test, argument);
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
            }
            ast::TestType::Macro(macro_name, params) => self
                .precompile_macro_call(
                    caller_name,
                    label,
                    test,
                    &TestMacroCallExpansor,
                    macro_name,
                    params,
                    working_code,
                    diagnostics,
                ),
        }
    }

    /// Expande um teste builtin em uma dada instrução
    ///
    /// - `builtin_test`: teste builtin a ser precompilado
    /// - `arg`: argumento do teste builtin
    fn precompile_builtin_test(
        &mut self,
        builtin_test: BuiltInTest,
        arg: &'ast ast::Symbol,
    ) -> TestKind {
        match builtin_test {
            BuiltInTest::Zero => TestKind::Zero(arg.content.clone()),
        }
    }

    /// Precompila uma outra chamada de macro em uma dada instrução de uma macro
    /// que está sendo precompilada. Caso retorne erro, significa que outra
    /// macro precisa ser precompilada antes de continuar a macro atual.
    /// Recomenda-se empilhar a macro atual e em seguida empilhar a macro
    /// requisitada.
    ///
    /// - `caller_name`: nome da macro (ou main) sendo atualmente processada, que
    /// causa a chamada desse método e que chama a macro dessa instrução
    /// - `label`: rótulo da instrução
    /// - `instr_kind`: tipo da instrução que está chamando a macro
    /// - `call_expansor`: estrutura que lida com a expansão de uma chamada de
    /// macro dentro de outra
    /// - `macro_name`: nome da macro chamada mais internamente
    /// - `arguments`: argumentos da nova chamada de macro
    /// - `working_code`: macro que estava em precompilação
    /// - `diagnostics`: vetor que armazena erros coletados durante a compilação
    fn precompile_macro_call<E>(
        &mut self,
        caller_name: &str,
        label: &'ast ast::Symbol,
        instr_kind: &'ast E::InstructionKind,
        call_expansor: &E,
        macro_name: &'ast ast::Symbol,
        arguments: &'ast [ast::MacroArgument],
        working_code: &mut WorkingCode,
        diagnostics: &mut Diagnostics,
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
                    found_type: precompiled_macro.macro_data().macro_type,
                };

                diagnostics.raise(Error::new(error_cause, macro_name.span));
            }

            Ok(())
        } else if self.target_macros.remove(&macro_name.content) {
            let working_macro = self.make_working_macro(&macro_name.content);
            Err(ExpansionRequired { working_macro })
        } else if self.ast.macros.contains_key(&macro_name.content) {
            let mut macro_names =
                self.stack_of(&macro_name.content).unwrap_or_default();
            macro_names.push(String::from(caller_name));
            let error_cause = RecursiveMacro { macro_names };
            diagnostics.raise(Error::new(error_cause, macro_name.span));
            Ok(())
        } else {
            let error_cause =
                UndefinedMacro { macro_name: macro_name.content.clone() };
            diagnostics.raise(Error::new(error_cause, macro_name.span));
            Ok(())
        }
    }

    /// Expande uma chamada de macro em uma dada instrução que está sendo
    /// precompilada
    ///
    /// - `call_macro_name`: nome da macro que chama a macro interna
    /// - `inner_precomp`: macro que é chamada internamente
    /// - `outer_label`: label da instrução da macro externa que chamou a macro
    ///   interna
    /// - `outer_instr_kind`: tipo da instrução da macro externa
    /// - `call_expansor`: estrutura que lida com a expansão de uma chamada de
    ///   macro dentro de outra
    /// - `arguments`: argumentos da chamada de macro
    /// - `working_code`: macro que estava em precompilação
    /// - `diagnostics`: vetor que armazena erros coletados durante a compilação
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
            diagnostics,
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

    /// Expande uma instrução de um macro pré-compilada para dentro de onde ela
    /// é chamada
    ///
    /// - `params_map`: mapeia parâmetros formais do macro pré-compilado para os
    ///   argumentos passados. Os Argumentos se encontram na chamada de fora.
    /// - `instr`: é a instrução a ser compilada.
    /// - `outer_label`: é o label da instrução que chama a macro de dentro. Será
    ///   prefixado a todos labels da macro de dentro.
    /// - `outer_instr_kind`: é o label após o `goto` na instrução da macro de
    ///   fora. Rótulos de saída da macro de dentro serão remapeados para esse
    ///   label.
    /// - `call_expansor`: estrutura que lida com a expansão de uma chamada de
    ///   macro dentro de outra
    /// - `inner_precomp`: é a precompilação da macro interna.
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
            }

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
            }
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

    /// Expande uma instrução do tipo operação
    ///
    /// - `params_map`: mapeia parâmetros formais do macro pré-compilado para os
    ///   argumentos passados. Os Argumentos se encontram na chamada de fora.
    /// - `operation`: é a operação da instrução a ser compilada.
    /// - `outer_label`: é o label da instrução que chama a macro de dentro. Será
    ///   prefixado a todos labels da macro de dentro.
    /// - `outer_instr_kind`: é o label após o `goto` na instrução da macro de
    ///   fora. Rótulos de saída da macro de dentro serão remapeados para esse
    ///   label.
    /// - `call_expansor`: estrutura que lida com a expansão de uma chamada de
    /// macro dentro de outra
    /// - `inner_precomp`: é a precompilação da macro interna.
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

    /// Expande uma instrução do tipo teste
    ///
    /// - `params_map`: mapeia parâmetros formais do macro pré-compilado para os
    ///   argumentos passados. Os Argumentos se encontram na chamada de fora.
    /// - `teste`: é o teste da instrução a ser compilada.
    /// - `outer_label`: é o label da instrução que chama a macro de dentro. Será
    ///   prefixado a todos labels da macro de dentro.
    /// - `outer_instr_kind`: é o label após o `goto` na instrução da macro de
    ///   fora. Rótulos de saída da macro de dentro serão remapeados para esse
    ///   label.
    /// - `call_expansor`: estrutura que lida com a expansão de uma chamada de
    /// macro dentro de outra
    /// - `inner_precomp`: é a precompilação da macro interna.
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

    /// Expande um dado rótulo
    ///
    /// - `inner_precomp`: é a precompilação da macro interna.
    /// - `inner_next_label`: o próximo label da instrução atual da macro chamada
    /// - `outer_label`: é o label da instrução que chama a macro de dentro. Será
    ///   prefixado a todos labels da macro de dentro
    /// - `outer_instr_kind`: é o label após o `goto` na instrução da macro de
    ///   fora. Rótulos de saída da macro de dentro serão remapeados para esse
    ///   label
    /// - `call_expansor`: estrutura que lida com a expansão de uma chamada de
    /// macro dentro de outra
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
        } else if label::is_true(inner_next_label) {
            call_expansor.expand_true_label(outer_instr_kind)
        } else if label::is_false(inner_next_label) {
            call_expansor.expand_false_label(outer_instr_kind)
        } else {
            call_expansor.expand_invalid_label(outer_instr_kind)
        }
    }

    /// Renomeia registradores que são parâmetros na definição da macro chamada,
    /// trocando-os pelos argumentos da chamada. Referente a isntrução do
    /// tipo operação
    ///
    /// - `operation_kind`: o tipo de operação executado pela instrução
    /// - `params_map`: mapeia parâmetros formais do macro pré-compilado para os
    ///   argumentos passados. Os Argumentos se encontram na chamada de fora.
    fn expand_oper_kind(
        &self,
        operation_kind: &OperationKind,
        params_map: &HashMap<&'ast str, &'ast str>,
    ) -> OperationKind {
        operation_kind.map_registers(|register| {
            self.map_param_to_arg(params_map, register)
        })
    }

    /// Renomeia registradores que são parâmetros na definição da macro chamada,
    /// trocando-os pelos argumentos da chamada. Referente a isntrução do
    /// tipo teste
    ///
    /// - `test_kind`: o tipo de teste executado pela instrução
    /// - `params_map`: mapeia parâmetros formais do macro pré-compilado para os
    ///   argumentos passados. Os Argumentos se encontram na chamada de fora.
    fn expand_test_kind(
        &self,
        test_kind: &TestKind,
        params_map: &HashMap<&'ast str, &'ast str>,
    ) -> TestKind {
        test_kind.map_registers(|register| {
            self.map_param_to_arg(params_map, register)
        })
    }

    /// Renomeia um registrador. Caso o registrador seja um parâmetro, será
    /// substituido pelo argumento correspondente na chamada. Caso não seja
    /// um parâmetro, o registrador é inalterado.
    ///
    /// - `params_map`: mapeia parâmetros formais do macro pré-compilado para os
    ///   argumentos passados. Os Argumentos se encontram na chamada de fora.
    /// - `register`: registrador dos parâmetros formais a ser mapeado
    fn map_param_to_arg(
        &self,
        params_map: &HashMap<&'ast str, &'ast str>,
        register: &str,
    ) -> String {
        params_map
            .get(register)
            .map_or_else(|| register.to_string(), |arg| arg.to_string())
    }

    /// Produz e retorna um mapeamento dos nomes de registradores dos parâmetros
    /// formais (chave) para os nomes de registradores passados como
    /// argumentos (valor).
    ///
    /// - `call_macro_name`: nome da macro que foi chamada internamente
    /// - `def_params`: vetor com todos os parâmetros formais de
    ///   `call_macro_name`
    /// - `args`: vetor com todos os argumentos de `call_macro_name`
    /// - `diagnostics`: vetor que armazena erros coletados durante a compilação
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
                found_num: args.len(),
            };
            diagnostics.raise(Error::new(error_cause, call_macro_name.span));
        }

        def_params
            .iter()
            .zip(args)
            .enumerate()
            .map(|(index, (param, arg))| {
                (
                    param.content.as_str(),
                    self.expect_register_arg(
                        call_macro_name,
                        arg,
                        index,
                        diagnostics,
                    ),
                )
            })
            .collect()
    }

    /// Se o argumento passado para a macro é Register, retorna o conteúdo do
    /// registrador, caso contrário retorna erro MismatchedArgType
    ///
    /// - `call_macro_name`: nome da macro que foi chamada internamente
    /// - `macro_argument`: argumento passado para a macro chamada
    /// - `index`: posição do `macro_argument` dentre os argumentos de
    ///   `call_macro_name`
    /// - `diagnostics`: vetor que armazena erros coletados durante a compilação
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
                    index,
                };
                diagnostics
                    .raise(Error::new(error_cause, call_macro_name.span));
                "?"
            }
        }
    }
}
