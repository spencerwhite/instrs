use proc_macro2::{TokenStream, Literal};
use proc_macro::TokenStream as TStream;
use quote_into::quote_into;
use syn::{*, spanned::Spanned};

pub(crate) struct Instruction {
    pub name: Ident,
    pub fields: Fields,
}

impl From<Variant> for Instruction {
    fn from(value: Variant) -> Self {
        Self { name: value.ident, fields: value.fields }
    }
}

pub(crate) struct Info {
    pub name: Ident,
    pub generics: Generics,
    pub instructions: Vec<Instruction>,
    pub repr: Box<dyn Fn(usize) -> Literal>,
}

impl From<DeriveInput> for Info {
    fn from(value: DeriveInput) -> Self {
        let name = value.ident;
        let generics = value.generics;

        let Data::Enum(data) = value.data else {panic!("derive macro currently only supports enums")};

        let instructions: Vec<_> = data.variants.into_iter()
            .map(|variant| variant.into())
            .collect(); 

        let last_variant = match instructions.len().checked_sub(1) {
            Some(n) => n,
            None => 0,
        };

        let repr = Box::new(match last_variant.checked_ilog2() {
            Some(n) => match n {
                00..=07 => |n: usize| Literal::u8_suffixed(n.try_into().unwrap()),
                08..=15 => |n: usize| Literal::u16_suffixed(n.try_into().unwrap()),
                16..=31 => |n: usize| Literal::u32_suffixed(n.try_into().unwrap()),
                32..=63 => |n: usize| Literal::u64_suffixed(n.try_into().unwrap()),
                64..=127 => |n: usize| Literal::u128_suffixed(n.try_into().unwrap()),
                _ => panic!("Enums with >=2^128 variants are not supported\nAlso how did you even do that lmao"),
            },
            None => |n: usize| Literal::u8_suffixed(n.try_into().unwrap()),
        });

        Self { name, generics, instructions, repr, }
    }
}
