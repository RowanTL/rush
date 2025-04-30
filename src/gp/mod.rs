use args::PushArgs;

pub mod args;
pub mod genome;
pub mod individual;
pub mod selection;
pub mod simplification;
pub mod utils;
pub mod variation;

pub fn gp_loop(push_args: PushArgs) -> bool {
    let _pop_size = push_args.pop_size;
    let _max_gens = push_args.max_generations;
    let _error_func = push_args.error_function;
    let _solution_error_threshold = push_args.solution_error_threshold;
    let _dont_end = push_args.dont_end;
    let _elitism = push_args.elitism;
    let _training_data = push_args.training_data;
    let _testing_data = push_args.testing_data;
    let _simplification = push_args.use_simplification;

    true
}
