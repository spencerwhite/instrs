#![feature(array_try_from_fn)]
#![feature(split_array)]
#![feature(trait_alias)]

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    /// The bytecode provided is too short. This should only ever happen if the bytecode is
    /// outdated, or something went wrong when generating it.
    #[error("Expected {0} bytes")]
    ExpectedBytes(u32),
    /// Expected a value in a given range, e.g., bools must be between 0 and 1.
    #[error("Expected a value in the range {0:?}")]
    ExpectedRange(std::ops::RangeInclusive<u32>),
    /// Parsing a string resulted in invalid utf8. 
    #[error("Attempted to parse string with invalid utf8: {0:?}")]
    InvalidUtf8(std::string::FromUtf8Error),
    /// See [Size](crate::Size).
    ///
    /// This error is generated when [into_bytes](crate::Serialize::into_bytes) is called 
    /// on a variable-length struct with >= 2^`S` items 
    #[error("{needed_bytes} bytes are needed to store this item's length, but only {max_bytes} bytes are available. Try increasing `S`.")]
    TooLarge {
        needed_bytes: u32,
        max_bytes: u32,
    },
    /// Encountered an invalid utf8 sequence
    #[error("Encountered an invalid utf8 sequence")]
    InvalidChar,
}

/// Represents a type that can be used to encode the size of a variable-length struct like
/// [Vec] and [String]. For example, a `u8` will be cheap and efficient, but
/// every vec in the bytecode can only be a maximum of 256 items long. 
///
/// ```rust
/// assert_eq!(
///     <Vec::<u8>>::from_bytes::<u8>(&mut [3,1,2,3].as_slice()), 
///     Ok(vec![1,2,3])
/// );
/// assert_eq!(
///     <Vec::<u8>>::from_bytes::<u16>(&mut [3,0, 1,2,3].as_slice()), 
///     Ok(vec![1,2,3])
/// );
/// ```
///
/// *Note*: You should not use a `usize` since this makes your bytecode platform-dependent.
pub trait Size = Serialize + TryFrom<usize> + Into<usize>;

/// An item that can be Serialized into bytes. 
///
/// # Examples
/// ```
/// // `S` parameter is needed in all cases
/// assert_eq!(u32::from_bytes::<u8>(&mut [3,0,0,0].as_slice(), Ok(3u32)));
///
/// // Passing more bytes than needed is not an error
/// assert_eq!(bool::from_bytes::<u8>(&mut [1,0,0,0,0].as_slice()), Ok(true));
///
/// assert_eq!([char; 4]::from_bytes::<u8>(&mut [65,0,0,0, 66,0,0,0, 67,0,0,0, 68,0,0,0].as_slice()), ['A', 'B', 'C', 'D']);
/// assert_eq!(String::from_bytes::<u8>(&mut [4, 65,66,67,68].as_slice()), String::from("ABCD"));
/// ```
pub trait Serialize: Sized {
    fn from_bytes<S: Size>(b: &mut &[u8]) -> Result<Self, Error>;
    fn into_bytes<S: Size>(&self, b: &mut Vec<u8>) -> Result<(), Error>;
}

