use crate::{
    compiler::{
        lexer::token::{BuiltInOperation, BuiltInTest},
        parser::ast,
        position::{Position, Span},
    },
    interpreter::program::{
        Instruction, InstructionKind, Operation, OperationKind, Program, Test,
        TestKind,
    },
};
use indexmap::IndexMap;

pub fn source_code() -> &'static str {
    "operation clear (A) {
        check: if zero A then goto done else goto step
        step: do dec A goto check
    }

    test notZero (A, Temp) {
        1: do clear (Temp) goto 2
        2: if zero A then goto false else goto true
    }

    operation prepare (A, B, C) {
        check: if notZero (A, C) then goto set else goto done
        set: do clear (B) goto done
    }

    main {
        1: do inc Y goto 2
        2: do prepare (X, Y, A) goto 3
        3: do inc Y goto 0
    }"
}

pub fn runtime_program() -> Program {
    let mut program = Program::empty();

    program.insert(Instruction::new(
        String::from("1"),
        InstructionKind::Operation(Operation {
            kind: OperationKind::Inc(String::from("Y")),
            next: String::from("2.prepare.check.notZero.1.clear.check"),
        }),
    ));
    program.insert(Instruction::new(
        String::from("2.prepare.check.notZero.1.clear.check"),
        InstructionKind::Test(Test {
            kind: TestKind::Zero(String::from("A")),
            next_then: String::from("2.prepare.check.notZero.2"),
            next_else: String::from("2.prepare.check.notZero.1.clear.step"),
        }),
    ));
    program.insert(Instruction::new(
        String::from("2.prepare.check.notZero.1.clear.step"),
        InstructionKind::Operation(Operation {
            kind: OperationKind::Dec(String::from("A")),
            next: String::from("2.prepare.check.notZero.1.clear.check"),
        }),
    ));
    program.insert(Instruction::new(
        String::from("2.prepare.check.notZero.2"),
        InstructionKind::Test(Test {
            kind: TestKind::Zero(String::from("X")),
            next_then: String::from("3"),
            next_else: String::from("2.prepare.set.clear.check"),
        }),
    ));
    program.insert(Instruction::new(
        String::from("2.prepare.set.clear.check"),
        InstructionKind::Test(Test {
            kind: TestKind::Zero(String::from("Y")),
            next_then: String::from("3"),
            next_else: String::from("2.prepare.set.clear.step"),
        }),
    ));
    program.insert(Instruction::new(
        String::from("2.prepare.set.clear.step"),
        InstructionKind::Operation(Operation {
            kind: OperationKind::Dec(String::from("Y")),
            next: String::from("2.prepare.set.clear.check"),
        }),
    ));
    program.insert(Instruction::new(
        String::from("3"),
        InstructionKind::Operation(Operation {
            kind: OperationKind::Inc(String::from("Y")),
            next: String::from("0"),
        }),
    ));

    program
}
