use crate::utils::canextractstate::Extract;
use quote::quote;
use syn::parse_macro_input;

mod utils;

/// Checks to see if extracting values from a state is possible.
#[proc_macro]
pub fn run_instruction(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let f = parse_macro_input!(input as Extract);
    quote! { #f }.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert!(true);
    }
}
