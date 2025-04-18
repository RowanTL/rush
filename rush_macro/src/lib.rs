use crate::utils::canextractstate::Extract;
use crate::utils::extractidents::ExtractIdents;
use crate::utils::varfunc::VarFunc;
use quote::quote;
use syn::parse_macro_input;

mod utils;

/// Runs a function passed as the first argument with a variable amount of functions
/// passed to it afterward.
#[proc_macro]
pub fn run_func(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let f = parse_macro_input!(input as VarFunc);
    quote! { #f }.into()
}

/// Checks to see if extracting values from a state is possible.
#[proc_macro]
pub fn run_canextract(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let f = parse_macro_input!(input as Extract);
    quote! { #f }.into()
}

/// Returns the arguments to this proc macro in a usable way to other
/// functions. Check the utils file for a more in depth explanation.
#[proc_macro]
pub fn run_extractidents(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let f = parse_macro_input!(input as ExtractIdents);
    quote! { #f }.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert!(true);
    }
}
