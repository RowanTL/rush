//! Extracts identifiers from a passed list of them.
//!
//! IdentExtract: identifier CommaIdentifier*
//!
//! CommaIdentifier: (, identifier)
//!
//! For Example:
//! run_extract_idents!(int, int, int) would return (literally):
//! `int, int, int` to rust

use crate::utils::parse_zero_or_more;
use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use std::io::stdout;
use syn::parse::{Parse, ParseStream};

pub struct ExtractIdents {
    ident: syn::Ident,
    comma_idents: Vec<CommaIdentifiers>,
}

impl Parse for ExtractIdents {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse()?;
        let comma_idents = parse_zero_or_more(input);
        Ok(Self {
            ident,
            comma_idents,
        })
    }
}

impl ToTokens for ExtractIdents {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let first_ident = &self.ident;
        let comma_idents = &self.comma_idents;

        /*tokens.extend(quote! { #first_ident });
        for id in comma_idents {
            let syn_id = &id.0;
            tokens.(syn_id);
        }*/

        let output = quote! {
            #first_ident #(, #comma_idents)*
        };

        tokens.extend(output);
    }
}

pub struct CommaIdentifiers(syn::Ident);

impl Parse for CommaIdentifiers {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        _ = input.parse::<syn::Token![,]>()?;
        syn::Ident::parse(input).map(Self)
    }
}

impl ToTokens for CommaIdentifiers {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.0.to_tokens(tokens)
    }
}
