#[cfg(test)]
mod test;

pub mod error;
pub mod ast;

use crate::compiler::{
    error::{Diagnostics, Error},
    lexer::token::{BuiltInOperation, BuiltInTest, Token, TokenType},
};
use ast::{
    Instruction,
    InstructionType,
    Macro,
    MacroArgument,
    MacroType,
    Main,
    Operation,
    OperationType,
    Program,
    Symbol,
    Test,
    TestType,
};
use error::{
    LabelAlreadyDeclared,
    MacroAlreadyDeclared,
    MainAlreadyDeclared,
    MainNotDeclared,
    UnexpectedEndOfInput,
    UnexpectedToken,
    InvalidLabel,
};
use indexmap::IndexMap;
use num_bigint::BigUint;
use std::str::FromStr;

/// Cria uma estrutura Parser e parsa a lista de tokens para um programa
/// 
/// - `tokens`: vetor de tokens
/// - `diagnostics`: vetor que armazena erros coletados durante a compilação 
pub fn parse(
    tokens: Vec<Token>,
    diagnostics: &mut Diagnostics,
) -> Option<Program> {
    Parser::new(tokens).parse_program(diagnostics).ok().flatten()
}

#[derive(Debug)]
/// Estrutura responsável por para o parser em situações críticas
struct Abort;

#[derive(Debug)]
struct Parser {
    ///
    /// - `tokens`: vetor de tokens a serem parsados
    tokens: Vec<Token>,
    ///
    /// - `curr_token`: índice do token que está sendo parsado
    curr_token: usize,
}

