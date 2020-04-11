use proc_macro2::Ident;
use syn::{Attribute, Lit, Meta, NestedMeta};

/// Handles the attributes seen on the field.
///
/// Used to rename field when encoding and decoding. Will panic if more
/// than one attribute is found, as there is no way to determine which
/// one to take.
///
/// # Arguments
///
/// * `name` - the identifier for which to process the attribute
/// * `attributes` - the slice of attributes to search for
pub fn process_field_attributes(name: &Option<Ident>, attributes: &[Attribute]) -> String {
    let filtered: Vec<&Attribute> = attributes
        .iter()
        .filter(|attribute| attribute.path.is_ident("bencode"))
        .collect();

    if filtered.is_empty() {
        return name.as_ref().unwrap().to_string();
    } else if filtered.len() > 1 {
        panic!("Only one attribute can be present at a time");
    }

    match filtered[0].parse_meta() {
        Ok(Meta::List(list)) => parse_field_name(list.nested.into_iter().collect()),
        _ => panic!("expected #[bencode(\"custom_name\")]"),
    }
}

/// Parse the name of the field, returning the renamed value of present,
///
/// Panics if the element is not a string literal.
fn parse_field_name(elements: Vec<NestedMeta>) -> String {
    if elements.len() != 1 {
        panic!("Expected single name element");
    }

    match &elements[0] {
        NestedMeta::Lit(Lit::Str(value)) => value.value(),
        _ => panic!("Expected quoted string"),
    }
}
