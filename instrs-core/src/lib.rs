#![feature(array_try_from_fn)]
#![feature(split_array)]
#![feature(trace_macros)]

use num_traits::Unsigned;

#[derive(Debug)]
pub enum Error {
    ExpectedBytes(u32),
    ExpectedRange(std::ops::RangeInclusive<u32>),
    InvalidUtf8(std::string::FromUtf8Error),
    InvalidChar,
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

impl Serialize for char {
    fn from_bytes(b: &mut &[u8]) -> Result<Self, Error> {
        match char::from_u32(u32::from_bytes(b)?) {
            Some(c) => Ok(c),
            None => Err(Error::InvalidChar),
        }
    }

    fn into_bytes(&self, b: &mut Vec<u8>) {
        (*self as u32).into_bytes(b)
    }
}

impl Serialize for bool {
    fn from_bytes(b: &mut &[u8]) -> Result<Self, Error> {
        let Some((byte, rem)) = b.split_first() else {return Err(Error::ExpectedBytes(1))};
        *b = rem;

        match byte {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(Error::ExpectedRange(0..=1)),
        } 
    }

    fn into_bytes(&self, b: &mut Vec<u8>) {
        (*self as u8).into_bytes(b)
    }
}

macro_rules! impl_byte_serialize_tuple {
    ( $($i:ident $n:tt),* ) => {
        #[allow(unused_variables)]
        impl<$($i : Serialize),*> Serialize for ( $($i,)* ) {
            fn from_bytes(b: &mut &[u8]) -> Result<Self, Error> {
                Ok((
                    $($i::from_bytes(b)?,)*
                ))
            }

            fn into_bytes(&self, b: &mut Vec<u8> ) {
                $(self.$n.into_bytes(b);)*
            }
        }
    }
}

impl_byte_serialize_tuple!();
impl_byte_serialize_tuple!(A 0);
impl_byte_serialize_tuple!(A 0, B 1);
impl_byte_serialize_tuple!(A 0, B 1, C 2);
impl_byte_serialize_tuple!(A 0, B 1, C 2, D 3);
impl_byte_serialize_tuple!(A 0, B 1, C 2, D 3, E 4);
impl_byte_serialize_tuple!(A 0, B 1, C 2, D 3, E 4, F 5);
impl_byte_serialize_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6);
impl_byte_serialize_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7);
impl_byte_serialize_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, J 8);
impl_byte_serialize_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, J 8, K 9);
impl_byte_serialize_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, J 8, K 9, L 10);


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

impl<B: Serialize> Serialize for Box<[B]> {
    fn from_bytes(b: &mut &[u8]) -> Result<Self, Error> {
        Ok(<Vec<B>>::from_bytes(b)?.into_boxed_slice())
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

impl<B: Serialize> Serialize for Option<B> {
    fn from_bytes(b: &mut &[u8]) -> Result<Self, Error> {
        let is_some = bool::from_bytes(b)?;

        match is_some {
            false => Ok(None),
            true => Ok(Some(B::from_bytes(b)?)),
        }
    }

    fn into_bytes(&self, b: &mut Vec<u8>) {
        self.is_some().into_bytes(b);

        if let Some(inner) = self {
            inner.into_bytes(b);
        }
    }
}

impl Serialize for String {
    fn from_bytes(b: &mut &[u8]) -> Result<Self, Error> {
        match std::string::String::from_utf8(<Vec<u8>>::from_bytes(b)?) {
            Ok(s) => Ok(s),
            Err(e) => Err(Error::InvalidUtf8(e)),
        }
    }

    fn into_bytes(&self, b: &mut Vec<u8>) {
        b.push(self.len() as u8);
        b.extend_from_slice(self.as_bytes())
    }
}

impl Serialize for &str {
    fn from_bytes(_b: &mut &[u8]) -> Result<Self, Error> {
        panic!("Can't convert bytes to &'str. Try converting to an owned `String` instead")
    }

    fn into_bytes(&self, b: &mut Vec<u8>) {
        b.push(self.len() as u8);
        b.extend_from_slice(self.as_bytes())
    }
}
