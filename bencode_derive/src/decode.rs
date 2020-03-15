use crate::attributes;
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{Data, Fields, FieldsNamed};

pub fn decode_fields(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => decode_struct(&data.fields),
        _ => unimplemented!(),
    }
}

fn decode_struct(fields: &Fields) -> TokenStream {
    match fields {
        Fields::Named(ref fields) => decode_named_fields(fields),
        _ => unimplemented!(),
    }
}

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
