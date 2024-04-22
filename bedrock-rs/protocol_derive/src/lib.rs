use proc_macro2::{Ident, TokenStream};
use syn::{Attribute, Data, DeriveInput, Fields, GenericParam, Generics, parse_macro_input, parse_quote};
use quote::{quote, TokenStreamExt, ToTokens};
use quote::__private::ext::RepToTokensExt;
use protocol_core::gamepacket;
use protocol_core::io::packet_writer::PacketWriter;
use protocol_core::info::GamePacket;

#[proc_macro_derive(GamepacketSerialize, attributes(repr, skip))]
pub fn gamepacket_serialize(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    for attr in input.attrs {
        println!("ATTR: {}", attr.to_token_stream().to_string())
    }
    
    let name = input.ident;
    let fields = &input.data;
    
    let generics = add_trait_bounds_gamepacket_serialize(input.generics);
    
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let return_data_tokens = build_gamepacket_serialize(&input.data, GamePacket::Login);

    let expanded = quote! {
        
        impl #impl_generics bedrock_rs::protocol::gamepacket::GamepacketSerialize for #name #ty_generics #where_clause {
            fn to_packet(&self) -> Result<Vec<u8>, bedrock_rs::protocol::error::ProtocolError> where Self: Sized + bedrock_rs::protocol::gamepacket::Serialize {
                let writer = bedrock_rs::protocol::io::packet_writer::PacketWriter::new_game_packet_writer();
                
                #return_data_tokens
                
                return Ok(vec![]);
            }
        }
    };
    
    proc_macro::TokenStream::from(expanded)
}


fn add_trait_bounds_gamepacket_serialize(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(bedrock_rs::protocol::gamepacket::DataSerialize));
        }
    }
    generics
}

fn build_gamepacket_serialize(data: &Data, game_packet: GamePacket) -> TokenStream {
    let mut write = vec![quote!{}];
    
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    'field_iter: for field in fields.named.iter() {
                        let ident = field.ident.as_ref().unwrap();
                        let attrs = field.attrs.clone();
                        
                        for attr in attrs {
                            if attr.path().is_ident("skip") {
                                continue 'field_iter;
                            }
                            
                            else { 
                                
                            }
                        }
                        
                        let stype = match field.ty.to_token_stream().to_string().as_str() {
                            "i64" => {
                                write.push( quote! {writer.})
                            },
                            other => {}
                        };
                    }
                    
                    
                }
                Fields::Unnamed(ref fields) => {
                    
                }
                Fields::Unit => {
                    unimplemented!("Serialization not implemented for unit structs")
                }
            }
        }
        Data::Enum(_) => {
            unimplemented!("Serialization not implemented for enums")
        }
        Data::Union(_) => {
            unimplemented!("Serialization not implemented for unions")
        }
    }

    quote!{
    }
}