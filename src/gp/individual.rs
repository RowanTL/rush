use crate::gp::args::PushArgs;
use crate::instructions::list::INSTR_NAME_MAP;
use crate::push::state::Gene;
use polars::prelude::*;
use rust_decimal::Decimal;
use std::fmt;

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
        let mut final_string: String = "".to_string();
        for gene in &self.plushy {
            let temp_str: String = match gene {
                Gene::StateFunc(func) => INSTR_NAME_MAP
                    .get(&(func.clone() as usize))
                    .unwrap()
                    .clone(),
                other => format!("{:?}", other),
            };
            final_string.push_str(&temp_str);
            final_string.push_str(" ");
        }
        final_string.push_str("\n------------------------------------------------\n");
        if let Some(program) = &self.push_program {
            for gene in program {
                let temp_str: String = match gene {
                    Gene::StateFunc(func) => INSTR_NAME_MAP
                        .get(&(func.clone() as usize))
                        .unwrap()
                        .clone(),
                    other => format!("{:?}", other), // TODO: Make this for loop a function
                };
                final_string.push_str(&temp_str);
                final_string.push_str(" ");
            }
        } else {
            final_string.push_str("No push program")
        }
        final_string.push_str("\n------------------------------------------------\n");
        if let Some(total_fitness) = &self.total_fitness {
            final_string.push_str(&format!("{:?}", total_fitness))
        } else {
            final_string.push_str("No total fitness")
        }
        final_string.push_str("\n------------------------------------------------\n");
        if let Some(fitness_cases) = &self.fitness_cases {
            final_string.push_str(&format!("{:?}", fitness_cases))
        } else {
            final_string.push_str("No fitness cases")
        }
        write!(f, "{}", final_string)
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
