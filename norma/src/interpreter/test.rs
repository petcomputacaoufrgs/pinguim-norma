use super::{
    program::{
        Instruction, InstructionKind, Operation, OperationKind, Program, Test,
        TestKind,
    },
    Interpreter,
};
use crate::machine::Machine;
use num_bigint::BigUint;
use num_traits::{One, Zero};

#[test]
fn collect_registers() {
    let mut program = Program::empty();

    program.insert(Instruction::new(
        String::from("1"),
        InstructionKind::Test(Test {
            kind: TestKind::Zero(String::from("X")),
            next_then: String::from("0"),
            next_else: String::from("2"),
        }),
    ));
    program.insert(Instruction::new(
        String::from("2"),
        InstructionKind::Operation(Operation {
            kind: OperationKind::Dec(String::from("A")),
            next: String::from("3"),
        }),
    ));
    program.insert(Instruction::new(
        String::from("3"),
        InstructionKind::Operation(Operation {
            kind: OperationKind::Dec(String::from("B")),
            next: String::from("4"),
        }),
    ));
    program.insert(Instruction::new(
        String::from("4"),
        InstructionKind::Operation(Operation {
            kind: OperationKind::Dec(String::from("C")),
            next: String::from("5"),
        }),
    ));
    program.insert(Instruction::new(
        String::from("5"),
        InstructionKind::Operation(Operation {
            kind: OperationKind::Inc(String::from("assim")),
            next: String::from("0"),
        }),
    ));

    let mut machine = Machine::new(BigUint::one());
    program.collect_registers(|reg_name| {
        machine.create(reg_name);
    });

    assert_eq!(machine.get_value("X"), BigUint::one());
    assert_eq!(machine.get_value("Y"), BigUint::zero());
    assert_eq!(machine.get_value("A"), BigUint::zero());
    assert_eq!(machine.get_value("B"), BigUint::zero());
    assert_eq!(machine.get_value("C"), BigUint::zero());
    assert_eq!(machine.get_value("assim"), BigUint::zero());

    let mut names = machine.register_names().collect::<Vec<_>>();
    names.sort();
    assert_eq!(names, &["A", "B", "C", "X", "Y", "assim"]);
}

#[test]
fn id_program() {
    // Y = X
    let mut program = Program::empty();

    program.insert(Instruction::new(
        String::from("1"),
        InstructionKind::Test(Test {
            kind: TestKind::Zero(String::from("X")),
            next_then: String::from("0"),
            next_else: String::from("2"),
        }),
    ));
    program.insert(Instruction::new(
        String::from("2"),
        InstructionKind::Operation(Operation {
            kind: OperationKind::Dec(String::from("X")),
            next: String::from("3"),
        }),
    ));
    program.insert(Instruction::new(
        String::from("3"),
        InstructionKind::Operation(Operation {
            kind: OperationKind::Inc(String::from("Y")),
            next: String::from("1"),
        }),
    ));

    let mut interpreter = Interpreter::new(program);

    interpreter.input(BigUint::zero());
    interpreter.run_all();
    assert_eq!(interpreter.output(), BigUint::zero());

    interpreter.reset();
    interpreter.input(BigUint::one());
    interpreter.run_all();
    assert_eq!(interpreter.output(), BigUint::one());

    interpreter.reset();
    interpreter.input(BigUint::from(7u8));
    interpreter.run_all();
    assert_eq!(interpreter.output(), BigUint::from(7u8));
}

#[test]
fn program_2x_plus_3() {
    // Y = 2*X + 3
    let mut program = Program::empty();

    program.insert(Instruction::new(
        String::from("1"),
        InstructionKind::Test(Test {
            kind: TestKind::Zero(String::from("X")),
            next_then: String::from("5"),
            next_else: String::from("2"),
        }),
    ));
    program.insert(Instruction::new(
        String::from("2"),
        InstructionKind::Operation(Operation {
            kind: OperationKind::Dec(String::from("X")),
            next: String::from("3"),
        }),
    ));
    program.insert(Instruction::new(
        String::from("3"),
        InstructionKind::Operation(Operation {
            kind: OperationKind::Inc(String::from("Y")),
            next: String::from("4"),
        }),
    ));
    program.insert(Instruction::new(
        String::from("4"),
        InstructionKind::Operation(Operation {
            kind: OperationKind::Inc(String::from("Y")),
            next: String::from("1"),
        }),
    ));
    program.insert(Instruction::new(
        String::from("5"),
        InstructionKind::Operation(Operation {
            kind: OperationKind::Inc(String::from("Y")),
            next: String::from("6"),
        }),
    ));
    program.insert(Instruction::new(
        String::from("6"),
        InstructionKind::Operation(Operation {
            kind: OperationKind::Inc(String::from("Y")),
            next: String::from("7"),
        }),
    ));
    program.insert(Instruction::new(
        String::from("7"),
        InstructionKind::Operation(Operation {
            kind: OperationKind::Inc(String::from("Y")),
            next: String::from("0"),
        }),
    ));

    let mut interpreter = Interpreter::new(program);

    interpreter.input(BigUint::zero());
    interpreter.run_all();
    assert_eq!(interpreter.output(), BigUint::from(3u8));

    interpreter.reset();
    interpreter.input(BigUint::one());
    interpreter.run_all();
    assert_eq!(interpreter.output(), BigUint::from(5u8));

    interpreter.reset();
    interpreter.input(BigUint::from(7u8));
    interpreter.run_all();
    assert_eq!(interpreter.output(), BigUint::from(17u8));
}

