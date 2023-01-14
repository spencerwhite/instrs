use instrs_derive::*;
use instrs_core::*;

#[derive(Serialize)]
pub enum Instruction {
    Add {
        a: u8,
        b: u8,
        addr: u8,
    },
    Jump(u32),
    Halt,
}


