extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct, Meta, NestedMeta};

/// The whole point.
/// 
/// Use this macro as a shorthand for a negative impl and to confuse 
/// your coworkers who will definitely misread it upon first sight:
/// 
/// ```
/// #![feature(negative_impls)]
/// use deprive::deprive;
/// #[deprive(Send, Sync)]
/// struct X {}
/// ```
///
/// The above will expand to:
///
/// ```
/// #![feature(negative_impls)]
/// struct X {}
/// impl !Send for X {}
/// impl !Sync for X {}
/// ```
#[proc_macro_attribute]
pub fn deprive(attr: TokenStream, input: TokenStream) -> TokenStream {
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

    let expanded = quote! {
        #input

        #impls
    };

    TokenStream::from(expanded)
}
