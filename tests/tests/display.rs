use tests::Instruction;
use instrs_core::*;

fn test_fmt(instruction: &Instruction, cmp: &'static str) {
    let mut s = String::new();
    instruction.into_string(&mut s);

    assert_eq!(s, cmp);
}

#[test]
fn unit() {
    let instruction = Instruction::Halt;

    test_fmt(&instruction, "Halt ");
}

#[test]
fn tuple() {
    let instruction = Instruction::Jump(64);
    test_fmt(&instruction, "Jump 64, ");
}

#[test]
fn r#struct() {
    let instruction = Instruction::Add { a: 1, b: 2, addr: 3 };
    test_fmt(&instruction, "Add 1,2,3, ")
}
