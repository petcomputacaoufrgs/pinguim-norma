use crate::compiler::token::*;
use crate::compiler::lexer::generate_tokens;
use crate::compiler::ast::*;
use crate::compiler::parser::parse;
use crate::compiler::error::Diagnostics;
use indexmap::IndexMap;

#[test]
fn main_and_macro() {
    let source_code = "test isNotZero (A) {
        1: if zero A then goto false else goto true
    }
    
    operation incIfNotZero (A, B) {
        1: if notZero (B) then goto 2 else goto 0
        2: do inc (A) goto 0
    }
    
    //  Y(X) = {
    //      1 se X > 1
    //      0 se X <= 1
    //  }
    main {
        dec_x: do dec X goto inc_y
        inc_y: do incIfNotZero (Y, X,) goto 0
    }";


    // dec_x: do dec X goto inc_y
    let instr_main_dec_x = Instruction {
        label: Symbol {
            content: String::from("dec_x"),
            span: Span {
                start: Position {
                    utf8_index: 0,
                    utf16_index: 0,
                    line: 0,
                    column: 0,
                },
                end: Position {
                    utf8_index: 0,
                    utf16_index: 0,
                    line: 0,
                    column: 0,
                }
            }
        },
        instruction_type: InstructionType::Operation(
            Operation {
                oper_type: OperationType::BuiltIn(
                    BuiltInOperation::Dec,
                    Symbol {
                        content: String::from("X"),
                        span: Span {
                            start: Position {
                                utf8_index: 0,
                                utf16_index: 0,
                                line: 0,
                                column: 0,
                            },
                            end: Position {
                                utf8_index: 0,
                                utf16_index: 0,
                                line: 0,
                                column: 0,
                            }
                        }
                    }
                ),
                next_label: Symbol {
                    content: String::from("inc_y"),
                    span: Span {
                        start: Position {
                            utf8_index: 0,
                            utf16_index: 0,
                            line: 0,
                            column: 0,
                        },
                        end: Position {
                            utf8_index: 0,
                            utf16_index: 0,
                            line: 0,
                            column: 0,
                        }
                    }
                }
            }
        )
    };

    // inc_y: do incIfNotZero (Y, X,) goto 0
    let incIfNotZero_macro_params = Vec::<MacroParam>::new();

    incIfNotZero_macro_params.push(
        MacroParam::Register(
            Symbol {
                content: String::from("Y"),
                span: Span {
                    start: Position {
                        utf8_index: 0,
                        utf16_index: 0,
                        line: 0,
                        column: 0,
                    },
                    end: Position {
                        utf8_index: 0,
                        utf16_index: 0,
                        line: 0,
                        column: 0,
                    }
                }
            }
        )
    );

    incIfNotZero_macro_params.push(
        MacroParam::Register(
            Symbol {
                content: String::from("X"),
                span: Span {
                    start: Position {
                        utf8_index: 0,
                        utf16_index: 0,
                        line: 0,
                        column: 0,
                    },
                    end: Position {
                        utf8_index: 0,
                        utf16_index: 0,
                        line: 0,
                        column: 0,
                    }
                }
            }
        )
    );

    let instr_main_inc_y = Instruction {
        label: Symbol {
            content: String::from("inc_y"),
            span: Span {
                start: Position {
                    utf8_index: 0,
                    utf16_index: 0,
                    line: 0,
                    column: 0,
                },
                end: Position {
                    utf8_index: 0,
                    utf16_index: 0,
                    line: 0,
                    column: 0,
                }
            }
        },
        instruction_type: InstructionType::Operation(
            Operation {
                oper_type: OperationType::Macro(
                    Symbol {
                        content: String::from("incIfNotZero"),
                        span: Span {
                            start: Position {
                                utf8_index: 0,
                                utf16_index: 0,
                                line: 0,
                                column: 0,
                            },
                            end: Position {
                                utf8_index: 0,
                                utf16_index: 0,
                                line: 0,
                                column: 0,
                            }
                        }
                    },
                    incIfNotZero_macro_params,                    
                ),
                next_label: Symbol {
                    content: String::from("0"),
                    span: Span {
                        start: Position {
                            utf8_index: 0,
                            utf16_index: 0,
                            line: 0,
                            column: 0,
                        },
                        end: Position {
                            utf8_index: 0,
                            utf16_index: 0,
                            line: 0,
                            column: 0,
                        }
                    }
                }
            }
        )
    };

    let main_code = IndexMap::<String, Instruction>::new();
    main_code.insert(instr_main_dec_x.label.content.clone(), instr_main_dec_x);
    main_code.insert(instr_main_inc_y.label.content.clone(), instr_main_inc_y);

    // test isNotZero (A) {
    //     1: if zero A then goto false else goto true
    // }


    let main = Main {
        code: main_code, 
    };

    let macros = IndexMap::<String, Macro>::new();

    let expected_result = Program {
        
    };

    let diagnostics = Diagnostics::new(); 
    let result = parse(generate_tokens(source_code, &mut diagnostics));
}



