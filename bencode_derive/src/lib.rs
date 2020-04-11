//! # bencode_derive
//!
//! This crate should be used with `bencode` for generating `Decodable`
//! and `Encodable` implementations for arbitrary structs, as long as
//! their fields all conform to `Encodable` and/or `Decodable` respectively.
//!
//! Individual fields can also be renamed if necessary using
//! `#[bencode("new name")]` attribute.
mod attributes;
mod decode;
mod encode;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Used to generate a `Decodable` impl block for the given struct.
///
/// Field names can be changed by using `#[bencode("<custom name>")]`.
///
/// # Example
///
/// ```
/// use bencode::Decodable;
/// use crate::bencode_derive::Decodable;
///
/// #[derive(Decodable)]
/// struct Test {
///     #[bencode("a b")]
///     value: String
/// }
///
/// let source = b"d3:a b3:abce";
/// let decoded = Test::read_bytes(&source[..]).unwrap();
///
/// assert_eq!("abc", decoded.value);
/// ```
#[proc_macro_derive(Decodable, attributes(bencode))]
pub fn derive_decodable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let fields = decode::decode_fields(&input.data);

    let name = input.ident;
    let expanded = quote! {
        impl Decodable for #name {
            type Output = #name;

            fn decode(value: &bencode::BencodeValue) -> std::io::Result<#name> {
                Ok(#name {
                    #fields
                })
            }
        }
    };

    TokenStream::from(expanded)
}

/// Generates an impl block for `Encodable` for the given struct.
///
/// All fields need to implement the `Encodable` trait. Fields can
/// be renamed by using `#[bencode("<custom name>")] attribute.
///
/// # Example
///
/// ```
/// use bencode::Encodable;
/// use crate::bencode_derive::Encodable;
///
/// #[derive(Encodable)]
/// struct Test {
///     #[bencode("a b")]
///     value: String,
/// }
///
/// let test = Test {
///     value: String::from("abc")
/// };
///
/// let encoded = test.encode().unwrap();
///
/// assert_eq!(b"d3:a b3:abce".to_vec(), encoded);
/// ```
#[proc_macro_derive(Encodable, attributes(bencode))]
pub fn derive_encodable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let fields = encode::encode_fields(&input.data);

    let name = input.ident;
    let expanded = quote! {
        impl Encodable for #name {
            fn to_bencode(&self) -> Option<bencode::BencodeValue> {
                let mut map = std::collections::HashMap::new();
                #fields
                Some(bencode::BencodeValue::Dictionary(map))
            }
        }
    };

    TokenStream::from(expanded)
}
