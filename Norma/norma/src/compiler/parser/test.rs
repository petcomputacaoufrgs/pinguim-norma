use crate::compiler::{
    ast::*,
    error::Diagnostics,
    lexer::generate_tokens,
    parser::parse,
    token::*,
};
use indexmap::IndexMap;

#[test]
fn greater_than_one() {
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

    let expected_main = gt_one_main();
    let expected_mac = gt_one_macros();
    let expected_result = Program { main: expected_main, macros: expected_mac };

    let diagnostics = Diagnostics::new();
    let result = parse(generate_tokens(source_code, &mut diagnostics));

    eprintln!("{:#?}", result);
}

/// dec_x: do dec X goto inc_y
fn gt_one_main_dec_x() -> Instruction {
    Instruction {
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
                },
            },
        },
        instruction_type: InstructionType::Operation(Operation {
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
                        },
                    },
                },
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
                    },
                },
            },
        }),
    }
}

/// (Y, X,)
fn gt_one_main_inc_y_params() -> Vec<MacroParam> {
    vec![
        MacroParam::Register(Symbol {
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
                },
            },
        }),
        MacroParam::Register(Symbol {
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
                },
            },
        }),
    ]
}

/// inc_y: do incIfNotZero (Y, X,) goto 0
fn gt_one_main_inc_y() -> Instruction {
    let macro_params = gt_one_main_inc_y_params();

    Instruction {
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
                },
            },
        },
        instruction_type: InstructionType::Operation(Operation {
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
                        },
                    },
                },
                macro_params,
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
                    },
                },
            },
        }),
    }
}

/// test isNotZero (A) {
///     1: if zero A then goto false else goto true
/// }
fn gt_one_main() -> Main {
    let instr_dec_x = gt_one_main_dec_x();
    let instr_inc_y = gt_one_main_inc_y();

    let main_code = IndexMap::new();
    main_code.insert(instr_dec_x.label.content.clone(), instr_dec_x);
    main_code.insert(instr_inc_y.label.content.clone(), instr_inc_y);

    Main { code: main_code }
}

/// 1: if zero A then goto false else goto true
fn gt_one_is_not_zero_1() -> Instruction {
    Instruction {
        label: Symbol {
            content: String::from("1"),
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
                },
            },
        },
        instruction_type: InstructionType::Test(Test {
            test_type: TestType::BuiltIn(
                BuiltInTest::Zero,
                Symbol {
                    content: String::from("1"),
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
                        },
                    },
                },
            ),
            next_true_label: Symbol {
                content: String::from("false"),
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
                    },
                },
            },
            next_false_label: Symbol {
                content: String::from("true"),
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
                    },
                },
            },
        }),
    }
}

fn gt_one_is_not_zero() -> Macro {
    let mut code = IndexMap::new();
    let instr_1 = gt_one_is_not_zero_1();
    code.insert(instr_1.label.content.clone(), instr_1);

    Macro {
        macro_type: MacroType::Test,
        name: Symbol {
            content: String::from("isNotZero"),
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
                },
            },
        },
        parameters: vec![Symbol {
            content: String::from("A"),
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
                },
            },
        }],
        instr: code,
    }
}

/// 1: if notZero (B) then goto 2 else goto 0
fn gt_one_inc_if_nz_1() -> Instruction {
    Instruction {
        label: Symbol {
            content: String::from("1"),
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
                },
            },
        },
        instruction_type: InstructionType::Test(Test {
            test_type: TestType::BuiltIn(
                BuiltInTest::Zero,
                Symbol {
                    content: String::from("1"),
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
                        },
                    },
                },
            ),
            next_true_label: Symbol {
                content: String::from("2"),
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
                    },
                },
            },
            next_false_label: Symbol {
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
                    },
                },
            },
        }),
    }
}

/// 2: do inc (A) goto 0
fn gt_one_inc_if_nz_2() -> Instruction {
    Instruction {
        label: Symbol {
            content: String::from("2"),
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
                },
            },
        },
        instruction_type: InstructionType::Operation(Operation {
            oper_type: OperationType::BuiltIn(
                BuiltInOperation::Inc,
                Symbol {
                    content: String::from("A"),
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
                        },
                    },
                },
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
fn gt_one_inc_if_nz() -> Macro {
    let mut code = IndexMap::new();
    let instr_1 = gt_one_inc_if_nz_1();
    code.insert(instr_1.label.content.clone(), instr_1);
    let instr_2 = gt_one_inc_if_nz_2();
    code.insert(instr_2.label.content.clone(), instr_2);

    Macro {
        macro_type: MacroType::Operation,
        name: Symbol {
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
                },
            },
        },
        parameters: vec![
            Symbol {
                content: String::from("A"),
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
                    },
                },
            },
            Symbol {
                content: String::from("B"),
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
                    },
                },
            },
        ],
        instr: code,
    }
}

fn gt_one_macros() -> IndexMap<String, Macro> {
    let mut macros = IndexMap::<String, Macro>::new();

    let is_not_zero = gt_one_is_not_zero();
    macros.insert(is_not_zero.name.content.clone(), is_not_zero);

    let inc_if_nz = gt_one_inc_if_nz();
    macros.insert(inc_if_nz.name.content.clone(), inc_if_nz);

    macros
}
