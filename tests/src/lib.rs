use instrs_derive::*;
use instrs_core::*;

#[derive(Serialize)]
pub enum Instruction {
    Add {
        a: usize,
        b: usize,
        addr: usize,
    },
    Jump(usize),
    Halt,
}


