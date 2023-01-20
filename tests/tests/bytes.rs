use tests::Instruction;

use instrs::*;

fn test_into_bytes<T: Serialize>(t: T, expected: &[u8]) {
    let mut v = Vec::new();
    T::into_bytes(&t, &mut v);
    assert_eq!(v, expected);
}

fn test_from_bytes<T: Serialize + std::fmt::Debug + PartialEq>(mut t: &[u8], expected: T) {
    let value = T::from_bytes(&mut t).expect(&format!("You mucked up the test for {:?}", expected));
    assert_eq!(value, expected);
}

#[test]
fn number() {
    test_into_bytes(1u32, &[1,0,0,0]); //Big Endian!
    test_into_bytes(-1i32, &[255,255,255,255]);

    test_from_bytes(&[255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255], u128::MAX);
}

#[test]
fn bool() {
    test_into_bytes(true, &[1]);
    test_from_bytes(&[0], false);
}

#[test]
fn char() {
    test_into_bytes('A', &[65,0,0,0]);
    test_from_bytes(&[5, 39, 0, 0], '✅');
}

#[test]
fn tuple() {
    test_into_bytes((), &[]);
    test_from_bytes(&[1, 2, 3, 4, 5], (1u8, 2u8, 3u8, 4u8, 5u8));
}

#[test]
fn array() {
    test_into_bytes(['A', 'B', 'C'], &[65,0,0,0,66,0,0,0,67,0,0,0]);
    test_from_bytes(&[65,0,0,0,66,0,0,0,67,0,0,0], ['A', 'B', 'C']);
}
#[test]
fn string() {
    test_into_bytes(String::from("ABC"), &[3,65,66,67]);
    test_from_bytes(&[3,65,66,67], String::from("ABC"));
}

#[test]
fn into_bytes() {
    test_into_bytes(Instruction::Add {a: 1, b: 2, addr: 3}, &[0, 1, 2, 3]);
    test_into_bytes(Instruction::Jump (u32::MAX), &[1, u8::MAX, u8::MAX, u8::MAX, u8::MAX]);
    test_into_bytes(Instruction::Halt, &[2]);
}

#[test]
fn from_bytes() {
    test_from_bytes(&[32], 32u8);
    test_from_bytes(&[255,255,255,255], u32::MAX);
    test_from_bytes(&[2,1,2,], vec![1u8, 2]);
}
