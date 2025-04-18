use crate::utils::instruction::Extract;
use quote::quote;
use syn::parse_macro_input;

mod utils;

/// This macro kinda goes super crazy mode
/// Here's how to use the macro:
///
/// `run_instruction!(function_name, output_stack, push state, any amount of
/// comma separated stacks by name ; (the semicolon instructs use whether the instruction
/// has multiple outputs. If ; passed, assumes multiple, without assumes just one output))`
///
/// An instruction for int add would be:
/// `run_instruction!(_add, int, state, int, int)`
/// assuming the _add function takes two integers and returns a single value (Hint: It does).
///
/// Important notice: the order in which the last amount
/// of stacks are passed in matters. The first `int` identifier will tell
/// the macro to pop the top int first, the second `int` will pop the next int
/// and so on. This even works with multiple different types of stacks.
///
/// Another important notice: This macro generates boundary checking as well.
/// If there are not enough items in the stack to run the function, it
/// will not be called.
///
/// A function with multiple outputs, for example this one:
/// ```
/// fn aux_iadd(x: i128, y: i128) -> Option<Vec<i128>> {
///     Some(vec![x + y, x - y])
/// }
///
/// run_instruction!(aux_iadd, int, state, int, int;);
/// ```
/// would have the ; placed at the end of the instruction. Check rush's `tests/instruction_test.rs`
/// file for an example using this code.
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
