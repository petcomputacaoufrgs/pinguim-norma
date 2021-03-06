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
use indexmap::IndexMap;
use pinguim_language::position::{Position, Span};

pub fn source_code() -> &'static str {
    "test notZero (A) {
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
    }"
}

pub fn ast() -> ast::Program {
    let expected_main = ast_main();
    let expected_mac = ast_macros();
    ast::Program { main: expected_main, macros: expected_mac }
}

pub fn runtime_program() -> Program {
    let mut program = Program::empty();

    program.insert(Instruction::new(
        String::from("dec_x"),
        InstructionKind::Operation(Operation {
            kind: OperationKind::Dec(String::from("X")),
            next: String::from("inc_y.incIfNotZero.1.notZero.1"),
        }),
    ));
    program.insert(Instruction::new(
        String::from("inc_y.incIfNotZero.1.notZero.1"),
        InstructionKind::Test(Test {
            kind: TestKind::Zero(String::from("X")),
            next_then: String::from("0"),
            next_else: String::from("inc_y.incIfNotZero.2"),
        }),
    ));
    program.insert(Instruction::new(
        String::from("inc_y.incIfNotZero.2"),
        InstructionKind::Operation(Operation {
            kind: OperationKind::Inc(String::from("Y")),
            next: String::from("0"),
        }),
    ));

    program
}

/// dec_x: do dec X goto inc_y
fn ast_main_dec_x() -> ast::Instruction {
    ast::Instruction {
        label: ast::Symbol {
            content: String::from("dec_x"),
            span: Span {
                start: Position {
                    utf8_index: 301,
                    utf16_index: 301,
                    line: 15,
                    column: 9,
                },
                end: Position {
                    utf8_index: 306,
                    utf16_index: 306,
                    line: 15,
                    column: 14,
                },
            },
        },
        instruction_type: ast::InstructionType::Operation(ast::Operation {
            oper_type: ast::OperationType::BuiltIn(
                BuiltInOperation::Dec,
                ast::Symbol {
                    content: String::from("X"),
                    span: Span {
                        start: Position {
                            utf8_index: 315,
                            utf16_index: 315,
                            line: 15,
                            column: 23,
                        },
                        end: Position {
                            utf8_index: 316,
                            utf16_index: 316,
                            line: 15,
                            column: 24,
                        },
                    },
                },
            ),
            next_label: ast::Symbol {
                content: String::from("inc_y"),
                span: Span {
                    start: Position {
                        utf8_index: 322,
                        utf16_index: 322,
                        line: 15,
                        column: 30,
                    },
                    end: Position {
                        utf8_index: 327,
                        utf16_index: 327,
                        line: 15,
                        column: 35,
                    },
                },
            },
        }),
    }
}

/// (Y, X,)
fn ast_main_inc_y_params() -> Vec<ast::MacroArgument> {
    vec![
        ast::MacroArgument::Register(ast::Symbol {
            content: String::from("Y"),
            span: Span {
                start: Position {
                    utf8_index: 360,
                    utf16_index: 360,
                    line: 16,
                    column: 33,
                },
                end: Position {
                    utf8_index: 361,
                    utf16_index: 361,
                    line: 16,
                    column: 34,
                },
            },
        }),
        ast::MacroArgument::Register(ast::Symbol {
            content: String::from("X"),
            span: Span {
                start: Position {
                    utf8_index: 363,
                    utf16_index: 363,
                    line: 16,
                    column: 36,
                },
                end: Position {
                    utf8_index: 364,
                    utf16_index: 364,
                    line: 16,
                    column: 37,
                },
            },
        }),
    ]
}

/// inc_y: do incIfNotZero (Y, X,) goto 0
fn ast_main_inc_y() -> ast::Instruction {
    let macro_params = ast_main_inc_y_params();

    ast::Instruction {
        label: ast::Symbol {
            content: String::from("inc_y"),
            span: Span {
                start: Position {
                    utf8_index: 336,
                    utf16_index: 336,
                    line: 16,
                    column: 9,
                },
                end: Position {
                    utf8_index: 341,
                    utf16_index: 341,
                    line: 16,
                    column: 14,
                },
            },
        },
        instruction_type: ast::InstructionType::Operation(ast::Operation {
            oper_type: ast::OperationType::Macro(
                ast::Symbol {
                    content: String::from("incIfNotZero"),
                    span: Span {
                        start: Position {
                            utf8_index: 346,
                            utf16_index: 346,
                            line: 16,
                            column: 19,
                        },
                        end: Position {
                            utf8_index: 358,
                            utf16_index: 358,
                            line: 16,
                            column: 31,
                        },
                    },
                },
                macro_params,
            ),
            next_label: ast::Symbol {
                content: String::from("0"),
                span: Span {
                    start: Position {
                        utf8_index: 372,
                        utf16_index: 372,
                        line: 16,
                        column: 45,
                    },
                    end: Position {
                        utf8_index: 373,
                        utf16_index: 373,
                        line: 16,
                        column: 46,
                    },
                },
            },
        }),
    }
}

