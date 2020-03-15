use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Fields, FieldsNamed};

#[proc_macro_derive(Decodable)]
pub fn derive_decodable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let fields = decode_fields(&input.data);

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

fn decode_fields(data: &Data) -> TokenStream2 {
    match *data {
        Data::Struct(ref data) => decode_struct(&data.fields),
        _ => unimplemented!(),
    }
}

fn decode_struct(fields: &Fields) -> TokenStream2 {
    match fields {
        Fields::Named(ref fields) => decode_named_fields(fields),
        _ => unimplemented!(),
    }
}

fn decode_named_fields(fields: &FieldsNamed) -> TokenStream2 {
    let recurse = fields.named.iter().map(|field| {
        let name = &field.ident;
        let ty = &field.ty;
        quote_spanned! { field.span() =>
            #name: bencode::decode::<#ty>(&value, stringify!(#name))?
        }
    });

    quote! {
        #(#recurse,)*
    }
}
