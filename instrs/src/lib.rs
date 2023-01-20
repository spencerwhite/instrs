//! # What is this?
//! A derive macro for (de)serializing your own enums into bytecode, mainly for the purpose of
//! building VMs
//!
//! # Why?
//! Converting enums with hundreds of variants into bytecode and back is not fun. This crate aims
//! to automate this task with a fairly opinionated layout that makes it easy to focus on your VM's
//! instructions rather than how it's laid out in memory. 
//!
//! # Usage
//! ```
//! // The instructions our VM can handle
//! pub enum Instruction {
//!     // Unit variants
//!     Nop,
//!
//!     // Tuple variants
//!     Jmp(usize),
//!
//!     // Struct variants
//!     Add {
//!         a: usize,
//!         b: usize,
//!         store_in: usize,
//!     },
//!
//!     //Supports most `std` types
//!     Etc(u8, i16, f32, usize, Option<char>, bool),
//!
//!     // Variable-size items
//!     PushString(String),
//!     PushMany(Vec<u32>),
//!
//!     // Arrays and tuples
//!     Foo( ([u8; 3], [u32; 4]) ),
//!
//!     // Rust type system allows for powerful composition
//!     DoFiveTimes(Box<Instruction>)
//! }
//! ````

pub use instrs_core::*;
pub use instrs_derive::*;
