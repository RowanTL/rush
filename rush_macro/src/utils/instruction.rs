//! This Stack that isn't repeated is the desired output stack.
//! Extract: Function, Stack, State, (`,` Stack)* ;?
//!
//! Function: identifier
//!
//! State: identifier
//!
//! Stack: identifier
//!
//! Aux: expression

use crate::utils::parse_zero_or_more;
use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use std::cmp::PartialEq;
use syn::parse::{Parse, ParseStream};

/// Checks if there is a semicolon at the end
fn parse_aux<T: Parse>(input: ParseStream) -> bool {
    if let Ok(_) = input.parse::<syn::Token![;]>() {
        return true;
    }
    false
}

pub struct Extract {
    func: Function,
    out_stack: Stack,
    state: State,
    stacks: Vec<Stack>,
    aux: bool,
}

impl Parse for Extract {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let func = input.parse()?;
        let out_stack = input.parse()?;
        let state = input.parse()?;
        let stacks = parse_zero_or_more(input);
        let aux = parse_aux::<syn::Token![,]>(input);
        Ok(Extract {
            func,
            out_stack,
            state,
            stacks,
            aux,
        })
    }
}

impl ToTokens for Extract {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let inner_func = &self.func;
        let inner_out_stack = &self.out_stack.0;
        let inner_state = &self.state.0;
        let stacks = &self.stacks;
        let aux = &self.aux;

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

        let values = stacks.iter().map(|stack| {
            let inner_stack = &&stack.0;
            quote! { #inner_state.#inner_stack.pop().unwrap() }
        });

        let aux_run = match aux {
            true => quote! {
                let result = #inner_func(#(#values),*);
                #inner_state.#inner_out_stack.extend(result.iter());
            },
            false => quote! {
                let result = #inner_func(#(#values),*);
                #inner_state.#inner_out_stack.push(result);
            },
        };

        tokens.extend(quote! {
            if true #(&& (#conditions))* {
                #aux_run
            }
        });
    }
}

struct State(syn::Ident);

impl Parse for State {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        _ = input.parse::<syn::Token![,]>();
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
