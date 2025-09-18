use crate::gp::args::PushArgs;
use crate::push::state::Gene;
use polars::prelude::*;
use rust_decimal::Decimal;

use super::genome::plushy_to_push;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Individual {
    pub plushy: Vec<Gene>,
    pub push_program: Option<Vec<Gene>>,
    pub total_fitness: Option<Decimal>,
    pub fitness_cases: Option<Vec<Decimal>>,
}

impl fmt::Display for Individual {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!()
    }
}

impl Individual {
    // Creates a new individual based off a plushy. Converts it to a push program
    // and runs the error function on it.
    pub fn with_error<F>(
        plushy: Vec<Gene>,
        error_func: F,
        push_args: &PushArgs,
        data: &DataFrame,
    ) -> Self
    where
        F: Fn(&PushArgs, &DataFrame, Vec<Gene>) -> Vec<Decimal>,
    {
        let push_program = Some(plushy_to_push(plushy.clone()));
        let error_vec = error_func(push_args, data, push_program.clone().unwrap());
        Self {
            plushy,
            push_program,
            total_fitness: Some(error_vec.iter().sum()),
            fitness_cases: Some(error_vec),
        }
    }
}
