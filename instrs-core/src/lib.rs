#![feature(array_try_from_fn)]

use num_traits::Unsigned;
use std::fmt::{Formatter, Error as FError};

#[derive(Debug)]
pub enum Error {
    Expected,
    Parse,
}

pub trait Serialize: Sized {
    type Size: Unsigned;

    fn from_bytes(b: &mut &[Self::Size]) -> Result<Self, Error>;
    fn into_bytes(&self, b: &mut Vec<Self::Size>);

    ///Essentially the FromStr trait, but this allows us to define it on std library types like
    ///[usize; N]
    fn from_string(s: &mut &str) -> Result<Self, Error>;
    ///Essentially the Display trait, but this allows us to define it on std library types like
    ///[usize; N]
    fn into_string(&self, f: &mut String);
}

macro_rules! impl_byte_serialize {
    ($($t:ty),+) => {
        $(
            impl Serialize for $t {
                type Size = Self;

                fn from_bytes(b: &mut &[Self::Size]) -> Result<Self, Error> {
                    let Some((byte, rem)) = b.split_first() else {return Err(Error::Expected)};
                    *b = rem;

                    Ok(*byte)
                }

                fn into_bytes(&self, b: &mut Vec<Self::Size>) {
                    b.push(*self)
                }

                fn from_string(s: &mut &str) -> Result<Self, Error> {
                    let Some((num, rem)) = s.split_once(',') else {return Err(Error::Expected)};
                    *s = rem;

                    num.parse().map_err(|_| Error::Parse)
                }

                fn into_string(&self, f: &mut String) {
                    let fmt = format!("{},", self);
                    f.push_str(&fmt);
                }
            }
        )+
    }
}

impl_byte_serialize!(u8,u16,u32,u64,u128,usize);

impl<B: Serialize, const N: usize> Serialize for [B; N] {
    type Size = B::Size;

    fn from_bytes(b: &mut &[Self::Size]) -> Result<Self, Error> {
        std::array::try_from_fn(|_| {
            <B>::from_bytes(b)
        })
    }

    fn into_bytes(&self, b: &mut Vec<Self::Size>) {
        self.iter().for_each(|inner| {
            inner.into_bytes(b);
        });
    }

    fn from_string(s: &mut &str) -> Result<Self, Error> {
        std::array::try_from_fn(|_| {
            <B>::from_string(s)
        })
    }

    fn into_string(&self, f: &mut String) {
        self.iter()
            .for_each(|b| b.into_string(f))
    }
}

impl<B: Serialize> Serialize for Vec<B> where
    B::Size: Into<usize> + Copy,
{
    type Size = B::Size;

    fn from_bytes(b: &mut &[Self::Size]) -> Result<Self, Error> {
        let Some((len, rem)) = b.split_first() else {return Err(Error::Expected)};
        *b = rem;

        let len = (*len).into();
        let mut v = Vec::with_capacity(len);

        for _ in 0..len {
            let next = <B>::from_bytes(b)?;
            v.push(next);
        }

        Ok(v)
    }

    fn into_bytes(&self, b: &mut Vec<Self::Size>) {
        self.iter()
            .for_each(|inner| {
                inner.into_bytes(b);
            });
    }

    fn from_string(s: &mut &str) -> Result<Self, Error> {
        let Some((len, rem)) = s.split_once(',') else {return Err(Error::Expected)};
        *s = rem;

        let Ok(len) = len.parse::<usize>() else {return Err(Error::Parse)};
        let mut v = Vec::with_capacity(len);

        for _ in 0..len {
            let next = <B>::from_string(s)?;
            v.push(next);
        }

        Ok(v)
    }
    fn into_string(&self, f: &mut String) {
        let fmt = format!("{},", self.len());
        f.push_str(&fmt);

        self.into_iter()
            .for_each(|b| b.into_string(f));
    }
}
