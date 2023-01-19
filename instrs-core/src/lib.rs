#![feature(array_try_from_fn)]
#![feature(split_array)]

use num_traits::Unsigned;
use std::fmt::{Formatter, Error as FError};

#[derive(Debug)]
pub enum Error {
    ExpectedBytes(u32),
    ExpectedRange(std::ops::RangeInclusive<u32>),
}

pub trait Serialize: Sized {
    fn from_bytes(b: &mut &[u8]) -> Result<Self, Error>;
    fn into_bytes(&self, b: &mut Vec<u8>);
}

macro_rules! impl_byte_serialize {
    ($($t:ty),+) => {
        $(
            impl Serialize for $t {
                fn from_bytes(b: &mut &[u8]) -> Result<Self, Error> { const SIZE: usize = std::mem::size_of::<$t>();

                    if SIZE > b.len() {return Err(Error::ExpectedBytes((SIZE - b.len()) as u32))};

                    let (bytes, rem) = b.split_array_ref::<SIZE>();
                    *b = rem;

                    Ok(<$t>::from_le_bytes(*bytes))
                }

                fn into_bytes(&self, b: &mut Vec<u8>) {
                    let bytes = self.to_le_bytes();

                    b.extend_from_slice(&bytes);
                }
            }
        )+
    }
}

impl_byte_serialize!(u8,u16,u32,u64,u128,usize,
                     // Using signed numbers in bytecode is stupid but I still support it because I am a nice person
                     i8,i16,i32,i64,i128,isize,
                     f32,f64);

impl<B: Serialize, const N: usize> Serialize for [B; N] {
    fn from_bytes(b: &mut &[u8]) -> Result<Self, Error> {
        std::array::try_from_fn(|_| {
            <B>::from_bytes(b)
        })
    }

    fn into_bytes(&self, b: &mut Vec<u8>) {
        self.iter().for_each(|inner| {
            inner.into_bytes(b);
        });
    }
}

impl<B: Serialize> Serialize for Vec<B> {
    fn from_bytes(b: &mut &[u8]) -> Result<Self, Error> {
        let Some((len, rem)) = b.split_first() else {return Err(Error::ExpectedBytes(1))};
        *b = rem;

        let len = (*len).into();
        let mut v = Vec::with_capacity(len);

        for _ in 0..len {
            let next = <B>::from_bytes(b)?;
            v.push(next);
        }

        Ok(v)
    }

    fn into_bytes(&self, b: &mut Vec<u8>) {
        self.iter()
            .for_each(|inner| {
                inner.into_bytes(b);
            });
    }
}

impl<B: Serialize> Serialize for Box<B> {
    fn from_bytes(b: &mut &[u8]) -> Result<Self, Error> {
        Ok(Box::new(B::from_bytes(b)?))
    }

    fn into_bytes(&self, b: &mut Vec<u8>) {
        B::into_bytes(self, b);
    }
}

//TODO: Add Option<B>
