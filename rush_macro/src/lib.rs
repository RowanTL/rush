use crate::utils::varfunc::VarFunc;
use quote::quote;
use syn::parse_macro_input;

mod utils;

#[proc_macro]
pub fn run_func(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let f = parse_macro_input!(input as VarFunc);
    quote! { #f }.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(true);
    }
}
