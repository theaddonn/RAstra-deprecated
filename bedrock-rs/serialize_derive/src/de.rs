use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, Data, GenericParam, Generics};

pub fn add_de_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param
                .bounds
                .push(parse_quote!(bedrock_rs::core::ser::MCDeserialize));
        }
    }
    generics
}

pub fn build_de(data: &Data) -> TokenStream {
    let fields = match *data {
        Data::Struct(ref data) => &data.fields,
        _ => panic!("Serialize macro only supports structs"),
    };

    let field_de_calls = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().expect("Field has no name");
        quote! {
            #field_name: match serialize::de::MCDeserialize::deserialize(cursor) {
                Ok(v) => { v },
                Err(e) => { return Err(e) }
            },
        }
    });

    let expand = quote! {
        #(#field_de_calls)*
    };

    TokenStream::from(expand)
}
