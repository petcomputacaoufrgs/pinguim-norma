use super::Machine;
use num_bigint::BigUint;
use num_traits::identities::{One, Zero};

fn make_machine() -> Machine {
    let mut machine = Machine::new(BigUint::from(4u8));
    machine.insert("A");
    machine.insert_with_value("B", BigUint::from(13u8));
    machine.insert("Y");
    machine
}

#[test]
fn insert() {
    let mut machine = Machine::new(BigUint::from(4u8));
    assert_eq!(machine.get_value("X"), BigUint::from(4u8));

    machine.insert("A");
    assert_eq!(machine.get_value("A"), BigUint::zero());
    assert_eq!(machine.get_value("X"), BigUint::from(4u8));

    machine.insert_with_value("B", BigUint::from(13u8));
    assert_eq!(machine.get_value("B"), BigUint::from(13u8));
    assert_eq!(machine.get_value("A"), BigUint::zero());
    assert_eq!(machine.get_value("X"), BigUint::from(4u8));
}

#[test]
#[should_panic]
fn invalid_get() {
    let mut machine = make_machine();
    machine.get_value("K");
}

#[test]
fn inc() {
    let mut machine = make_machine();
    machine.inc("X");
    assert_eq!(machine.get_value("X"), BigUint::from(5u8));
    assert_eq!(machine.get_value("A"), BigUint::zero());
    assert_eq!(machine.get_value("B"), BigUint::from(13u8));
    assert_eq!(machine.get_value("Y"), BigUint::zero());

    machine.inc("B");
    assert_eq!(machine.get_value("X"), BigUint::from(5u8));
    assert_eq!(machine.get_value("A"), BigUint::zero());
    assert_eq!(machine.get_value("B"), BigUint::from(14u8));
    assert_eq!(machine.get_value("Y"), BigUint::zero());

    machine.inc("A");
    assert_eq!(machine.get_value("X"), BigUint::from(5u8));
    assert_eq!(machine.get_value("A"), BigUint::one());
    assert_eq!(machine.get_value("B"), BigUint::from(14u8));
    assert_eq!(machine.get_value("Y"), BigUint::zero());
}

#[test]
fn dec() {
    let mut machine = make_machine();
    machine.dec("X");
    assert_eq!(machine.get_value("X"), BigUint::from(3u8));
    assert_eq!(machine.get_value("A"), BigUint::zero());
    assert_eq!(machine.get_value("B"), BigUint::from(13u8));
    assert_eq!(machine.get_value("Y"), BigUint::zero());

    machine.dec("B");
    assert_eq!(machine.get_value("X"), BigUint::from(3u8));
    assert_eq!(machine.get_value("A"), BigUint::zero());
    assert_eq!(machine.get_value("B"), BigUint::from(12u8));
    assert_eq!(machine.get_value("Y"), BigUint::zero());

    machine.dec("A");
    assert_eq!(machine.get_value("X"), BigUint::from(3u8));
    assert_eq!(machine.get_value("A"), BigUint::zero());
    assert_eq!(machine.get_value("B"), BigUint::from(12u8));
    assert_eq!(machine.get_value("Y"), BigUint::zero());
}

#[test]
fn is_zero() {
    let mut machine = make_machine();
    assert!(!machine.is_zero("X").unwrap());
    assert!(machine.is_zero("A").unwrap());
    assert!(!machine.is_zero("B").unwrap());
    assert!(machine.is_zero("Y").unwrap());

    machine.inc("A");
    assert!(!machine.is_zero("X").unwrap());
    assert!(!machine.is_zero("A").unwrap());
    assert!(!machine.is_zero("B").unwrap());
    assert!(machine.is_zero("Y").unwrap());

    for _ in 0 .. 10 {
        machine.dec("X");
    }
    assert!(machine.is_zero("X").unwrap());
    assert!(!machine.is_zero("A").unwrap());
    assert!(!machine.is_zero("B").unwrap());
    assert!(machine.is_zero("Y").unwrap());
}

#[test]
fn cons_sum() {
    let mut machine = make_machine();
    machine.cons_sum("X", 1234567890);
    assert_eq!(machine.get_value("X"), BigUint::from(1234567894u128));
    assert_eq!(machine.get_value("A"), BigUint::zero());
    assert_eq!(machine.get_value("B"), BigUint::from(13u8));
    assert_eq!(machine.get_value("Y"), BigUint::zero());
}

#[test]
fn cons_sub() {
    let mut machine = make_machine();
    machine.cons_sub("X", 2);
    assert_eq!(machine.get_value("X"), BigUint::from(2u8));
    assert_eq!(machine.get_value("A"), BigUint::zero());
    assert_eq!(machine.get_value("B"), BigUint::from(13u8));
    assert_eq!(machine.get_value("Y"), BigUint::zero());

    machine.cons_sub("X", 1234567890);
    assert_eq!(machine.get_value("X"), BigUint::zero());
    assert_eq!(machine.get_value("A"), BigUint::zero());
    assert_eq!(machine.get_value("B"), BigUint::from(13u8));
    assert_eq!(machine.get_value("Y"), BigUint::zero());
}

#[test]
fn cons_cmp() {
    let mut machine = make_machine();
    assert!(machine.cons_cmp("X", 4).unwrap());
    assert!(machine.cons_cmp("A", 0).unwrap());
    assert!(machine.cons_cmp("B", 13).unwrap());
    assert!(machine.cons_cmp("Y", 0).unwrap());

    assert!(!machine.cons_cmp("X", 0).unwrap());
    assert!(!machine.cons_cmp("A", 1).unwrap());
    assert!(!machine.cons_cmp("B", 12).unwrap());
    assert!(!machine.cons_cmp("Y", 9).unwrap());
}
