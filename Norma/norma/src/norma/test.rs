use super::Machine;
use num_bigint::BigUint;
use num_traits::identities::{One, Zero};

#[test]
fn insert() {
    let mut machine = Machine::new(BigUint::from(4u8));
    assert_eq!(
        machine.get_register("X").unwrap().get_value(),
        BigUint::from(4u8)
    );

    machine.insert("A");
    assert_eq!(machine.get_register("A").unwrap().get_value(), BigUint::zero());
    assert_eq!(
        machine.get_register("X").unwrap().get_value(),
        BigUint::from(4u8)
    );

    machine.insert_with_value("B", BigUint::from(13u8));
    assert_eq!(
        machine.get_register("B").unwrap().get_value(),
        BigUint::from(13u8)
    );
    assert_eq!(machine.get_register("A").unwrap().get_value(), BigUint::zero());
    assert_eq!(
        machine.get_register("X").unwrap().get_value(),
        BigUint::from(4u8)
    );
}

fn make_machine() -> Machine {
    let mut machine = Machine::new(BigUint::from(4u8));
    machine.insert("A");
    machine.insert_with_value("B", BigUint::from(13u8));
    machine.insert("Y");
    machine
}

#[test]
fn inc() {
    let mut machine = make_machine();
    machine.inc("X");
    assert_eq!(
        machine.get_register("X").unwrap().get_value(),
        BigUint::from(5u8)
    );
    assert_eq!(machine.get_register("A").unwrap().get_value(), BigUint::zero());
    assert_eq!(
        machine.get_register("B").unwrap().get_value(),
        BigUint::from(13u8)
    );
    assert_eq!(machine.get_register("Y").unwrap().get_value(), BigUint::zero());

    machine.inc("B");
    assert_eq!(
        machine.get_register("X").unwrap().get_value(),
        BigUint::from(5u8)
    );
    assert_eq!(machine.get_register("A").unwrap().get_value(), BigUint::zero());
    assert_eq!(
        machine.get_register("B").unwrap().get_value(),
        BigUint::from(14u8)
    );
    assert_eq!(machine.get_register("Y").unwrap().get_value(), BigUint::zero());

    machine.inc("A");
    assert_eq!(
        machine.get_register("X").unwrap().get_value(),
        BigUint::from(5u8)
    );
    assert_eq!(machine.get_register("A").unwrap().get_value(), BigUint::one());
    assert_eq!(
        machine.get_register("B").unwrap().get_value(),
        BigUint::from(14u8)
    );
    assert_eq!(machine.get_register("Y").unwrap().get_value(), BigUint::zero());
}

#[test]
fn dec() {
    let mut machine = make_machine();
    machine.dec("X");
    assert_eq!(
        machine.get_register("X").unwrap().get_value(),
        BigUint::from(3u8)
    );
    assert_eq!(machine.get_register("A").unwrap().get_value(), BigUint::zero());
    assert_eq!(
        machine.get_register("B").unwrap().get_value(),
        BigUint::from(13u8)
    );
    assert_eq!(machine.get_register("Y").unwrap().get_value(), BigUint::zero());

    machine.dec("B");
    assert_eq!(
        machine.get_register("X").unwrap().get_value(),
        BigUint::from(3u8)
    );
    assert_eq!(machine.get_register("A").unwrap().get_value(), BigUint::zero());
    assert_eq!(
        machine.get_register("B").unwrap().get_value(),
        BigUint::from(12u8)
    );
    assert_eq!(machine.get_register("Y").unwrap().get_value(), BigUint::zero());

    machine.dec("A");
    assert_eq!(
        machine.get_register("X").unwrap().get_value(),
        BigUint::from(3u8)
    );
    assert_eq!(machine.get_register("A").unwrap().get_value(), BigUint::zero());
    assert_eq!(
        machine.get_register("B").unwrap().get_value(),
        BigUint::from(12u8)
    );
    assert_eq!(machine.get_register("Y").unwrap().get_value(), BigUint::zero());
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
