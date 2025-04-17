// Run: Function Argument*
//
// Function: identifier
//
// Argument: (, expression)

use crate::utils::parse_zero_or_more;
use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use syn::parse::{Parse, ParseStream};

pub struct VarFunc {
    func: Function,
    args: Vec<Argument>,
}

impl Parse for VarFunc {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let func = input.parse()?;
        let args = parse_zero_or_more(input);
        Ok(Self { func, args })
    }
}

impl ToTokens for VarFunc {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let function = &self.func;
        let args = &self.args;
        tokens.extend(quote! {
            #function(#(#args),*)
        })
    }
}

struct Function(syn::Ident);

impl Parse for Function {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        syn::Ident::parse(input).map(Self)
    }
}

impl ToTokens for Function {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.0.to_tokens(tokens)
    }
}

struct Argument(syn::Expr);

impl Parse for Argument {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        _ = input.parse::<syn::Token![,]>()?;
        syn::Expr::parse(input).map(Self)
    }
}

impl ToTokens for Argument {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.0.to_tokens(tokens)
    }
}
