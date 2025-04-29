use crate::push::state::Gene;
use rust_decimal::Decimal;

#[derive(Clone)]
pub struct Individual {
    pub plushy: Vec<Gene>,
    pub total_fitness: Option<Decimal>,
    pub fitness_cases: Option<Vec<Decimal>>,
}
