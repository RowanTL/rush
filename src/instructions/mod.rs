#[macro_use]
pub mod macros {
    /// A macro that makes a push instruction given: the name of the input stack to use,
    /// the name of the output stack, an internal function to call, the type of a function,
    /// and the arity of the internal function call.
    ///
    /// The `in_stack` argument refers to which push stack should this operate on.
    /// The `out_stack` argument refers to which push stack should the result be pushed to.
    /// The `fn_name` argement refers to the name of the function that is to operate
    /// on the values popped from `in_stack`.
    /// The `fn_type` argument refers to the type of `in_stack`. For example, the
    /// int stack is type: *Vec<i128>*. `fn_type` is *i128* in this case.
    /// The `fn_arity` argument refers to how many popped stack items are needed to
    /// execute the instruction. If the amount of items in the stack is less than
    /// this value, the instruction does nothing.
    ///
    /// What causes an instruction to NoOp:
    /// 1) There aren't enough values on a stack to execute an instruction.
    /// 2) The internal operation the instruction executes is unable to be ran without
    ///    erroring such as division by 0.
    #[macro_export]
    macro_rules! make_instruction {
        ($in_stack:ident, $out_stack:ident, $fn_name:ident, $fn_type:ty, $fn_arity:stmt) => {
            paste::item! {
                fn [< $in_stack $fn_name >] (state: &mut PushState) {
                    if state.$in_stack.len() < $fn_arity {
                        return;
                    }
                    let mut inputs: Vec<$fn_type> = Vec::with_capacity($fn_arity);
                    for n in 1..=$fn_arity {
                        inputs.push(state.$in_stack[state.$in_stack.len() - n]);
                    }
                    if let Some(result) = $fn_name(inputs) {
                        for _ in 0..$fn_arity {
                            state.$in_stack.pop();
                        }
                        state.$out_stack.push(result);
                    }
                }
            }
        };
    }
}

pub mod numeric;
pub mod utils;

// /// A macro that makes a push instruction given: the name of the stack to use,
// /// an internal function to call, and the type of a function.
// #[macro_export]
// macro_rules! make_instruction {
//     ($stack_name:ident, $fn_name:ident, $fn_type:ty) => {
//         paste::item! {
//             fn [< $stack_name $fn_name >] (state: &mut PushState, num_inputs: usize) {
//                 if state.$stack_name.len() < num_inputs {
//                     return;
//                 }
//                 let mut inputs: Vec<$fn_type> = Vec::with_capacity(num_inputs);
//                 for n in 0..num_inputs {
//                     inputs.push(state.$stack_name[n]);
//                 }
//                 if let Some(result) = $fn_name(inputs) {
//                     for _ in 0..num_inputs {
//                         state.$stack_name.pop();
//                     }
//                     state.$stack_name.push(result);
//                 }
//             }
//         }
//     };
// }
