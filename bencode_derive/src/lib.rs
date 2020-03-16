mod attributes;
mod decode;
mod encode;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Decodable, attributes(bencode))]
pub fn derive_decodable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let fields = decode::decode_fields(&input.data);

    let name = input.ident;
    let expanded = quote! {
        impl Decodable for #name {
            type Output = #name;

            fn decode(value: &BencodeValue) -> std::io::Result<#name> {
                Ok(#name {
                    #fields
                })
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(Encodable, attributes(bencode))]
pub fn derive_encodable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let fields = encode::encode_fields(&input.data);

    let name = input.ident;
    let expanded = quote! {
        impl Encodable for #name {
            fn to_bencode(&self) -> Option<BencodeValue> {
                let mut map = bencode::InsertOrderMap::new();
                #fields
                Some(BencodeValue::Dictionary(map))
            }
        }
    };

    TokenStream::from(expanded)
}
