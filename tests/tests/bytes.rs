use tests::Instruction;

use instrs_core::*;

#[test]
fn into_string() {
    fn test_into_string<T: Serialize>(t: T, expected: &'static str) {
        let mut s = String::new();
        t.into_string(&mut s);
        assert_eq!(s, expected);
    }

    test_into_string(3usize, "3,");
    test_into_string([3usize; 3], "3,3,3,");
    test_into_string(vec![1usize,2,3], "3,1,2,3,");
}

#[test]
fn from_string() {
    fn test_from_string<T: Serialize + std::fmt::Debug + Eq>(s: &mut &str, expected: T) {
        assert_eq!(expected, T::from_string(s).unwrap());
    }

    test_from_string(&mut "3,", 3usize);
    test_from_string(&mut "3,3,3,", [3usize;3]);
    test_from_string(&mut "3,1,2,3,", vec![1usize,2,3]);
}

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
