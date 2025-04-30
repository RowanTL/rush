use crate::gp::selection::Selection;
use crate::gp::variation::Variation;
use crate::push::state::Gene;
use polars::prelude::*;
use rust_decimal::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Copy)]
pub enum ClosingType {
    Specified,
    Balanced,
    None,
}

#[derive(Clone, Copy)]
pub enum SearchDirection {
    Min,
    Max,
}

#[allow(dead_code)]
pub struct PushArgs {
    pub alignment_deviation: Decimal, // For alternation, std dev of deviation of index when alternating
    pub alternation_rate: usize, // For alternation, prob of switching parents at each location. A number 0-100
    pub closes: ClosingType,     // How push should automatically place Gene::Close into a plushy
    pub dont_end: bool,          // If true, keep running until limit regardless of success
    // pub downsample: bool, // Whether or not to downsample. TODO later with all the related args
    pub elitism: bool, // Whether to always add the best individual to next generation
    pub error_function: Option<fn(&PushArgs, &DataFrame, Vec<Gene>) -> Vec<Decimal>>, // The error function
    pub instructions: Option<Vec<Gene>>, // Instructions to use in a run
    pub max_generations: usize,          // Max amount of generations
    pub max_init_plushy_size: usize,     // max initial plushy size
    pub max_stack_size: usize,           // max size a stack is allowed to reach during execution
    pub parent_selection: Selection,     // Selection to use, TODO change this later.
    pub pop_size: usize,                 // Population size
    pub replacement_rate: f64,           // For uniform replacement, rate items replaced
    pub use_simplification: bool,        // Whether to use simplification at end of run
    pub search_direction: SearchDirection, // Whether the problem is a minimization or maximization problem
    pub simplification_k: usize, // Max amt of genes to attempt removal during one round of simplification process
    pub simplification_steps: usize, // How many attempts to find simplified genomes
    pub simplification_verbose: bool, // Whether to send extra messages about simplification or not
    pub solution_error_threshold: Decimal, // Max total error for solutions
    pub use_single_thread: bool, // if true, only single threaded
    pub step_limit: usize,       // Amount of steps a push interpreter can run for
    pub testing_data: Option<DataFrame>, // The testing data, must be formatted the same as training data
    pub tournament_size: usize,          // Tournament size for tournament selection
    pub training_data: Option<DataFrame>, // The training data, must be formatted the same as testing data
    pub umad_rate: f64,                   // addition rate (deletion rate derived) for UMAD
    pub variation: HashMap<Variation, f64>, // genetic operators and probability for use. should sum to 1,
}

impl PushArgs {
    /// Holds the default arguments
    pub fn new() -> Self {
        let mut map = HashMap::new();
        map.insert(Variation::UMAD, 1.0);

        Self {
            alignment_deviation: dec!(2.0),
            alternation_rate: 10,
            closes: ClosingType::Specified,
            dont_end: false,
            elitism: false,
            error_function: None,
            instructions: None,
            max_generations: 1000,
            max_init_plushy_size: 100,
            max_stack_size: 100,
            parent_selection: Selection::Lexicase,
            pop_size: 1000,
            replacement_rate: 0.1,
            use_simplification: true,
            search_direction: SearchDirection::Min,
            simplification_k: 4,
            simplification_steps: 1000,
            simplification_verbose: true,
            use_single_thread: false,
            solution_error_threshold: dec!(0.0),
            step_limit: 1000,
            testing_data: None,
            tournament_size: 5,
            training_data: None,
            umad_rate: 0.1,
            variation: map,
        }
    }
}
