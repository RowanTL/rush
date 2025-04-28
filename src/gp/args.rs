use crate::gp::variation::Variation;
use crate::push::state::Gene;
use polars::prelude::*;
use rust_decimal::prelude::*;
use std::collections::HashMap;

pub enum ClosingType {
    Specified,
    Balanced,
    None,
}

struct PushArgs {
    alignment_deviation: usize, // For alternation, std dev of deviation of index when alternating
    alternation_rate: Decimal,  // For alternation, prob of switching parents at each location
    closes: ClosingType,        // How push should automatically place Gene::Close into a plushy
    dont_end: bool,             // If true, keep running until limit regardless of success
    // downsample: bool, // Whether or not to downsample. TODO later with all the related args
    elitism: bool, // Whether to always add the best individual to next generation
    error_function: fn(&PushArgs, DataFrame, Vec<Gene>) -> Series, // The error function
    instructions: Vec<Gene>, // Instructions to use in a run
    max_init_plushy_size: usize, // max initial plushy size
    max_generations: usize, // Max amount of generations
    parent_selection: usize, // Selection to use, TODO change this later.
    pop_size: usize, // Population size
    replacement_rate: Decimal, // For uniform replacement, rate items replaced
    use_simplification: bool, // Whether to use simplification at end of run
    simplification_k: usize, // Max amt of genes to attempt removal during one round of simplification process
    simplification_steps: usize, // How many attempts to find simplified genomes
    use_single_thread: bool, // if true, only single threaded
    step_limit: usize,       // Amount of steps a push interpreter can run for
    testing_data: DataFrame, // The testing data, must be formatted the same as training data
    tournament_size: usize,  // Tournament size for tournament selection
    training_data: DataFrame, // The training data, must be formatted the same as testing data
    umad_rate: Decimal,      // addition rate (deletion rate derived) for UMAD
    variation: HashMap<Variation, Decimal>, // genetic operators and probability for use. should sum to 1,
}
