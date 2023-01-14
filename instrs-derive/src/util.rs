
use proc_macro2::TokenStream;
use proc_macro::TokenStream as TStream;
use quote_into::quote_into;
use syn::{*, spanned::Spanned};

pub trait Callback = FnMut(&Field, &Ident, &mut TokenStream);

pub fn unnamed_field_ident(field: &Field, i: usize) -> Ident {
    let ident_str = format!("ident_{}", i);
    let ident_span = field.ty.span();
    Ident::new(&ident_str, ident_span)
}

pub fn idents(fields: &Fields, s: &mut TokenStream) {
    foreach_field(fields, s, |_, ident, s| {
        quote_into!{s +=
            #(ident),
        }
    })
}

pub fn foreach_field(fields: &Fields, s: &mut TokenStream, mut callback: impl Callback) {
    match fields {
        Fields::Named(fields) => for field in fields.named.iter() {
            let ident = field.clone().ident.unwrap();
            callback(field, &ident, s);
        },
        Fields::Unnamed(fields) => for (i, field) in fields.unnamed.iter().enumerate() {
            let ident = unnamed_field_ident(&field, i);
            callback(field, &ident, s);
        },
        Fields::Unit => {},
    }
}

pub fn match_fields(fields: &Fields, s: &mut TokenStream) {
    if let Fields::Unit {..} = fields {return;}

    if let Fields::Unnamed {..} = fields {
        quote_into!{ s +=
            (#{
                foreach_field(fields, s, |_, ident, s| {
                    quote_into!{s += #(ident),}
                })
            })
        }
    }

    if let Fields::Named {..} = fields {
        quote_into!{ s +=
            {#{
                foreach_field(fields, s, |_, ident, s| {
                    quote_into!{s += #(ident),}
                })
            }}
        }
    }
}
