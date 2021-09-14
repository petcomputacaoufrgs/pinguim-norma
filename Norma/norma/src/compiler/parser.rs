#[cfg(test)]
mod test;

use crate::compiler::{ast::*, token::*};
use indexmap::IndexMap;
use num_bigint::BigUint;
use std::str::FromStr;
use crate::compiler::error::*;
use std::error::Error as StdError;

pub fn parse(tokens: Vec<Token>, diagnostics: &mut Diagnostics) -> Result<Option<Program>, Abort> {
    Parser::new(tokens).parse_program(diagnostics)
}

pub struct Abort;

#[derive(Debug)]
struct Parser {
    tokens: Vec<Token>,
    curr_token: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, curr_token: 0 }
    }

    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.curr_token)
    }

    fn require_current(&self, diagnostics: &mut Diagnostics) -> Result<&Token, Abort> {
        match self.current() {
            Some(token) => Ok(token),
            None => {
                diagnostics.raise(Error::with_no_span(UnexpectedEndOfInput));
                Err(Abort)
            }
        }
    }

    fn next(&mut self) {
        self.curr_token += 1;
    }

    fn expect(&mut self, expected_type: TokenType, diagnostics: &mut Diagnostics) -> Result<(), Abort> {
        let token = self.require_current(diagnostics)?;

        if token.token_type == expected_type {
            self.next();
        } else {
            let expected_types = vec![expected_type];
            diagnostics.raise(Error::new(UnexpectedToken { expected_types }, token.span));
        }

        Ok(())
    }

    fn check_expect(&mut self, expected_type: TokenType, diagnostics: &mut Diagnostics) -> Result<bool, Abort> {
        let token = self.require_current(diagnostics)?;

        if token.token_type == expected_type {
            self.next();
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn parse_program(&mut self, diagnostics: &mut Diagnostics) -> Result<Option<Program>, Abort> {
        let mut macros = IndexMap::<String, Macro>::new();
        let mut main_option: Option<Main> = None;

        while let Some(token) = self.current() {
            match token.token_type {
                TokenType::Main => {
                    // conferir se existe mais de uma main
                    main_option = self.parse_main(diagnostics)?;
                },

                TokenType::Operation => {
                    if let Some(macro_aux) =
                        self.parse_macro_def(MacroType::Operation, diagnostics)?
                    {
                        // conferir se ja nao esta no indexmap
                        macros
                            .insert(macro_aux.name.content.clone(), macro_aux);
                    }
                },

                TokenType::Test => {
                    if let Some(macro_aux) =
                        self.parse_macro_def(MacroType::Test, diagnostics)?
                    {
                        // conferir se ja nao esta no indexmap
                        macros
                            .insert(macro_aux.name.content.clone(), macro_aux);
                    }
                },

                _ => {
                    let expected_types = vec![TokenType::Main, TokenType::Operation, TokenType::Test];
                    diagnostics.raise(Error::new(UnexpectedToken { expected_types }, token.span));
                }
            }
        }
        
        // ver se a main é vazia e colocar erro no diagnostics

        Ok(main_option.map(|main| Program { main, macros }))
    }

    fn parse_main(&mut self, diagnostics: &mut Diagnostics) -> Result<Option<Main>, Abort> {
        self.next();
        let instructions = self.parse_func_body(diagnostics)?;

        Ok(Some(Main { code: instructions }))
    }

    fn parse_func_body(&mut self, diagnostics: &mut Diagnostics) -> Result<IndexMap<String, Instruction>, Abort> {
        self.expect(TokenType::OpenCurly, diagnostics);
        let mut code = IndexMap::<String, Instruction>::new();

        loop {
            let token = self.require_current(diagnostics)?;
            if token.token_type == TokenType::CloseCurly {
                self.next();
                break;
            } else if let Some(instr) = self.parse_instr(diagnostics)? {

                // fazer a verificação de label duplicado
                code.insert(instr.label.content.clone(), instr);
            }
        }

        Ok(code)
    }

    fn parse_macro_def(&mut self, macro_type: MacroType, diagnostics: &mut Diagnostics) -> Result<Option<Macro>, Abort> {
        self.next();
        let name_option = self.parse_macro_name(diagnostics)?;
        let parameters = self.parse_macro_def_params(diagnostics)?;
        let instructions = self.parse_func_body(diagnostics)?;

        Ok(name_option.map(|name| Macro {
            macro_type,
            name: name,
            parameters: parameters,
            instr: instructions,
        }))
    }

    fn parse_macro_name(&mut self, diagnostics: &mut Diagnostics) -> Result<Option<Symbol>, Abort> {
        let token = self.require_current(diagnostics)?;

        if token.token_type == TokenType::Identifier {
            let macro_name = Symbol {
                content: token.content.clone(),
                span: token.span,
            };

            self.next();
            Ok(Some(macro_name))
            
        } else {
            let expected_types = vec![TokenType::Identifier];
            diagnostics.raise(Error::new(UnexpectedToken { expected_types }, token.span));
            Ok(None)
        }
    }

    fn parse_macro_def_params(&mut self, diagnostics: &mut Diagnostics) -> Result<Vec<Symbol>, Abort> {
        self.parse_list_params(Self::parse_register, diagnostics)
    }

    fn parse_instr(&mut self, diagnostics: &mut Diagnostics) -> Result<Option<Instruction>, Abort> {
        let instr_label_option = self.parse_label(diagnostics)?;
        self.expect(TokenType::Colon, diagnostics);

        let type_option = self.parse_instr_type(diagnostics)?;

        let zipped = instr_label_option.zip(type_option);
        let instr = zipped.map(|(label, instruction_type)| Instruction { 
            label,
            instruction_type
        });

        Ok(instr)
    }

    fn parse_instr_type(&mut self, diagnostics: &mut Diagnostics) -> Result<Option<InstructionType>, Abort> {
        let token = self.require_current(diagnostics)?;
        
        if token.token_type == TokenType::Do {
            let op_option = self.parse_instr_op(diagnostics)?;
            Ok(op_option.map(|operation| InstructionType::Operation(operation)))

        } else if token.token_type == TokenType::If {
            let test_option = self.parse_instr_test(diagnostics)?;
            Ok(test_option.map(|test| InstructionType::Test(test)))

        } else {
            let expected_types = vec![TokenType::Do, TokenType::If];
            diagnostics.raise(Error::new(UnexpectedToken { expected_types }, token.span));
            Ok(None)
        }
    }

    fn parse_instr_op(&mut self, diagnostics: &mut Diagnostics) -> Result<Option<Operation>, Abort> {
        self.next();
        let oper_type = self.parse_operation_type(diagnostics)?;

        self.expect(TokenType::Goto, diagnostics);
        let oper_label = self.parse_label(diagnostics)?;

        let zipped = oper_type.zip(oper_label);
        let operation = zipped.map(|(oper_type, oper_label)| Operation {
            oper_type: oper_type,
            next_label: oper_label,
        });

        Ok(operation)
    }

    fn parse_operation_type(&mut self, diagnostics: &mut Diagnostics) -> Result<Option<OperationType>, Abort> {
        let token = self.require_current(diagnostics)?;

        match token.token_type {
            TokenType::Identifier => {
                let macro_name = Symbol {
                    content: token.content.clone(),
                    span: token.span,
                };

                self.next();

                let parameters = self.parse_macro_params(diagnostics)?;
                Ok(Some(OperationType::Macro(macro_name, parameters)))
            },

            TokenType::BuiltInOper(oper) => {
                self.next();
                let parameter_option = self.parse_builtin_param(diagnostics)?;
                Ok(parameter_option.map(|parameter| OperationType::BuiltIn(oper, parameter)))
            },

            _ => {
                let expected_types = vec![TokenType::BuiltInOper(BuiltInOperation::Inc), TokenType::BuiltInOper(BuiltInOperation::Dec), TokenType::Identifier];
                diagnostics.raise(Error::new(UnexpectedToken { expected_types }, token.span));
                Ok(None)
            }
        }
            
    }

    fn parse_instr_test(&mut self, diagnostics: &mut Diagnostics) -> Result<Option<Test>, Abort> {
        self.next();
        let test_type = self.parse_test_type(diagnostics)?;

        self.expect(TokenType::Then, diagnostics);
        self.expect(TokenType::Goto, diagnostics);
        let then_label = self.parse_label(diagnostics)?;

        self.expect(TokenType::Else, diagnostics);
        self.expect(TokenType::Goto, diagnostics);
        let else_label = self.parse_label(diagnostics)?;

        let zipped = test_type.zip(then_label).zip(else_label);
        let test = zipped.map(|((test_type, then_label), else_label)| Test {
            test_type: test_type,
            next_true_label: then_label,
            next_false_label: else_label,
        });
        
        Ok(test)
    }

    fn parse_test_type(&mut self, diagnostics: &mut Diagnostics) -> Result<Option<TestType>, Abort> {
        let token = self.require_current(diagnostics)?;

        match token.token_type {
            TokenType::Identifier => {
                let macro_name = Symbol {
                    content: token.content.clone(),
                    span: token.span,
                };

                self.next();

                let parameters = self.parse_macro_params(diagnostics)?;
                Ok(Some(TestType::Macro(macro_name, parameters)))
            },

            TokenType::BuiltInTest(test) => {
                self.next();
                let parameter_option = self.parse_builtin_param(diagnostics)?;
                Ok(parameter_option.map(|parameter| TestType::BuiltIn(test, parameter)))
            },

            _ => {
                let expected_types = vec![TokenType::BuiltInTest(BuiltInTest::Zero), TokenType::Identifier];
                diagnostics.raise(Error::new(UnexpectedToken { expected_types }, token.span));
                Ok(None)
            }
        }
    }

    fn parse_builtin_param(&mut self, diagnostics: &mut Diagnostics) -> Result<Option<Symbol>, Abort> {
        let has_parens = self.check_expect(TokenType::OpenParen, diagnostics)?;
        let parameter = self.parse_register(diagnostics)?;
        if has_parens {
            self.expect(TokenType::CloseParen, diagnostics)?;
        }
        Ok(parameter)
    }

    fn parse_list_params<F, T>(&mut self, mut parse_param: F, diagnostics: &mut Diagnostics) -> Result<Vec<T>, Abort> 
    where 
        F: FnMut(&mut Self, &mut Diagnostics) -> Result<Option<T>, Abort>
    {
        self.expect(TokenType::OpenParen, diagnostics);

        let mut parameters = Vec::new();
        let mut needs_comma = false;

        while !self.check_expect(TokenType::CloseParen, diagnostics)? {
            if needs_comma {
                panic!("errooooou")
            }

            if let Some(parameter) = parse_param(self, diagnostics)? {
                parameters.push(parameter);
                needs_comma = !self.check_expect(TokenType::Comma, diagnostics)?;
            }
        }

        Ok(parameters)
    }

    fn parse_macro_params(&mut self, diagnostics: &mut Diagnostics) -> Result<Vec<MacroParam>, Abort> {
        self.parse_list_params(Self::parse_macro_param, diagnostics)
    }

    fn parse_macro_param(&mut self, diagnostics: &mut Diagnostics) -> Result<Option<MacroParam>, Abort> {
        let token = self.require_current(diagnostics)?;

        match token.token_type {
            TokenType::Identifier => {
                let symbol = Symbol {
                    content: token.content.clone(),
                    span: token.span,
                };
                self.next();
                Ok(Some(MacroParam::Register(symbol)))
            },

            TokenType::Number => {
                let constant = BigUint::from_str(&token.content).expect(
                    "Lexer só deve permitir tokens Number só com dígitos",
                );
                self.next();
                Ok(Some(MacroParam::Number(constant)))
            },

            _ => {
                let expected_types = vec![TokenType::Identifier, TokenType::Number];
                diagnostics.raise(Error::new(UnexpectedToken { expected_types }, token.span));
                Ok(None)
            }
        }
    }

    fn parse_register(&mut self, diagnostics: &mut Diagnostics) -> Result<Option<Symbol>, Abort> {
        let token = self.require_current(diagnostics)?;

        if token.token_type == TokenType::Identifier {
            let symbol = Symbol {
                content: token.content.clone(),
                span: token.span,
            };
            self.next();
            Ok(Some(symbol))
        } else {
            let expected_types = vec![TokenType::Identifier];
            diagnostics.raise(Error::new(UnexpectedToken { expected_types }, token.span));
            Ok(None)
        }
    }

    fn parse_label(&mut self, diagnostics: &mut Diagnostics) -> Result<Option<Symbol>, Abort> {
        let token = self.require_current(diagnostics)?;

        match token.token_type {
            TokenType::Number | TokenType::Identifier => {
                let label = Symbol {
                    content: token.content.clone(),
                    span: token.span,
                };

                self.next();
                Ok(Some(label))
            },

            _ => {
                let expected_types = vec![TokenType::Identifier, TokenType::Number];
                diagnostics.raise(Error::new(UnexpectedToken { expected_types }, token.span));
                Ok(None)
            }
        }
    }
}
