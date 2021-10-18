use num_bigint::BigUint;

#[test]
fn modtuple_input_3_2() {
    assert_eq!(
        norma::run(SOURCE, BigUint::from(2u64.pow(3) * 3u64.pow(2))).unwrap(),
        BigUint::from(1u8)
    );
}

#[test]
fn modtuple_input_2_3() {
    assert_eq!(
        norma::run(SOURCE, BigUint::from(2u64.pow(2) * 3u64.pow(3))).unwrap(),
        BigUint::from(2u8)
    );
}

#[test]
fn modtuple_input_4_2() {
    assert_eq!(
        norma::run(SOURCE, BigUint::from(2u64.pow(4) * 3u64.pow(2))).unwrap(),
        BigUint::from(0u8)
    );
}

#[test]
fn modtuple_input_5_2() {
    assert_eq!(
        norma::run(SOURCE, BigUint::from(2u64.pow(5) * 3u64.pow(2))).unwrap(),
        BigUint::from(1u8)
    );
}

#[test]
fn modtuple_input_5_3() {
    assert_eq!(
        norma::run(SOURCE, BigUint::from(2u64.pow(5) * 3u64.pow(3))).unwrap(),
        BigUint::from(2u8)
    );
}

#[test]
fn modtuple_input_7_3() {
    assert_eq!(
        norma::run(SOURCE, BigUint::from(2u64.pow(7) * 3u64.pow(3))).unwrap(),
        BigUint::from(1u8)
    );
}

const SOURCE: &str = "// input A
// output A := 0
operation clear (A) {
 1: if zero A then goto 3 else goto 2
 2: do dec A goto 1
}

// input A
// input B
// output A := A + B
// output B := 0
operation restore (A, B) {
 1: if zero B then goto 0 else goto 2
 2: do inc A goto 3
 3: do dec B goto 1
}

// input A
// input B
// input C
// output A := A + C
// output B := B + C
// output C := 0
operation restore2 (A, B, C) {
 1: if zero C then goto 0 else goto 2
 2: do inc A goto 3
 3: do inc B goto 4
 4: do dec C goto 1
}

// input B
// output A := B
// temp C
operation copy (A, B, C) {
 1: do clear (A) goto 2
 2: do clear (C) goto 3
 3: do restore2 (A, C, B) goto 4
 4: do restore (B, C) goto 0
}

// input A
// input B
// temp C
// test A < B
test lessThan (A, B, C) {
 1: do clear (C) goto 2
 2: if zero B then goto 3 else goto 4
 3: do restore2 (A, B, C) goto false
 4: if zero A then goto 5 else goto 6
 5: do restore2 (A, B, C) goto true
 6: do dec A goto 7
 7: do dec B goto 8
 8: do inc C goto 2
}

// input A
// input B
// output A := A - B
// temp C
operation sub (A, B, C) {
 1: do clear (C) goto 2
 2: if zero B then goto 3 else goto 4
 3: do restore (B, C) goto 0
 4: do dec B goto 5
 5: do dec A goto 6
 6: do inc C goto 2
}

// input B
// input C
// output A := B / C
// output B := B % C
// temp D
operation div (A, B, C, D) {
 1: do clear (A) goto 2
 2: if lessThan (B, C, D) then goto 0 else goto 3
 3: do sub (B, C, D) goto 4
 4: do inc A goto 2
}

// input B (tuple)
// input C (prime number)
// output A := B[C]
// temp D
// temp E
// temp F
operation readField (A, B, C, D, E, F) {
 1: do clear (A) goto 2
 2: do copy (D, B, F) goto 3
 3: do div (E, D, C, F) goto 4
 4: if zero D then goto 5 else goto 0
 5: do copy (D, E, F) goto 6
 6: do inc A goto 3
}

main {
 1: do inc A goto 2
 2: do inc A goto 3
 3: do readField (Y, X, A, B, C, D) goto 4
 4: do inc A goto 5
 5: do readField (B, X, A, C, D, E) goto 6
 6: do div (C, Y, B, D) goto 0
}";
