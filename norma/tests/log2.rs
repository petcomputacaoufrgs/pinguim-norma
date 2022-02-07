use num_bigint::BigUint;

#[test]
fn log2_input0() {
    assert_eq!(
        norma::run(SOURCE, BigUint::from(0u8)).unwrap(),
        BigUint::from(0u8)
    );
}

#[test]
fn log2_input1() {
    assert_eq!(
        norma::run(SOURCE, BigUint::from(1u8)).unwrap(),
        BigUint::from(0u8)
    );
}

#[test]
fn log2_input2() {
    assert_eq!(
        norma::run(SOURCE, BigUint::from(2u8)).unwrap(),
        BigUint::from(1u8)
    );
}

#[test]
fn log2_input3() {
    assert_eq!(
        norma::run(SOURCE, BigUint::from(3u8)).unwrap(),
        BigUint::from(1u8)
    );
}

#[test]
fn log2_input4() {
    assert_eq!(
        norma::run(SOURCE, BigUint::from(4u8)).unwrap(),
        BigUint::from(2u8)
    );
}

#[test]
fn log2_input8() {
    assert_eq!(
        norma::run(SOURCE, BigUint::from(8u8)).unwrap(),
        BigUint::from(3u8)
    );
}

#[test]
fn log2_input11() {
    assert_eq!(
        norma::run(SOURCE, BigUint::from(11u8)).unwrap(),
        BigUint::from(3u8)
    );
}

#[test]
fn log2_input15() {
    assert_eq!(
        norma::run(SOURCE, BigUint::from(15u8)).unwrap(),
        BigUint::from(3u8)
    );
}

#[test]
fn log2_input29() {
    assert_eq!(
        norma::run(SOURCE, BigUint::from(29u8)).unwrap(),
        BigUint::from(4u8)
    );
}

#[test]
fn log2_input345() {
    assert_eq!(
        norma::run(SOURCE, BigUint::from(345u16)).unwrap(),
        BigUint::from(8u8)
    );
}

const SOURCE: &str = "operation clear (R) {
    1: if zero R then goto 0 else goto 2
    2: do dec R goto 1
}

operation restore (Out, Temp) {
    1: if zero Temp then goto 0 else goto 2
    2: do dec Temp goto 3
    3: do inc Out goto 1
}

operation copy (Out, In, Temp) {
    1: if zero In then goto 5 else goto 2
    2: do dec In goto 3
    3: do inc Temp goto 4
    4: do inc Out goto 1
    5: do restore (In, Temp) goto 0
}

operation div2 (Quot, In, Temp) {
    1: do clear (Quot) goto 2
    2: do clear (Temp) goto 3
    3: if zero In then goto 10 else goto 4
    4: do dec In goto 5
    5: do inc Temp goto 6
    6: if zero In then goto 10 else goto 7
    7: do dec In goto 8
    8: do inc Temp goto 9
    9: do inc Quot goto 3
    10: do restore (In, Temp) goto 0
}

operation log2 (Out, In, TempI, TempQ, Temp) {
    1: do clear (Out) goto 2
    2: do clear (TempI) goto 3
    3: do copy (TempI, In, Temp) goto 4
    4: do div2 (TempQ, TempI, Temp) goto 5
    5: if zero TempQ then goto 0 else goto 6
    6: do inc Out goto 7
    7: do clear (TempI) goto 8
    8: do copy (TempI, TempQ, Temp) goto 4
}

main {
    1: do log2 (Y, X, A, B, C) goto 0
}";