macro_rules! impl_byte_serialize {
    ($($t:ty),+) => {
        $(
            impl Serialize for $t {
                fn from_bytes<S: Size>(b: &mut &[u8]) -> Result<Self, Error> { const SIZE: usize = std::mem::size_of::<$t>();

                    if SIZE > b.len() {return Err(Error::ExpectedBytes((SIZE - b.len()) as u32))};

                    let (bytes, rem) = b.split_array_ref::<SIZE>();
                    *b = rem;

                    Ok(<$t>::from_le_bytes(*bytes))
                }

                fn into_bytes<S: Size>(&self, b: &mut Vec<u8>) -> Result<(), Error> {
                    let bytes = self.to_le_bytes();

                    b.extend_from_slice(&bytes);

                    Ok(())
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
    fn from_bytes<S: Size>(b: &mut &[u8]) -> Result<Self, Error> {
        match char::from_u32(u32::from_bytes::<S>(b)?) {
            Some(c) => Ok(c),
            None => Err(Error::InvalidChar),
        }
    }

    fn into_bytes<S: Size>(&self, b: &mut Vec<u8>) -> Result<(), Error> {
        (*self as u32).into_bytes::<S>(b)
    }
}

impl Serialize for bool {
    fn from_bytes<S: Size>(b: &mut &[u8]) -> Result<Self, Error> {
        let Some((byte, rem)) = b.split_first() else {return Err(Error::ExpectedBytes(1))};
        *b = rem;

        match byte {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(Error::ExpectedRange(0..=1)),
        } 
    }

    fn into_bytes<S: Size>(&self, b: &mut Vec<u8>) -> Result<(), Error> {
        (*self as u8).into_bytes::<S>(b)
    }
}

macro_rules! impl_byte_serialize_tuple {
    ( $($i:ident $n:tt),* ) => {
        #[allow(unused_variables)]
        impl<$($i : Serialize),*> Serialize for ( $($i,)* ) {
            fn from_bytes<S: Size>(b: &mut &[u8]) -> Result<Self, Error> {
                Ok((
                    $($i::from_bytes::<S>(b)?,)*
                ))
            }

            fn into_bytes<S: Size>(&self, b: &mut Vec<u8>) -> Result<(), Error> {
                $(self.$n.into_bytes::<S>(b)?;)*
                Ok(())
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
    fn from_bytes<S: Size>(b: &mut &[u8]) -> Result<Self, Error> {
        std::array::try_from_fn(|_| {
            <B>::from_bytes::<S>(b)
        })
    }

    fn into_bytes<S: Size>(&self, b: &mut Vec<u8>) -> Result<(), Error> {
        self.iter().try_for_each(|inner| {
            inner.into_bytes::<S>(b)
        })
    }
}

fn push_len<S: Size>(len: usize, b: &mut Vec<u8>) -> Result<(), Error> {
    //`len` is bigger than can be stored in `S`. This is much more likely than the error in
    //`from_bytes` since many implementations may choose an `S` of u8 to prioritize speed, but run
    //out of space at 256 items.

    let len = match len.try_into() {
        Ok(len) => len,
        Err(_) => return Err(Error::TooLarge {
            needed_bytes: len.ilog2(),
            max_bytes: std::mem::size_of::<S>() as u32,
        }),
    };

    S::into_bytes::<S>(&len, b)
}

impl<B: Serialize> Serialize for Vec<B> {
    fn from_bytes<S: Size>(b: &mut &[u8]) -> Result<Self, Error> {
        //The length of the vec will be too large to store on this machine's memory.
        //In reality this should never happen since, if S > usize, the machine will fail to load
        //the bytecode into memory.

        let len = match S::from_bytes::<S>(b)?.try_into() {
            Ok(len) => len,
            Err(_) => return Err(Error::TooLarge {
                needed_bytes: std::mem::size_of::<S>() as u32,
                max_bytes: std::mem::size_of::<usize>() as u32,
            })
        };

        let mut v = Vec::with_capacity(len);

        for _ in 0..len {
            let next = <B>::from_bytes::<S>(b)?;
            v.push(next);
        }

        Ok(v)
    }

    fn into_bytes<S: Size>(&self, b: &mut Vec<u8>) -> Result<(), Error> {
        push_len::<S>(self.len(), b)?;

        self.iter()
            .try_for_each(|inner| {
                inner.into_bytes::<S>(b)
            })
    }
}

impl<B: Serialize> Serialize for Box<[B]> {
    fn from_bytes<S: Size>(b: &mut &[u8]) -> Result<Self, Error> {
        Ok(<Vec<B>>::from_bytes::<S>(b)?.into_boxed_slice())
    }

    fn into_bytes<S: Size>(&self, b: &mut Vec<u8>) -> Result<(), Error> {
        self.iter()
            .try_for_each(|inner| {
                inner.into_bytes::<S>(b)
            })
    }
}

impl<B: Serialize> Serialize for Box<B> {
    fn from_bytes<S: Size>(b: &mut &[u8]) -> Result<Self, Error> {
        Ok(Box::new(B::from_bytes::<S>(b)?))
    }

    fn into_bytes<S: Size>(&self, b: &mut Vec<u8>) -> Result<(), Error> {
        B::into_bytes::<S>(self, b)
    }
}

impl<B: Serialize> Serialize for Option<B> {
    fn from_bytes<S: Size>(b: &mut &[u8]) -> Result<Self, Error> {
        let is_some = bool::from_bytes::<S>(b)?;

        match is_some {
            false => Ok(None),
            true => Ok(Some(B::from_bytes::<S>(b)?)),
        }
    }

    fn into_bytes<S: Size>(&self, b: &mut Vec<u8>) -> Result<(), Error> {
        self.is_some().into_bytes::<S>(b)?;

        if let Some(inner) = self {
            inner.into_bytes::<S>(b)?;
        }

        Ok(())
    }
}

impl Serialize for String {
    fn from_bytes<S: Size>(b: &mut &[u8]) -> Result<Self, Error> {
        match std::string::String::from_utf8(<Vec<u8>>::from_bytes::<S>(b)?) {
            Ok(s) => Ok(s),
            Err(e) => Err(Error::InvalidUtf8(e)),
        }
    }

    fn into_bytes<S: Size>(&self, b: &mut Vec<u8>) -> Result<(), Error> {
        self.as_str().into_bytes::<S>(b)
    }
}

impl Serialize for &str {
    /// This function will panic. Make sure to convert to an owned `String` instead
    fn from_bytes<S: Size>(_b: &mut &[u8]) -> Result<Self, Error> {
        panic!("Can't convert bytes to &'str. Try converting to an owned `String` instead")
    }

    fn into_bytes<S: Size>(&self, b: &mut Vec<u8>) -> Result<(), Error> {
        push_len::<S>(self.len(), b)?;
        b.extend_from_slice(self.as_bytes());

        Ok(())
    }
}
