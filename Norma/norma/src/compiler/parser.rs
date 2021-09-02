use crate::compiler::token::*;
use crate::compiler::ast::*;
use std::ops::Range;
use std::collections::HashMap;
use indexmap::IndexMap;

pub struct Parser {
    tokens: Vec<Token>,
    curr_token: usize,

}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            curr_token: 0
        }
    }

    pub fn current(&self) -> Option<&Token> {
        self.tokens.get(self.curr_token)
    } 

    pub fn next(&mut self) {
        self.curr_token += 1;
    } 

    pub fn parse_program(&mut self) -> Option<Program> {
        let mut macros = IndexMap::<String, Macro>::new();
        let mut main_option: Option<Main> = None;

        while let Some(token) = self.current() {
            match token.token_type {
                TokenType::Main => {
                    // conferir se existe mais de uma main
                    main_option = self.parse_main();
                },

                TokenType::Operation => {
                    if let Some(macro_aux) = self.parse_macro(MacroType::Operation) {
                        // conferir se ja nao esta no indexmap
                        macros.insert(macro_aux.name.content.clone(), macro_aux);
                    }
                },

                TokenType::Test => {
                    if let Some(macro_aux) = self.parse_macro(MacroType::Test) {
                        // conferir se ja nao esta no indexmap
                        macros.insert(macro_aux.name.content.clone(), macro_aux);
                    }
                },

                _ => panic!("Whatever")
            }
        }

        let program = Program {
            main: main_option?,
            macros,
        };

        Some(program)
    }

    pub fn parse_main(&mut self) -> Option<Main> {
        // ler todos os tokens da main e construir uma estrutura Main a partir disso
        self.next();
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
                None => panic!("Whatever")
            }
        }

        Some(Main { code })
    }

    pub fn parse_macro(&mut self, macro_type: MacroType) -> Option<Macro> {
        todo!()
    }

    pub fn parse_instr(&mut self) -> Option<Instruction> {
        let instr_label = self.parse_label();
        self.expect(TokenType::Colon);

        let type_option = self.parse_instr_type();
        let (instruction_type, parameters) = type_option?;

        let instr = Instruction {
            label: instr_label?,
            instruction_type,
            parameters,
        };

        Some(instr)
    }

    pub fn parse_instr_type(&mut self) -> Option<(InstructionType, Parameters)> {
        match self.current() {
            Some(token) => {
                if token.token_type == TokenType::Do {
                    let op_option = self.parse_instr_op();
                    let (operation, parameters) = op_option?;
                    Some((InstructionType::Operation(operation), parameters))

                } else if token.token_type == TokenType::If {
                    let test_option = self.parse_instr_test();
                    let (test, parameters) = test_option?;
                    Some((InstructionType::Test(test), parameters))

                } else {
                    panic!("Whatever")
                }
            },
            None => panic!("Whatever")
        }
    }

    pub fn parse_instr_op(&mut self) -> Option<(Operation, Parameters)> {
        todo!()
    }

    pub fn parse_operation_type(&mut self) -> Option<OperationType> {
        todo!()
        // criar enum BuiltIn para OperationType e TestType ????
    }

    pub fn parse_instr_test(&mut self) -> Option<(Test, Parameters)> {
        todo!()
    }

    pub fn parse_parameters() -> Option<Parameters> {
        todo!()
    }

    pub fn expect(&mut self, expected_type: TokenType) {
        // falta passar o diagnostics para cá depois
        match self.current() {
            Some(token) => {
                if token.token_type == expected_type {
                    self.next();
                } else {
                    panic!("Whatever") 
                }
            },
            None => panic!("Whatever") 
        }
    }

    pub fn parse_label(&mut self) -> Option<Symbol> {
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

                _ => panic!("Whaaaatever")
            },

            None => panic!("Whaaaatever")
        }
    } 
}


