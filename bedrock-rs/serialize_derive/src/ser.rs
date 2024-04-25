use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, Data, GenericParam, Generics};

pub fn add_ser_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param
                .bounds
                .push(parse_quote!(bedrock_rs::core::ser::MCSerialize));
        }
    }
    generics
}

pub fn build_ser(data: &Data) -> TokenStream {
    let fields = match *data {
        Data::Struct(ref data) => &data.fields,
        _ => panic!("Serialize macro only supports structs"),
    };

    let field_ser_calls = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().expect("Field has no name");
        quote! {
            buf.extend_from_slice(&*(match self.#field_name.serialize() {
                Ok(v) => { v },
                Err(e) => { return Err(e) }
            }));
        }
    });

    let expand = quote! {
        #(#field_ser_calls)*
    };

    TokenStream::from(expand)
}
