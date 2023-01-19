use proc_macro2::TokenStream;
use quote_into::quote_into;
use syn::*;

use crate::util::*;
use crate::info::*;

pub(crate) fn impl_into_bytes(input: &Info, s: &mut TokenStream) {
    fn foreach_field_into_bytes(fields: &Fields, s: &mut TokenStream) {
        foreach_field(fields, s, |_, ident, s| {
            quote_into!{s +=
                Serialize::into_bytes(#(ident), f);
            }
        })
    }

    fn instruction_matches(name: &Ident, instructions: &Vec<Instruction>, s: &mut TokenStream) {
        for (i, instruction) in instructions.iter().enumerate() {
            quote_into!{s += 
                #name::#(instruction.name) #{match_fields(&instruction.fields, s)} => {
                    f.push(#(i as u8));
                    #{foreach_field_into_bytes(&instruction.fields, s)}
                },
            };
        }
    }

    quote_into!{ s +=
        fn into_bytes(&self, f: &mut Vec<u8>) {
            match self {
                #{instruction_matches(&input.name, &input.instructions, s)}
            }
        }
    };
}

pub(crate) fn impl_from_bytes(input: &Info, s: &mut TokenStream) {
    fn foreach_field_from_string(fields: &Fields, s: &mut TokenStream) {
        foreach_field(fields, s, |_, ident, s| {
            quote_into!{s +=
                let #(ident) = Serialize::from_bytes(f)?;
            }
        })
    }

    fn instruction_matches(name: &Ident, instructions: &Vec<Instruction>, s: &mut TokenStream) {
        for (i, instruction) in instructions.iter().enumerate() {
            quote_into!{ s+=
                Some(#(i as u8)) => {
                    *f = &f[1..];
                    #{foreach_field_from_string(&instruction.fields, s)}
                    
                    return Ok(Self::#(instruction.name) #{match_fields(&instruction.fields, s)})
                }
            };
        }
    }

    let n_instructions = input.instructions.len() as u32 - 1;

    quote_into!{ s +=
        fn from_bytes(f: &mut &[u8]) -> Result<Self, instrs::Error> {
            match f.first() {
                #{instruction_matches(&input.name, &input.instructions, s)}

                None => return Err(instrs::Error::ExpectedBytes(1)),
                _ => return Err(Error::ExpectedRange(0..=#(n_instructions)))
            }
        }
    };
}
