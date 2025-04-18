// Extract: State (`,` Stack `:` Amount)*
//
// State: identifier
//
// Stack: identifier
//
// Amount: expression

use crate::utils::parse_zero_or_more;
use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use std::cmp::PartialEq;
use syn::parse::{Parse, ParseStream};

pub struct Extract {
    state: State,
    stacks: Vec<Stack>,
}

impl Parse for Extract {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let state = input.parse()?;
        let stacks = parse_zero_or_more(input);
        Ok(Extract { state, stacks })
    }
}

impl ToTokens for Extract {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let inner_state = &self.state.0;
        let stacks = &self.stacks;

        let mut counts = Vec::new();
        for stack in stacks {
            match counts.iter_mut().find(|(x, _)| x == stack) {
                Some((_, count)) => *count += 1,
                None => counts.push((stack, 1usize)),
            }
        }

        let conditions = counts.iter().map(|(stack, count)| {
            let inner_stack = &stack.0;
            quote! { #inner_state.#inner_stack.len() >= #count }
        });

        tokens.extend(quote! {
            true #(&& (#conditions))*
        });
    }
}

struct State(syn::Ident);

impl Parse for State {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        syn::Ident::parse(input).map(Self)
    }
}

impl ToTokens for State {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.0.to_tokens(tokens)
    }
}

struct Stack(syn::Ident);

impl Parse for Stack {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        _ = input.parse::<syn::Token![,]>();
        syn::Ident::parse(input).map(Self)
    }
}

impl ToTokens for Stack {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.0.to_tokens(tokens)
    }
}

impl PartialEq<Stack> for &Stack {
    fn eq(&self, other: &Stack) -> bool {
        self.0 == other.0
    }
}
