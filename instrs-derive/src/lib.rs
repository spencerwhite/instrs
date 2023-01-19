#![feature(trait_alias)]

mod serialize;
mod util;
mod info;

use proc_macro2::TokenStream;
use proc_macro::TokenStream as TStream;
use quote_into::quote_into;
use syn::{*, spanned::Spanned};

use crate::serialize::*;
use crate::info::Info;

#[proc_macro_derive(ByteSerialize)]
pub fn derive_byte_serialize(input: TStream) -> TStream {
    let input: Info = parse_macro_input!(input as DeriveInput).into();

    let mut s = proc_macro2::TokenStream::new();

    quote_into!{s += 
        impl Serialize for #(input.name) {
            #{impl_into_bytes(&input, s)}
            #{impl_from_bytes(&input, s)}
        }
    }

    s.into()
}