fn ast_main() -> ast::Main {
    let instr_dec_x = ast_main_dec_x();
    let instr_inc_y = ast_main_inc_y();

    let mut main_code = IndexMap::new();
    main_code.insert(instr_dec_x.label.content.clone(), instr_dec_x);
    main_code.insert(instr_inc_y.label.content.clone(), instr_inc_y);

    ast::Main { code: main_code }
}

/// 1: if zero A then goto false else goto true
fn ast_not_zero_1() -> ast::Instruction {
    ast::Instruction {
        label: ast::Symbol {
            content: String::from("1"),
            span: Span {
                start: Position {
                    utf8_index: 27,
                    utf16_index: 27,
                    line: 2,
                    column: 9,
                },
                end: Position {
                    utf8_index: 28,
                    utf16_index: 28,
                    line: 2,
                    column: 10,
                },
            },
        },
        instruction_type: ast::InstructionType::Test(ast::Test {
            test_type: ast::TestType::BuiltIn(
                BuiltInTest::Zero,
                ast::Symbol {
                    content: String::from("A"),
                    span: Span {
                        start: Position {
                            utf8_index: 38,
                            utf16_index: 38,
                            line: 2,
                            column: 20,
                        },
                        end: Position {
                            utf8_index: 39,
                            utf16_index: 39,
                            line: 2,
                            column: 21,
                        },
                    },
                },
            ),
            next_true_label: ast::Symbol {
                content: String::from("false"),
                span: Span {
                    start: Position {
                        utf8_index: 50,
                        utf16_index: 50,
                        line: 2,
                        column: 32,
                    },
                    end: Position {
                        utf8_index: 55,
                        utf16_index: 55,
                        line: 2,
                        column: 37,
                    },
                },
            },
            next_false_label: ast::Symbol {
                content: String::from("true"),
                span: Span {
                    start: Position {
                        utf8_index: 66,
                        utf16_index: 66,
                        line: 2,
                        column: 48,
                    },
                    end: Position {
                        utf8_index: 70,
                        utf16_index: 70,
                        line: 2,
                        column: 52,
                    },
                },
            },
        }),
    }
}

/// test notZero (A) {
///    1: if zero A then goto false else goto true
/// }
fn ast_not_zero() -> ast::Macro {
    let mut code = IndexMap::new();
    let instr_1 = ast_not_zero_1();
    code.insert(instr_1.label.content.clone(), instr_1);

    ast::Macro {
        macro_type: ast::MacroType::Test,
        name: ast::Symbol {
            content: String::from("notZero"),
            span: Span {
                start: Position {
                    utf8_index: 5,
                    utf16_index: 5,
                    line: 1,
                    column: 6,
                },
                end: Position {
                    utf8_index: 12,
                    utf16_index: 12,
                    line: 1,
                    column: 13,
                },
            },
        },
        parameters: vec![ast::Symbol {
            content: String::from("A"),
            span: Span {
                start: Position {
                    utf8_index: 14,
                    utf16_index: 14,
                    line: 1,
                    column: 15,
                },
                end: Position {
                    utf8_index: 15,
                    utf16_index: 15,
                    line: 1,
                    column: 16,
                },
            },
        }],
        instr: code,
    }
}

/// (B)
fn ast_inc_if_nz_1_params() -> Vec<ast::MacroArgument> {
    vec![ast::MacroArgument::Register(ast::Symbol {
        content: String::from("B"),
        span: Span {
            start: Position {
                utf8_index: 141,
                utf16_index: 141,
                line: 6,
                column: 24,
            },
            end: Position {
                utf8_index: 142,
                utf16_index: 142,
                line: 6,
                column: 25,
            },
        },
    })]
}

