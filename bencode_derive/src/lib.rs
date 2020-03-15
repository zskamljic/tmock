use proc_macro::TokenStream;
use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, Attribute, Data, DeriveInput, Fields, FieldsNamed, Lit, Meta, NestedMeta,
};

#[proc_macro_derive(Decodable, attributes(bencode))]
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
        let serialized_name = process_field_attributes(name, &field.attrs);
        quote_spanned! { field.span() =>
            #name: bencode::decode::<#ty>(&value, #serialized_name)?
        }
    });

    quote! {
        #(#recurse,)*
    }
}

fn process_field_attributes(name: &Option<Ident>, attributes: &Vec<Attribute>) -> String {
    let filtered: Vec<&Attribute> = attributes
        .iter()
        .filter(|attribute| attribute.path.is_ident("bencode"))
        .collect();

    if filtered.len() == 0 {
        return name.as_ref().unwrap().to_string();
    } else if filtered.len() > 1 {
        panic!("Only one attribute can be present at a time");
    }

    match filtered[0].parse_meta() {
        Ok(Meta::List(list)) => parse_field_name(list.nested.into_iter().collect()),
        _ => panic!("expected #[bencode(\"custom_name\")]"),
    }
}

fn parse_field_name(elements: Vec<NestedMeta>) -> String {
    if elements.len() != 1 {
        panic!("Expected single name element");
    }

    match &elements[0] {
        NestedMeta::Lit(Lit::Str(value)) => value.value(),
        _ => panic!("Expected quoted string"),
    }
}
