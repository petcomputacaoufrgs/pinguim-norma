#[cfg(test)]
mod test;

use crate::compiler::{ast::*, token::*};
use indexmap::IndexMap;
use num_bigint::BigUint;
use std::str::FromStr;

pub fn parse(tokens: Vec<Token>) -> Option<Program> {
    Parser::new(tokens).parse_program()
}

pub struct Abort;

#[derive(Clone, Debug)]
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

    fn next(&mut self) {
        self.curr_token += 1;
    }

    fn expect(&mut self, expected_type: TokenType) {
        // falta passar o diagnostics para cá depois
        match self.current() {
            Some(token) => {
                if token.token_type == expected_type {
                    self.next();
                } else {
                    panic!("Whatever")
                }
            },
            None => panic!("Whatever"),
        }
    }

    fn check_expect(&mut self, expected_type: TokenType) -> bool {
        match self.current() {
            Some(token) => {
                if token.token_type == expected_type {
                    self.next();
                    true
                } else {
                    false
                }
            },
            None => {
                panic!("ver dps");
                true
            },
        }
    }

    fn parse_program(&mut self) -> Option<Program> {
        let mut macros = IndexMap::<String, Macro>::new();
        let mut main_option: Option<Main> = None;

        while let Some(token) = self.current() {
            match token.token_type {
                TokenType::Main => {
                    // conferir se existe mais de uma main
                    main_option = self.parse_main();
                },

                TokenType::Operation => {
                    if let Some(macro_aux) =
                        self.parse_macro_def(MacroType::Operation)
                    {
                        // conferir se ja nao esta no indexmap
                        macros
                            .insert(macro_aux.name.content.clone(), macro_aux);
                    }
                },

                TokenType::Test => {
                    if let Some(macro_aux) =
                        self.parse_macro_def(MacroType::Test)
                    {
                        // conferir se ja nao esta no indexmap
                        macros
                            .insert(macro_aux.name.content.clone(), macro_aux);
                    }
                },

                _ => panic!("Whatever"),
            }
        }

        let program = Program { main: main_option?, macros };

        Some(program)
    }

    fn parse_main(&mut self) -> Option<Main> {
        self.next();
        let instructions = self.parse_func_body();

        Some(Main { code: instructions? })
    }

    fn parse_func_body(&mut self) -> Option<IndexMap<String, Instruction>> {
        self.expect(TokenType::OpenCurly);
        let mut code = IndexMap::<String, Instruction>::new();

        loop {
            match self.current() {
                Some(token) => {
                    if token.token_type == TokenType::CloseCurly {
                        self.next();
                        break;
                    } else if let Some(instr) = self.parse_instr() {
                        // fazer a verificação de label duplicado
                        code.insert(instr.label.content.clone(), instr);
                    }
                },
                None => panic!("Whatever"),
            }
        }

        Some(code)
    }

    fn parse_macro_def(&mut self, macro_type: MacroType) -> Option<Macro> {
        self.next();
        let name = self.parse_macro_name();
        let parameters = self.parse_macro_def_params();
        let instructions = self.parse_func_body();

        let macro_def = Macro {
            macro_type,
            name: name?,
            parameters: parameters?,
            instr: instructions?,
        };

        Some(macro_def)
    }

    fn parse_macro_name(&mut self) -> Option<Symbol> {
        match self.current() {
            Some(token) => {
                if token.token_type == TokenType::Identifier {
                    let macro_name = Symbol {
                        content: token.content.clone(),
                        span: token.span,
                    };

                    self.next();
                    Some(macro_name)
                } else {
                    panic!("erro")
                }
            }, 
            None => panic!("erro dps")
        }
    }

    fn parse_macro_def_params(&mut self) -> Option<Vec<Symbol>> {
        self.parse_list_params(Self::parse_register)
    }

    fn parse_instr(&mut self) -> Option<Instruction> {
        let instr_label = self.parse_label();
        self.expect(TokenType::Colon);

        let type_option = self.parse_instr_type();
        let instruction_type = type_option?;

        let instr = Instruction { label: instr_label?, instruction_type };

        Some(instr)
    }

    fn parse_instr_type(&mut self) -> Option<InstructionType> {
        match self.current() {
            Some(token) => {
                if token.token_type == TokenType::Do {
                    let op_option = self.parse_instr_op();
                    let operation = op_option?;
                    Some(InstructionType::Operation(operation))
                } else if token.token_type == TokenType::If {
                    let test_option = self.parse_instr_test();
                    let test = test_option?;
                    Some(InstructionType::Test(test))
                } else {
                    panic!("Whatever")
                }
            },
            None => panic!("Whatever"),
        }
    }

    fn parse_instr_op(&mut self) -> Option<Operation> {
        self.next();
        let oper_type = self.parse_operation_type();

        self.expect(TokenType::Goto);
        let next_label = self.parse_label();

        let operation =
            Operation { oper_type: oper_type?, next_label: next_label? };

        Some(operation)
    }

    fn parse_operation_type(&mut self) -> Option<OperationType> {
        match self.current() {
            Some(token) => match token.token_type {
                TokenType::Identifier => {
                    let macro_name = Symbol {
                        content: token.content.clone(),
                        span: token.span,
                    };

                    self.next();

                    let parameters = self.parse_macro_params();
                    Some(OperationType::Macro(macro_name, parameters?))
                },

                TokenType::BuiltInOper(oper) => {
                    self.next();
                    let parameter = self.parse_builtin_param();
                    Some(OperationType::BuiltIn(oper, parameter?))
                },

                _ => panic!("Erro"),
            },
            None => panic!("AAAA"),
        }
    }

    fn parse_instr_test(&mut self) -> Option<Test> {
        self.next();
        let test_type = self.parse_test_type();

        self.expect(TokenType::Then);
        self.expect(TokenType::Goto);
        let then_label = self.parse_label();

        self.expect(TokenType::Else);
        self.expect(TokenType::Goto);
        let else_label = self.parse_label();

        let test = Test {
            test_type: test_type?,
            next_true_label: then_label?,
            next_false_label: else_label?,
        };

        Some(test)
    }

    fn parse_test_type(&mut self) -> Option<TestType> {
        match self.current() {
            Some(token) => match token.token_type {
                TokenType::Identifier => {
                    let macro_name = Symbol {
                        content: token.content.clone(),
                        span: token.span,
                    };

                    self.next();

                    let parameters = self.parse_macro_params();
                    Some(TestType::Macro(macro_name, parameters?))
                },

                TokenType::BuiltInTest(test) => {
                    self.next();
                    let parameter = self.parse_builtin_param();
                    Some(TestType::BuiltIn(test, parameter?))
                },

                _ => panic!("Erro"),
            },
            None => panic!("AAAA"),
        }
    }

    fn parse_builtin_param(&mut self) -> Option<Symbol> {
        let has_parens = self.check_expect(TokenType::OpenParen);
        let parameter = self.parse_register();
        if has_parens {
            self.expect(TokenType::CloseParen);
        }
        parameter
    }

    fn parse_list_params<F, T>(&mut self, mut parse_param: F) -> Option<Vec<T>> 
    where 
        F: FnMut(&mut Self) -> Option<T>
    {
        self.expect(TokenType::OpenParen);

        let mut parameters = Vec::new();
        let mut needs_comma = false;

        while !self.check_expect(TokenType::CloseParen) {
            if needs_comma {
                panic!("errooooou")
            }

            if let Some(parameter) = parse_param(self) {
                parameters.push(parameter);
                needs_comma = !self.check_expect(TokenType::Comma);
            }
        }

        Some(parameters)
    }

    fn parse_macro_params(&mut self) -> Option<Vec<MacroParam>> {
        self.parse_list_params(Self::parse_macro_param)
    }

    fn parse_macro_param(&mut self) -> Option<MacroParam> {
        match self.current() {
            Some(token) => match token.token_type {
                TokenType::Identifier => {
                    let symbol = Symbol {
                        content: token.content.clone(),
                        span: token.span,
                    };
                    self.next();
                    Some(MacroParam::Register(symbol))
                },

                TokenType::Number => {
                    let constant = BigUint::from_str(&token.content).expect(
                        "Lexer só deve permitir tokens Number só com dígitos",
                    );
                    self.next();
                    Some(MacroParam::Number(constant))
                },

                _ => panic!("erro dps"),
            },
            None => panic!("erro dps"),
        }
    }

    fn parse_register(&mut self) -> Option<Symbol> {
        match self.current() {
            Some(token) => {
                if token.token_type == TokenType::Identifier {
                    let symbol = Symbol {
                        content: token.content.clone(),
                        span: token.span,
                    };
                    self.next();
                    Some(symbol)
                } else {
                    panic!("erro dps")
                }
            },
            None => panic!("erro dps"),
        }
    }

    fn parse_label(&mut self) -> Option<Symbol> {
        match self.current() {
            Some(token) => match token.token_type {
                TokenType::Number | TokenType::Identifier => {
                    let label = Symbol {
                        content: token.content.clone(),
                        span: token.span,
                    };

                    self.next();
                    Some(label)
                },

                _ => panic!("Whaaaatever"),
            },

            None => panic!("Whaaaatever"),
        }
    }
}
