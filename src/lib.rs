extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct, Meta, NestedMeta};

#[proc_macro_attribute]
pub fn deprive(attr: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as ItemStruct);
    let attr = parse_macro_input!(attr as syn::AttributeArgs);

    let id = input.ident.clone();

    let impls = attr
        .iter()
        .map(|trayt| match trayt {
            NestedMeta::Meta(Meta::Path(trayt)) => {
                quote! {
                    impl ! #trayt for #id {}
                }
            }
            _ => {
                panic!("Unsupported");
            }
        })
        .fold(quote! {}, |acc, new| quote! {#acc #new});

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        #input

        #impls
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
