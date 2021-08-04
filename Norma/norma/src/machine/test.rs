use super::Machine;
use num_bigint::BigUint;
use num_traits::identities::{One, Zero};
use std::cmp::Ordering;

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
    let machine = make_machine();
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
    assert!(!machine.is_zero("X"));
    assert!(machine.is_zero("A"));
    assert!(!machine.is_zero("B"));
    assert!(machine.is_zero("Y"));

    machine.inc("A");
    assert!(!machine.is_zero("X"));
    assert!(!machine.is_zero("A"));
    assert!(!machine.is_zero("B"));
    assert!(machine.is_zero("Y"));

    machine.inc("B");
    assert!(!machine.is_zero("X"));
    assert!(!machine.is_zero("A"));
    assert!(!machine.is_zero("B"));
    assert!(machine.is_zero("Y"));

    machine.dec("Y");
    assert!(!machine.is_zero("X"));
    assert!(!machine.is_zero("A"));
    assert!(!machine.is_zero("B"));
    assert!(machine.is_zero("Y"));

    for _ in 0 .. 10 {
        machine.dec("X");
    }
    assert!(machine.is_zero("X"));
    assert!(!machine.is_zero("A"));
    assert!(!machine.is_zero("B"));
    assert!(machine.is_zero("Y"));
}

#[test]
fn add_const() {
    let mut machine = make_machine();
    machine.add_const("X", &BigUint::from(1234567890u64));
    assert_eq!(machine.get_value("X"), BigUint::from(1234567894u128));
    assert_eq!(machine.get_value("A"), BigUint::zero());
    assert_eq!(machine.get_value("B"), BigUint::from(13u8));
    assert_eq!(machine.get_value("Y"), BigUint::zero());

    machine.add_const("Y", &BigUint::from(2u64));
    assert_eq!(machine.get_value("X"), BigUint::from(1234567894u128));
    assert_eq!(machine.get_value("A"), BigUint::zero());
    assert_eq!(machine.get_value("B"), BigUint::from(13u8));
    assert_eq!(machine.get_value("Y"), BigUint::from(2u8));

    machine.add_const("A", &BigUint::from(0u64));
    assert_eq!(machine.get_value("X"), BigUint::from(1234567894u128));
    assert_eq!(machine.get_value("A"), BigUint::zero());
    assert_eq!(machine.get_value("B"), BigUint::from(13u8));
    assert_eq!(machine.get_value("Y"), BigUint::from(2u8));

    machine.add_const("B", &BigUint::from(0u64));
    assert_eq!(machine.get_value("X"), BigUint::from(1234567894u128));
    assert_eq!(machine.get_value("A"), BigUint::zero());
    assert_eq!(machine.get_value("B"), BigUint::from(13u8));
    assert_eq!(machine.get_value("Y"), BigUint::from(2u8));
}

#[test]
fn sub_const() {
    let mut machine = make_machine();
    machine.sub_const("X", &BigUint::from(2u64));
    assert_eq!(machine.get_value("X"), BigUint::from(2u8));
    assert_eq!(machine.get_value("A"), BigUint::zero());
    assert_eq!(machine.get_value("B"), BigUint::from(13u8));
    assert_eq!(machine.get_value("Y"), BigUint::zero());

    machine.sub_const("X", &BigUint::from(1234567890u64));
    assert_eq!(machine.get_value("X"), BigUint::zero());
    assert_eq!(machine.get_value("A"), BigUint::zero());
    assert_eq!(machine.get_value("B"), BigUint::from(13u8));
    assert_eq!(machine.get_value("Y"), BigUint::zero());

    machine.sub_const("A", &BigUint::from(1u64));
    assert_eq!(machine.get_value("X"), BigUint::zero());
    assert_eq!(machine.get_value("A"), BigUint::zero());
    assert_eq!(machine.get_value("B"), BigUint::from(13u8));
    assert_eq!(machine.get_value("Y"), BigUint::zero());

    machine.sub_const("A", &BigUint::from(0u64));
    assert_eq!(machine.get_value("X"), BigUint::zero());
    assert_eq!(machine.get_value("A"), BigUint::zero());
    assert_eq!(machine.get_value("B"), BigUint::from(13u8));
    assert_eq!(machine.get_value("Y"), BigUint::zero());

    machine.sub_const("B", &BigUint::from(0u64));
    assert_eq!(machine.get_value("X"), BigUint::zero());
    assert_eq!(machine.get_value("A"), BigUint::zero());
    assert_eq!(machine.get_value("B"), BigUint::from(13u8));
    assert_eq!(machine.get_value("Y"), BigUint::zero());
}

#[test]
fn eq_const() {
    let mut machine = make_machine();
    assert!(machine.eq_const("X", &BigUint::from(4u64)));
    assert!(machine.eq_const("A", &BigUint::from(0u64)));
    assert!(machine.eq_const("B", &BigUint::from(13u64)));
    assert!(machine.eq_const("Y", &BigUint::from(0u64)));

    assert!(!machine.eq_const("X", &BigUint::from(0u64)));
    assert!(!machine.eq_const("A", &BigUint::from(1u64)));
    assert!(!machine.eq_const("B", &BigUint::from(12u64)));
    assert!(!machine.eq_const("Y", &BigUint::from(9u64)));
}

#[test]
fn counter() {
    let mut machine = make_machine();
    assert_eq!(machine.get_counted_steps(), BigUint::zero());
    machine.inc("X");
    assert_eq!(machine.get_counted_steps(), BigUint::one());
    machine.dec("X");
    assert_eq!(machine.get_counted_steps(), BigUint::from(2u8));
    machine.inc("Y");
    assert_eq!(machine.get_counted_steps(), BigUint::from(3u8));

    machine.inc("A");
    machine.inc("B");
    machine.inc("X");
    machine.dec("A");
    machine.inc("X");
    assert_eq!(machine.get_counted_steps(), BigUint::from(8u8));

    machine.add_const("A", &BigUint::from(9u64));
    assert_eq!(machine.get_counted_steps(), BigUint::from(17u8));

    machine.sub_const("B", &BigUint::from(10u64));
    assert_eq!(machine.get_counted_steps(), BigUint::from(27u8));
}