impl Parser {
    /// Cria uma nova estrutura de Parser
    /// 
    /// - `tokens`: vetor de tokens
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, curr_token: 0 }
    }

    /// Pega o token o qual está sendo parsado no momento dado seu índice
    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.curr_token)
    }

    /// Pega o token o qual está sendo parsado no momento e garante que ele exista
    /// 
    /// - `diagnostics`: vetor que armazena erros coletados durante a compilação
    fn require_current(
        &self,
        diagnostics: &mut Diagnostics,
    ) -> Result<&Token, Abort> {
        match self.current() {
            Some(token) => Ok(token),
            None => {
                diagnostics.raise(Error::with_no_span(UnexpectedEndOfInput));
                Err(Abort)
            },
        }
    }

    /// Incrementa o índice para o próximo token
    fn next(&mut self) {
        self.curr_token += 1;
    }

    /// Confere se o próximo token é do tipo esperado, adicionando erro no diagnóstico quando não for
    /// 
    /// - `expected_type`: tipo de token que é esperado encontrar
    /// - `diagnostics`: vetor que armazena erros coletados durante a compilação
    fn expect(
        &mut self,
        expected_type: TokenType,
        diagnostics: &mut Diagnostics,
    ) -> Result<(), Abort> {
        let token = self.require_current(diagnostics)?;

        if token.token_type == expected_type {
            self.next();
        } else {
            let expected_types = vec![expected_type];
            diagnostics.raise(Error::new(
                UnexpectedToken { expected_types },
                token.span,
            ));
        }

        Ok(())
    }

    /// Confere se o próximo token é do tipo esperado, retornando true se for, false se não for
    /// 
    /// - `expected_type`: tipo de token esperado
    /// - `diagnostics`: vetor que armazena erros coletados durante a compilação
    fn check_expect(
        &mut self,
        expected_type: TokenType,
        diagnostics: &mut Diagnostics,
    ) -> Result<bool, Abort> {
        let token = self.require_current(diagnostics)?;

        if token.token_type == expected_type {
            self.next();
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Faz o parse do vetor de tokens em um programa
    /// 
    /// - `diagnostics`: vetor que armazena erros coletados durante a compilação
    fn parse_program(
        &mut self,
        diagnostics: &mut Diagnostics,
    ) -> Result<Option<Program>, Abort> {
        let mut macros = IndexMap::<String, Macro>::new();
        let mut main_option: Option<Main> = None;
        let mut main_declared = false;

        while let Some(token) = self.current() {
            let token_span = token.span;

            match token.token_type {
                TokenType::Main => {
                    // se main não declarada ainda, fazer parse
                    if !main_declared {
                        main_option = self.parse_main(diagnostics)?;
                        main_declared = true;

                    // se main já declarada, jogar erro
                    } else {
                        diagnostics
                            .raise(Error::new(MainAlreadyDeclared, token_span));
                    }
                },

                TokenType::Operation => {
                    if let Some(macro_aux) =
                        self.parse_macro_def(MacroType::Operation, diagnostics)?
                    {
                        self.insert_macro_def(
                            &mut macros,
                            macro_aux,
                            diagnostics,
                        );
                    }
                },

                TokenType::Test => {
                    if let Some(macro_aux) =
                        self.parse_macro_def(MacroType::Test, diagnostics)?
                    {
                        self.insert_macro_def(
                            &mut macros,
                            macro_aux,
                            diagnostics,
                        );
                    }
                },

                _ => {
                    let expected_types = vec![
                        TokenType::Main,
                        TokenType::Operation,
                        TokenType::Test,
                    ];
                    diagnostics.raise(Error::new(
                        UnexpectedToken { expected_types },
                        token.span,
                    ));
                },
            }
        }

        // se depois de parsar todas as macros, não foi declarada nenhuma main
        if !main_declared {
            diagnostics.raise(Error::with_no_span(MainNotDeclared));
        }

        Ok(main_option.map(|main| Program { main, macros }))
    }

    /// Insere definição de macro na estrutura que armazena todas as definições de macros
    /// 
    /// - `macros`: estrutura <nome da macro, definição da macro>
    /// - `macro_def`: definição de uma macro
    /// - `diagnostics`: vetor que armazena erros coletados durante a compilação
    fn insert_macro_def(
        &mut self,
        macros: &mut IndexMap<String, Macro>,
        macro_def: Macro,
        diagnostics: &mut Diagnostics,
    ) {
        let macro_name = macro_def.name.content.clone();

        // se ainda não existe uma macro com tal nome, insere no indexmap
        if !macros.contains_key(macro_name.as_str()) {
            macros.insert(macro_name, macro_def);

        // se já existe, adicionar erro
        } else {
            diagnostics.raise(Error::new(
                MacroAlreadyDeclared { macro_name },
                macro_def.name.span,
            ));
        }
    }

    /// Faz o parse do vetor de tokens no programa da main
    /// 
    /// - `diagnostics`: vetor que armazena erros coletados durante a compilação
    fn parse_main(
        &mut self,
        diagnostics: &mut Diagnostics,
    ) -> Result<Option<Main>, Abort> {
        self.next();
        let instructions = self.parse_func_body(diagnostics)?;

        Ok(Some(Main { code: instructions }))
    }

    /// Faz o parse do código de qualquer função
    /// 
    /// - `diagnostics`: vetor que armazena erros coletados durante a compilação
    fn parse_func_body(
        &mut self,
        diagnostics: &mut Diagnostics,
    ) -> Result<IndexMap<String, Instruction>, Abort> {
        self.expect(TokenType::OpenCurly, diagnostics)?;
        let mut code = IndexMap::<String, Instruction>::new();

        loop {
            let token = self.require_current(diagnostics)?;
            let token_span = token.span;

            if token.token_type == TokenType::CloseCurly {
                self.next();
                break;
            } else if let Some(instr) = self.parse_instr(diagnostics)? {
                let label_name = instr.label.content.clone();

                // se ainda não existe uma instrução com tal label, insere no
                // indexmap
                if !code.contains_key(label_name.as_str()) {
                    code.insert(label_name, instr);

                // se já existe, adicionar erro
                } else {
                    diagnostics.raise(Error::new(
                        LabelAlreadyDeclared { label_name },
                        token_span,
                    ));
                }
            }
        }

        Ok(code)
    }

    /// Faz o parse da definição de uma macro
    /// 
    /// - `macro_type`: tipo da macro
    /// - `diagnostics`: vetor que armazena erros coletados durante a compilação
    fn parse_macro_def(
        &mut self,
        macro_type: MacroType,
        diagnostics: &mut Diagnostics,
    ) -> Result<Option<Macro>, Abort> {
        self.next();
        let name_option = self.parse_macro_name(diagnostics)?;
        let parameters = self.parse_macro_def_params(diagnostics)?;
        let instructions = self.parse_func_body(diagnostics)?;

        Ok(name_option.map(|name| Macro {
            macro_type,
            name,
            parameters,
            instr: instructions,
        }))
    }

    /// Faz o parse do nome de uma macro
    /// 
    /// - `diagnostics`: vetor que armazena erros coletados durante a compilação
    fn parse_macro_name(
        &mut self,
        diagnostics: &mut Diagnostics,
    ) -> Result<Option<Symbol>, Abort> {
        let token = self.require_current(diagnostics)?;

        if token.token_type == TokenType::Identifier {
            let macro_name =
                Symbol { content: token.content.clone(), span: token.span };

            self.next();
            Ok(Some(macro_name))
        } else {
            let expected_types = vec![TokenType::Identifier];
            diagnostics.raise(Error::new(
                UnexpectedToken { expected_types },
                token.span,
            ));
            Ok(None)
        }
    }

    /// Faz o parse dos parametros formais de uma macro
    /// 
    /// - `diagnostics`: vetor que armazena erros coletados durante a compilação
    fn parse_macro_def_params(
        &mut self,
        diagnostics: &mut Diagnostics,
    ) -> Result<Vec<Symbol>, Abort> {
        self.parse_param_list(Self::parse_register, diagnostics)
    }

    /// Faz o parse uma instrução do corpo (código) da macro
    /// 
    /// - `diagnostics`: vetor que armazena erros coletados durante a compilação
    fn parse_instr(
        &mut self,
        diagnostics: &mut Diagnostics,
    ) -> Result<Option<Instruction>, Abort> {
        let instr_label_option = self.parse_label(diagnostics)?;
        self.expect(TokenType::Colon, diagnostics)?;

        let type_option = self.parse_instr_type(diagnostics)?;

        let zipped = instr_label_option.zip(type_option);
        let instr = zipped.map(|(label, instruction_type)| {
            if label.content == "true" || label.content == "false" {
                diagnostics.raise(Error::new(InvalidLabel, label.span));
            }
            Instruction {
                label,
                instruction_type,
            }
        } );

        Ok(instr)
    }

    /// Faz o parse o tipo de uma instrução
    /// 
    /// - `diagnostics`: vetor que armazena erros coletados durante a compilação
    fn parse_instr_type(
        &mut self,
        diagnostics: &mut Diagnostics,
    ) -> Result<Option<InstructionType>, Abort> {
        let token = self.require_current(diagnostics)?;

        if token.token_type == TokenType::Do {
            let op_option = self.parse_instr_op(diagnostics)?;
            Ok(op_option.map(|operation| InstructionType::Operation(operation)))
        } else if token.token_type == TokenType::If {
            let test_option = self.parse_instr_test(diagnostics)?;
            Ok(test_option.map(|test| InstructionType::Test(test)))
        } else {
            let expected_types = vec![TokenType::Do, TokenType::If];
            diagnostics.raise(Error::new(
                UnexpectedToken { expected_types },
                token.span,
            ));
            Ok(None)
        }
    }

    /// Faz o parse uma instrução do tipo operação
    /// 
    /// - `diagnostics`: vetor que armazena erros coletados durante a compilação
    fn parse_instr_op(
        &mut self,
        diagnostics: &mut Diagnostics,
    ) -> Result<Option<Operation>, Abort> {
        self.next();
        let oper_type = self.parse_operation_type(diagnostics)?;

        self.expect(TokenType::Goto, diagnostics)?;
        let oper_label = self.parse_label(diagnostics)?;

        let zipped = oper_type.zip(oper_label);
        let operation = zipped.map(|(oper_type, oper_label)| Operation {
            oper_type,
            next_label: oper_label,
        });

        Ok(operation)
    }

    /// Faz o parse o tipo de operação de uma instrução do tipo operação
    /// 
    /// - `diagnostics`: vetor que armazena erros coletados durante a compilação
    fn parse_operation_type(
        &mut self,
        diagnostics: &mut Diagnostics,
    ) -> Result<Option<OperationType>, Abort> {
        let token = self.require_current(diagnostics)?;

        match token.token_type {
            TokenType::Identifier => {
                let macro_name =
                    Symbol { content: token.content.clone(), span: token.span };

                self.next();

                let arguments = self.parse_macro_args(diagnostics)?;
                Ok(Some(OperationType::Macro(macro_name, arguments)))
            },

            TokenType::BuiltInOper(oper) => {
                self.next();
                let argument_option = self.parse_builtin_arg(diagnostics)?;
                Ok(argument_option
                    .map(|argument| OperationType::BuiltIn(oper, argument)))
            },

            _ => {
                let expected_types = vec![
                    TokenType::BuiltInOper(BuiltInOperation::Inc),
                    TokenType::BuiltInOper(BuiltInOperation::Dec),
                    TokenType::Identifier,
                ];
                diagnostics.raise(Error::new(
                    UnexpectedToken { expected_types },
                    token.span,
                ));
                Ok(None)
            },
        }
    }

    /// Faz o parse uma instrução do tipo teste
    /// 
    /// - `diagnostics`: vetor que armazena erros coletados durante a compilação
    fn parse_instr_test(
        &mut self,
        diagnostics: &mut Diagnostics,
    ) -> Result<Option<Test>, Abort> {
        self.next();
        let test_type = self.parse_test_type(diagnostics)?;

        self.expect(TokenType::Then, diagnostics)?;
        self.expect(TokenType::Goto, diagnostics)?;
        let then_label = self.parse_label(diagnostics)?;

        self.expect(TokenType::Else, diagnostics)?;
        self.expect(TokenType::Goto, diagnostics)?;
        let else_label = self.parse_label(diagnostics)?;

        let zipped = test_type.zip(then_label).zip(else_label);
        let test = zipped.map(|((test_type, then_label), else_label)| Test {
            test_type,
            next_true_label: then_label,
            next_false_label: else_label,
        });

        Ok(test)
    }

    /// Faz o parse do tipo de teste de uma instrução do tipo teste
    /// 
    /// - `diagnostics`: vetor que armazena erros coletados durante a compilação
    fn parse_test_type(
        &mut self,
        diagnostics: &mut Diagnostics,
    ) -> Result<Option<TestType>, Abort> {
        let token = self.require_current(diagnostics)?;

        match token.token_type {
            TokenType::Identifier => {
                let macro_name =
                    Symbol { content: token.content.clone(), span: token.span };

                self.next();

                let argument = self.parse_macro_args(diagnostics)?;
                Ok(Some(TestType::Macro(macro_name, argument)))
            },

            TokenType::BuiltInTest(test) => {
                self.next();
                let argument_option = self.parse_builtin_arg(diagnostics)?;
                Ok(argument_option
                    .map(|argument| TestType::BuiltIn(test, argument)))
            },

            _ => {
                let expected_types = vec![
                    TokenType::BuiltInTest(BuiltInTest::Zero),
                    TokenType::Identifier,
                ];
                diagnostics.raise(Error::new(
                    UnexpectedToken { expected_types },
                    token.span,
                ));
                Ok(None)
            },
        }
    }

    /// Faz o parse de argumentos de testes ou operações builtin
    /// 
    ///  - `diagnostics`: vetor que armazena erros coletados durante a compilação
    fn parse_builtin_arg(
        &mut self,
        diagnostics: &mut Diagnostics,
    ) -> Result<Option<Symbol>, Abort> {
        let has_parens =
            self.check_expect(TokenType::OpenParen, diagnostics)?;
        let parameter = self.parse_register(diagnostics)?;
        if has_parens {
            self.expect(TokenType::CloseParen, diagnostics)?;
        }
        Ok(parameter)
    }

    /// Faz o parse de qualquer lista de parâmetros/argumentos
    /// 
    /// - `parse_param`: função genérica que faz o parse de parâmetros
    /// - `diagnostics`: vetor que armazena erros coletados durante a compilação
    fn parse_param_list<F, T>(
        &mut self,
        mut parse_param: F,
        diagnostics: &mut Diagnostics,
    ) -> Result<Vec<T>, Abort>
    where
        F: FnMut(&mut Self, &mut Diagnostics) -> Result<Option<T>, Abort>,
    {
        self.expect(TokenType::OpenParen, diagnostics)?;

        let mut parameters = Vec::new();
        let mut needs_comma = false;

        while !self.check_expect(TokenType::CloseParen, diagnostics)? {
            if needs_comma {
                let expected_types =
                    vec![TokenType::Comma, TokenType::CloseParen];
                let span = self.require_current(diagnostics)?.span;
                diagnostics.raise(Error::new(
                    UnexpectedToken { expected_types },
                    span,
                ));
            }

            if let Some(parameter) = parse_param(self, diagnostics)? {
                parameters.push(parameter);
                needs_comma =
                    !self.check_expect(TokenType::Comma, diagnostics)?;
            }
        }

        Ok(parameters)
    }

    /// Faz o parse dos argumentos da macro
    /// 
    /// - `diagnostics`: vetor que armazena erros coletados durante a compilação
    fn parse_macro_args(
        &mut self,
        diagnostics: &mut Diagnostics,
    ) -> Result<Vec<MacroArgument>, Abort> {
        self.parse_param_list(Self::parse_macro_arg, diagnostics)
    }

    /// Faz o parse de um argumento por vez
    /// 
    /// - `diagnostics`: vetor que armazena erros coletados durante a compilação
    fn parse_macro_arg(
        &mut self,
        diagnostics: &mut Diagnostics,
    ) -> Result<Option<MacroArgument>, Abort> {
        let token = self.require_current(diagnostics)?;

        match token.token_type {
            TokenType::Identifier => {
                let symbol =
                    Symbol { content: token.content.clone(), span: token.span };
                self.next();
                Ok(Some(MacroArgument::Register(symbol)))
            },

            TokenType::Number => {
                let constant = BigUint::from_str(&token.content).expect(
                    "Lexer só deve permitir tokens Number só com dígitos",
                );
                self.next();
                Ok(Some(MacroArgument::Number(constant)))
            },

            _ => {
                let expected_types =
                    vec![TokenType::Identifier, TokenType::Number];
                diagnostics.raise(Error::new(
                    UnexpectedToken { expected_types },
                    token.span,
                ));
                Ok(None)
            },
        }
    }

    /// Faz o parse de registradores
    /// 
    /// - `diagnostics`: vetor que armazena erros coletados durante a compilação
    fn parse_register(
        &mut self,
        diagnostics: &mut Diagnostics,
    ) -> Result<Option<Symbol>, Abort> {
        let token = self.require_current(diagnostics)?;

        if token.token_type == TokenType::Identifier {
            let symbol =
                Symbol { content: token.content.clone(), span: token.span };
            self.next();
            Ok(Some(symbol))
        } else {
            let expected_types = vec![TokenType::Identifier];
            diagnostics.raise(Error::new(
                UnexpectedToken { expected_types },
                token.span,
            ));
            Ok(None)
        }
    }

    /// Faz o parse do rótulo de uma instrução
    /// 
    /// - `diagnostics`: vetor que armazena erros coletados durante a compilação
    fn parse_label(
        &mut self,
        diagnostics: &mut Diagnostics,
    ) -> Result<Option<Symbol>, Abort> {
        let token = self.require_current(diagnostics)?;

        match token.token_type {
            TokenType::Number | TokenType::Identifier => {
                let label =
                    Symbol { content: token.content.clone(), span: token.span };

                self.next();
                Ok(Some(label))
            },

            _ => {
                let expected_types =
                    vec![TokenType::Identifier, TokenType::Number];
                diagnostics.raise(Error::new(
                    UnexpectedToken { expected_types },
                    token.span,
                ));
                Ok(None)
            },
        }
    }
}