/// 1: if notZero (B) then goto 2 else goto 0
fn ast_inc_if_nz_1() -> ast::Instruction {
    ast::Instruction {
        label: ast::Symbol {
            content: String::from("1"),
            span: Span {
                start: Position {
                    utf8_index: 126,
                    utf16_index: 126,
                    line: 6,
                    column: 9,
                },
                end: Position {
                    utf8_index: 127,
                    utf16_index: 127,
                    line: 6,
                    column: 10,
                },
            },
        },
        instruction_type: ast::InstructionType::Test(ast::Test {
            test_type: ast::TestType::Macro(
                ast::Symbol {
                    content: String::from("notZero"),
                    span: Span {
                        start: Position {
                            utf8_index: 132,
                            utf16_index: 132,
                            line: 6,
                            column: 15,
                        },
                        end: Position {
                            utf8_index: 139,
                            utf16_index: 139,
                            line: 6,
                            column: 22,
                        },
                    },
                },
                ast_inc_if_nz_1_params(),
            ),
            next_true_label: ast::Symbol {
                content: String::from("2"),
                span: Span {
                    start: Position {
                        utf8_index: 154,
                        utf16_index: 154,
                        line: 6,
                        column: 37,
                    },
                    end: Position {
                        utf8_index: 155,
                        utf16_index: 155,
                        line: 6,
                        column: 38,
                    },
                },
            },
            next_false_label: ast::Symbol {
                content: String::from("0"),
                span: Span {
                    start: Position {
                        utf8_index: 166,
                        utf16_index: 166,
                        line: 6,
                        column: 49,
                    },
                    end: Position {
                        utf8_index: 167,
                        utf16_index: 167,
                        line: 6,
                        column: 50,
                    },
                },
            },
        }),
    }
}

/// 2: do inc (A) goto 0
fn ast_inc_if_nz_2() -> ast::Instruction {
    ast::Instruction {
        label: ast::Symbol {
            content: String::from("2"),
            span: Span {
                start: Position {
                    utf8_index: 176,
                    utf16_index: 176,
                    line: 7,
                    column: 9,
                },
                end: Position {
                    utf8_index: 177,
                    utf16_index: 177,
                    line: 7,
                    column: 10,
                },
            },
        },
        instruction_type: ast::InstructionType::Operation(ast::Operation {
            oper_type: ast::OperationType::BuiltIn(
                BuiltInOperation::Inc,
                ast::Symbol {
                    content: String::from("A"),
                    span: Span {
                        start: Position {
                            utf8_index: 187,
                            utf16_index: 187,
                            line: 7,
                            column: 20,
                        },
                        end: Position {
                            utf8_index: 188,
                            utf16_index: 188,
                            line: 7,
                            column: 21,
                        },
                    },
                },
            ),
            next_label: ast::Symbol {
                content: String::from("0"),
                span: Span {
                    start: Position {
                        utf8_index: 195,
                        utf16_index: 195,
                        line: 7,
                        column: 28,
                    },
                    end: Position {
                        utf8_index: 196,
                        utf16_index: 196,
                        line: 7,
                        column: 29,
                    },
                },
            },
        }),
    }
}

/// operation incIfNotZero (A, B) {
///     1: if notZero (B) then goto 2 else goto 0
///     2: do inc (A) goto 0
/// }
fn ast_inc_if_nz() -> ast::Macro {
    let mut code = IndexMap::new();
    let instr_1 = ast_inc_if_nz_1();
    code.insert(instr_1.label.content.clone(), instr_1);
    let instr_2 = ast_inc_if_nz_2();
    code.insert(instr_2.label.content.clone(), instr_2);

    ast::Macro {
        macro_type: ast::MacroType::Operation,
        name: ast::Symbol {
            content: String::from("incIfNotZero"),
            span: Span {
                start: Position {
                    utf8_index: 96,
                    utf16_index: 96,
                    line: 5,
                    column: 15,
                },
                end: Position {
                    utf8_index: 108,
                    utf16_index: 108,
                    line: 5,
                    column: 27,
                },
            },
        },
        parameters: vec![
            ast::Symbol {
                content: String::from("A"),
                span: Span {
                    start: Position {
                        utf8_index: 110,
                        utf16_index: 110,
                        line: 5,
                        column: 29,
                    },
                    end: Position {
                        utf8_index: 111,
                        utf16_index: 111,
                        line: 5,
                        column: 30,
                    },
                },
            },
            ast::Symbol {
                content: String::from("B"),
                span: Span {
                    start: Position {
                        utf8_index: 113,
                        utf16_index: 113,
                        line: 5,
                        column: 32,
                    },
                    end: Position {
                        utf8_index: 114,
                        utf16_index: 114,
                        line: 5,
                        column: 33,
                    },
                },
            },
        ],
        instr: code,
    }
}

fn ast_macros() -> IndexMap<String, ast::Macro> {
    let mut macros = IndexMap::<String, ast::Macro>::new();

    let not_zero = ast_not_zero();
    macros.insert(not_zero.name.content.clone(), not_zero);

    let ast_inc_if_nz = ast_inc_if_nz();
    macros.insert(ast_inc_if_nz.name.content.clone(), ast_inc_if_nz);

    macros
}
