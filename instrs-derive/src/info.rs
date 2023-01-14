use proc_macro2::TokenStream;
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
}

impl From<DeriveInput> for Info {
    fn from(value: DeriveInput) -> Self {
        let name = value.ident;
        let generics = value.generics;

        let Data::Enum(data) = value.data else {panic!("derive macro currently only supports enums")};

        let instructions = data.variants.into_iter()
            .map(|variant| variant.into())
            .collect();

        Self { name, generics, instructions, }
    }
}
