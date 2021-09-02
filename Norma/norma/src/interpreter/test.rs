use super::{
    program::{
        Instruction,
        InstructionKind,
        Operation,
        OperationKind,
        Program,
        Test,
        TestKind,
    },
    Interpreter,
};
use num_bigint::BigUint;
use num_traits::{One, Zero};

#[test]
fn id_program() {
    let mut program = Program::empty();

    program.insert(Instruction {
        label: String::from("1"),
        kind: InstructionKind::Test(Test {
            kind: TestKind::Zero(String::from("X")),
            next_then: String::from("0"),
            next_else: String::from("2"),
        }),
    });

    program.insert(Instruction {
        label: String::from("2"),
        kind: InstructionKind::Operation(Operation {
            kind: OperationKind::Dec(String::from("X")),
            next: String::from("3"),
        }),
    });

    program.insert(Instruction {
        label: String::from("3"),
        kind: InstructionKind::Operation(Operation {
            kind: OperationKind::Inc(String::from("Y")),
            next: String::from("1"),
        }),
    });

    let mut interpreter = Interpreter::new(program, Vec::new());

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
