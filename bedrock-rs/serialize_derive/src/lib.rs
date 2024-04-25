use bytes::BufMut;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{parse_macro_input, DeriveInput};

use crate::de::{add_de_trait_bounds, build_de};
use crate::ser::{add_ser_trait_bounds, build_ser};

mod de;
mod ser;

#[proc_macro_derive(MCSerialize)]
pub fn serialize(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    let name = input.ident;
    let fields = &input.data;

    let generics = add_ser_trait_bounds(input.generics);

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let ser = build_ser(&input.data);

    let expanded = quote! {
        impl #impl_generics serialize::ser::MCSerialize for #name #ty_generics #where_clause {
            fn serialize(&self) -> Result<Vec<u8>, serialize::error::SerilizationError> where Self: Sized {
                let mut buf = vec![];

                #ser

                return Ok(buf);
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

#[proc_macro_derive(MCDeserialize)]
pub fn deserialize(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    let name = input.ident;
    let fields = &input.data;

    let generics = add_de_trait_bounds(input.generics);

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let de = build_de(&input.data);

    let expanded = quote! {
        impl #impl_generics serialize::de::MCDeserialize for #name #ty_generics #where_clause {
            fn deserialize(cursor: &mut std::io::Cursor<Vec<u8>>) -> Result<Self, serialize::error::DeserilizationError> where Self: Sized {
                return Ok(
                    Self {
                        #de
                    }
                );
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
