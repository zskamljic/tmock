use crate::attributes;
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{Data, Fields, FieldsNamed};

pub fn encode_fields(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => encode_struct(&data.fields),
        _ => unimplemented!(),
    }
}

fn encode_struct(fields: &Fields) -> TokenStream {
    match fields {
        Fields::Named(ref fields) => encode_named_fields(fields),
        _ => unimplemented!(),
    }
}

fn encode_named_fields(fields: &FieldsNamed) -> TokenStream {
    let recurse = fields.named.iter().map(|field| {
        let name = &field.ident;
        let serialized_name = attributes::process_field_attributes(name, &field.attrs);
        quote_spanned! { field.span() =>
            if let Some(value) = self.#name.to_bencode() {
                map.insert(#serialized_name.to_string(), value);
            }
        }
    });

    quote! {
        #(#recurse)*
    }
}
