//! Where the hard work for automatically generating instructions is done.
//! Check `run_instruction!` for more details.
//!
//! This Stack that isn't repeated is the desired output stack.
//! Extract: Function, Stack, State (`,` Stack)* ;?
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

        // Gets the counts of each stack passed to the macro held
        // similarly to a map as: (stack, count).
        //
        // Chosen over a HashMap bc these types don't implement Hash
        // HashMaps are O(nlogn) at worst this is O(n^2). Largest instruction
        // passes 4 stacks so this shouldn't matter in the long run.
        let mut counts = Vec::new();
        for stack in stacks {
            match counts.iter_mut().find(|(x, _)| x == stack) {
                Some((_, count)) => *count += 1,
                None => counts.push((stack, 1usize)),
            }
        }

        // Writes the piece of the code that ensures the stacks have enough values
        // to function without error.
        let conditions = counts.iter().map(|(stack, count)| {
            let inner_stack = &stack.0;
            quote! { #inner_state.#inner_stack.len() >= #count }
        });

        // In case the instruction returns None (meaning revert the state),
        // need to store the values to return them
        let store_values = stacks.iter().enumerate().map(|(i, stack)| {
            let inner_stack = &&stack.0;
            let var_name = quote::format_ident!("val_{}", i);
            quote! { let #var_name = #inner_state.#inner_stack.pop().unwrap(); }
        });

        // Create the variable names themselves to store the
        // popped values.
        let value_vars = (0..stacks.len())
            .map(|i| quote::format_ident!("val_{}", i))
            .collect::<Vec<_>>();

        // Create restore code in case None is returned from the function.
        let restore_values =
            stacks
                .iter()
                .rev()
                .zip(value_vars.iter().rev())
                .map(|(stack, var)| {
                    let inner_stack = &&stack.0;
                    quote! { #inner_state.#inner_stack.push(#var.clone()); }
                });

        // The logic for running the function and returning values
        // if bad.
        let aux_run = match aux {
            true => quote! {
                let result = #inner_func(#(#value_vars.clone()),*);
                if let Some(result) = result {
                    // Transforming the result vector into an iterator with .iter() was
                    // causing problems with the vector_string stack. Iterating this way
                    // fixes the problem.
                    for n in 0..result.len() {
                        #inner_state.#inner_out_stack.push(result[n].clone())
                    }
                } else {
                    #(#restore_values)*
                }
            },
            false => quote! { // This arm is used most of the time
                let result = #inner_func(#(#value_vars.clone()),*);
                if let Some(result) = result {
                    #inner_state.#inner_out_stack.push(result);
                } else {
                    #(#restore_values)*
                }
            },
        };

        // Where the pieces of the puzzle are put together.
        // tokens then used to create the function.
        tokens.extend(quote! {
            if true #(&& (#conditions))* {
                #(#store_values)*
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
