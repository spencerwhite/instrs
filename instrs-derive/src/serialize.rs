use proc_macro2::TokenStream;
use quote_into::quote_into;
use syn::*;

use crate::util::*;
use crate::info::*;

pub(crate) fn impl_into_string(input: &Info, s: &mut TokenStream) {
    fn foreach_field_into_string(fields: &Fields, s: &mut TokenStream) {
        foreach_field(fields, s, |_, ident, s| {
            quote_into!{s +=
                Serialize::into_string(#(ident), f);
            }
        })
    }

    fn instruction_matches(name: &Ident, instructions: &Vec<Instruction>, s: &mut TokenStream) {
        for instruction in instructions.iter() {
            let mut name_space = instruction.name.to_string();
            name_space.push(' ');

            quote_into!{s += 
                //OpCode::Add               {a, b, c}                              => write!(f, "Add {} {} {}"                , a, b, c, )
                #name::#(instruction.name) #{match_fields(&instruction.fields, s)} => {
                    f.push_str(#(name_space));
                    #{foreach_field_into_string(&instruction.fields, s)}

                    #{
                        if let Fields::Unit = instruction.fields {} else {
                            quote_into!{s +=
                                f.push(' ');
                            }
                        }
                    }
                },
            };
        }
    }

    quote_into!{ s +=
        fn into_string(&self, f: &mut String) {
            match self {
                #{instruction_matches(&input.name, &input.instructions, s)}
            }
        }
    };
}

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

pub(crate) fn impl_from_string(input: &Info, s: &mut TokenStream) {
    fn foreach_field_from_string(fields: &Fields, s: &mut TokenStream) {
        foreach_field(fields, s, |_, ident, s| {
            quote_into!{s +=
                let #(ident) = Serialize::from_string(f)?;
            }
        })
    }

    fn instruction_matches(name: &Ident, instructions: &Vec<Instruction>, s: &mut TokenStream) {
        for instruction in instructions.iter() {
            let mut name_space = instruction.name.to_string();
            name_space.push(' ');

            let len = name_space.len();

            quote_into!{s += 
                if f.starts_with(#(name_space)) {
                    *f = &f[#(len)..];
                    #{foreach_field_from_string(&instruction.fields, s)}

                    return Ok(Self::#(instruction.name) #{match_fields(&instruction.fields, s)})
                }
            };
        }
    }

    quote_into!{ s +=
        fn from_string(f: &mut &str) -> Result<Self, Error> {
            #{instruction_matches(&input.name, &input.instructions, s)}
            return Err(Error::Expected)
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

    quote_into!{ s +=
        fn from_bytes(f: &mut &[u8]) -> Result<Self, Error> {
            match f.first() {
                #{instruction_matches(&input.name, &input.instructions, s)}

                _ => return Err(Error::Expected)
            }
        }
    };
}
