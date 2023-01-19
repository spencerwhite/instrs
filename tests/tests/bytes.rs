use tests::Instruction;

use instrs::*;

#[test]
fn into_bytes() {
    fn test_into_bytes<T: Serialize>(t: T, expected: &[u8]) {
        let mut v = Vec::new();
        T::into_bytes(&t, &mut v);
        assert_eq!(v, expected);
    }

    test_into_bytes(Instruction::Add {a: 1, b: 2, addr: 3}, &[0, 1, 2, 3]);
    test_into_bytes(Instruction::Jump (u32::MAX), &[1, u8::MAX, u8::MAX, u8::MAX, u8::MAX]);
    test_into_bytes(Instruction::Halt, &[2]);
}

#[test]
fn from_bytes() {
    fn test_from_bytes<T: Serialize + std::fmt::Debug + PartialEq>(mut t: &[u8], expected: T) {
        let value = T::from_bytes(&mut t).expect(&format!("You mucked up the test for {:?}", expected));
        assert_eq!(value, expected);
    }

    test_from_bytes(&[32], 32u8);
    test_from_bytes(&[255,255,255,255], u32::MAX);
    test_from_bytes(&[2,1,2,], vec![1u8, 2]);
}
