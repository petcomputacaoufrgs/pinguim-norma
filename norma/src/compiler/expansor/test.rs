use crate::compiler::{
    error::Diagnostics,
    expansor::expand,
    test::greater_than_one,
};

#[test]
fn expand_greater_than_one() {
    let ast = greater_than_one::ast();
    let expected = greater_than_one::runtime_program();
    let mut diagnostics = Diagnostics::new();
    let found = expand(&ast, &mut diagnostics).unwrap();

    assert_eq!(found, expected);
}
