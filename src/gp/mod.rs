use args::PushArgs;

pub mod args;
pub mod genome;
pub mod individual;
pub mod selection;
pub mod simplification;
pub mod utils;
pub mod variation;

pub fn gp_loop(push_args: PushArgs) -> bool {
    let pop_size = push_args.pop_size;
    let max_gens = push_args.max_generations;
    let error_func = push_args.error_function;
    let solution_error_threshold = push_args.solution_error_threshold;
    let dont_end = push_args.dont_end;
    let elitism = push_args.elitism;
    let training_data = push_args.training_data;
    let testing_data = push_args.testing_data;
    let simplification = push_args.use_simplification;

    true
}
