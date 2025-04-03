#[macro_use]
pub mod macros {
    //  A macro that makes a push instruction given: the name of the stack to use,
    //  an internal function to call, and the type of a function.
    #[macro_export]
    macro_rules! make_instruction {
        ($stack_name:ident, $fn_name:ident, $fn_type:ty) => {
            paste::item! {
                fn [< $stack_name $fn_name >] (state: &mut PushState, num_inputs: usize) {
                    if state.$stack_name.len() < num_inputs {
                        return;
                    }
                    let mut inputs: Vec<$fn_type> = Vec::with_capacity(num_inputs);
                    for n in 0..num_inputs {
                        inputs.push(state.$stack_name[n]);
                    }
                    if let Some(result) = $fn_name(inputs) {
                        for _ in 0..num_inputs {
                            state.$stack_name.pop();
                        }
                        state.$stack_name.push(result);
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
