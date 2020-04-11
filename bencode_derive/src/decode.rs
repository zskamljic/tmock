use crate::attributes;
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{Data, Fields, FieldsNamed};

/// Generate a stream of fields and their decode functions.
///
/// Only struct type is supported right now, other types will panic with
/// unimplemented.
pub fn decode_fields(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => decode_struct(&data.fields),
        _ => unimplemented!("Only struct type decoding is supported"),
    }
}

/// Decode the values of a struct.
///
/// Fails with unimplemented panic if fields are not named (should not
/// happen with structs)
fn decode_struct(fields: &Fields) -> TokenStream {
    match fields {
        Fields::Named(ref fields) => decode_named_fields(fields),
        _ => unimplemented!("Only named fields are supported"),
    }
}

/// Decode individual fields using the attribute to rename them if
/// necessary.
fn decode_named_fields(fields: &FieldsNamed) -> TokenStream {
    let recurse = fields.named.iter().map(|field| {
        let name = &field.ident;
        let ty = &field.ty;
        let serialized_name = attributes::process_field_attributes(name, &field.attrs);
        quote_spanned! { field.span() =>
            #name: bencode::decode::<#ty>(&value, #serialized_name)?
        }
    });

    quote! {
        #(#recurse,)*
    }
}
