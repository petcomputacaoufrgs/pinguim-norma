pub mod greater_than_one;
pub mod one_plus_is_zero;

use super::compile;

#[test]
fn compile_one_pls_isz() {
    let source = one_plus_is_zero::source_code();
    let expected_program = one_plus_is_zero::runtime_program();
    let program = compile(source).unwrap();

    assert_eq!(expected_program, program);
}

#[test]
fn compile_gt_one() {
    let source = greater_than_one::source_code();
    let expected_program = greater_than_one::runtime_program();
    let program = compile(source).unwrap();

    assert_eq!(expected_program, program);
}