#[test]
fn x_is_odd() {
    // Y = X % 2
    let mut program = Program::empty();

    program.insert(Instruction::new(
        String::from("1"),
        InstructionKind::Test(Test {
            kind: TestKind::Zero(String::from("X")),
            next_then: String::from("0"),
            next_else: String::from("2"),
        }),
    ));
    program.insert(Instruction::new(
        String::from("2"),
        InstructionKind::Operation(Operation {
            kind: OperationKind::Dec(String::from("X")),
            next: String::from("3"),
        }),
    ));
    program.insert(Instruction::new(
        String::from("3"),
        InstructionKind::Test(Test {
            kind: TestKind::Zero(String::from("X")),
            next_then: String::from("5"),
            next_else: String::from("4"),
        }),
    ));
    program.insert(Instruction::new(
        String::from("4"),
        InstructionKind::Operation(Operation {
            kind: OperationKind::Dec(String::from("X")),
            next: String::from("1"),
        }),
    ));
    program.insert(Instruction::new(
        String::from("5"),
        InstructionKind::Operation(Operation {
            kind: OperationKind::Inc(String::from("Y")),
            next: String::from("0"),
        }),
    ));

    let mut interpreter = Interpreter::new(program);

    interpreter.input(BigUint::zero());
    interpreter.run_all();
    assert_eq!(interpreter.output(), BigUint::zero());

    interpreter.reset();
    interpreter.input(BigUint::one());
    interpreter.run_all();
    assert_eq!(interpreter.output(), BigUint::one());

    interpreter.reset();
    interpreter.input(BigUint::from(7u8));
    interpreter.run_all();
    assert_eq!(interpreter.output(), BigUint::one());

    interpreter.reset();
    interpreter.input(BigUint::from(20u8));
    interpreter.run_all();
    assert_eq!(interpreter.output(), BigUint::zero());
}

#[test]
fn x_square() {
    // Y = X ** 2
    //
    // add (A, X, B)
    // enquanto X > 0 {
    //      add (Y, A, B)
    //      dec X
    // }
    let mut program = Program::empty();

    program.insert(Instruction::new(
        String::from("1"),
        InstructionKind::Operation(Operation {
            kind: OperationKind::Add(
                String::from("A"),
                String::from("X"),
                String::from("B"),
            ),
            next: String::from("2"),
        }),
    ));

    program.insert(Instruction::new(
        String::from("2"),
        InstructionKind::Test(Test {
            kind: TestKind::Zero(String::from("X")),
            next_then: String::from("0"),
            next_else: String::from("3"),
        }),
    ));

    program.insert(Instruction::new(
        String::from("3"),
        InstructionKind::Operation(Operation {
            kind: OperationKind::Add(
                String::from("Y"),
                String::from("A"),
                String::from("B"),
            ),
            next: String::from("4"),
        }),
    ));

    program.insert(Instruction::new(
        String::from("4"),
        InstructionKind::Operation(Operation {
            kind: OperationKind::Dec(String::from("X")),
            next: String::from("2"),
        }),
    ));

    let mut interpreter = Interpreter::new(program);

    interpreter.input(BigUint::zero());
    interpreter.run_all();
    assert_eq!(interpreter.output(), BigUint::zero());

    interpreter.reset();
    interpreter.input(BigUint::one());
    interpreter.run_all();
    assert_eq!(interpreter.output(), BigUint::one());

    interpreter.reset();
    interpreter.input(BigUint::from(7u8));
    interpreter.run_all();
    assert_eq!(interpreter.output(), BigUint::from(49u8));

    interpreter.reset();
    interpreter.input(BigUint::from(20u8));
    interpreter.run_all();
    assert_eq!(interpreter.output(), BigUint::from(400u16));
}
