#![feature(array_try_from_fn)]

use num_traits::Unsigned;
use std::{fmt::Display, str::FromStr};

pub enum Error {
    Expected,
}

/// A trait for converting instructions to and from a string. The FromStr implementation should
/// **not** provide a complex lexer and parser, but instead a very simple, strict parser for
/// debugging purposes.
///
/// The result of the Display impl should provide a valid input for the FromStr impl and vice versa
pub trait Serialize: Display + FromStr {}
impl<T> Serialize for T where T: Display + FromStr {}


pub trait ByteSerialize: Sized {
    type Size: Unsigned;

    fn from_bytes(b: &mut &[Self::Size]) -> Result<Self, Error>;
    fn into_bytes(self) -> Box<[Self::Size]>;
}

macro_rules! impl_byte_serialize {
    ($($t:ty),+) => {
        $(
            impl ByteSerialize for $t {
                type Size = Self;

                fn from_bytes(b: &mut &[Self::Size]) -> Result<Self, Error> {
                    let Some(&byte) = b.get(0) else {return Err(Error::Expected)};
                    *b = &b[1..];

                    Ok(byte)
                }
                fn into_bytes(self) -> Box<[Self::Size]> {
                    [self].into()
                }
            }
        )+
    }
}

impl_byte_serialize!(u8,u16,u32,u64,u128,usize);

impl<B: ByteSerialize, const N: usize> ByteSerialize for [B; N] {
    type Size = B::Size;

    fn from_bytes(b: &mut &[Self::Size]) -> Result<Self, Error> {
        let arr = std::array::try_from_fn(|_| {
            let Some(mut bytes) = b.get(0..N) else {return Err(Error::Expected)};
            *b = &b[N..];

            B::from_bytes(&mut bytes)
        })?;

        Ok(arr.try_into().unwrap())
    }
    //TODO: Make this more efficient, mayhaps
    fn into_bytes(self) -> Box<[Self::Size]> {
        self.map(|inner| inner.into_bytes().into_vec())
            .into_iter()
            .flatten()
            .collect::<Vec<_>>()
            .into()
    }
}

impl<B: Unsigned + Copy> ByteSerialize for Vec<B> {
    type Size = B;

    fn from_bytes(b: &mut &[Self::Size]) -> Result<Self, Error> {
        let mut v = Vec::new();
        for byte in b.iter() {
            v.push(*byte)
        }

        Ok(v)
    }
    fn into_bytes(self) -> Box<[Self::Size]> {
        self.into()
    }
}
