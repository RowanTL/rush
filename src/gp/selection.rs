use crate::gp::args::{PushArgs, SearchDirection};
use crate::gp::individual::Individual;
use crate::gp::utils::absolute_median_deviation;
use rand::Rng;
use rand::seq::{IndexedRandom, SliceRandom};
use rust_decimal::Decimal;

fn tournament_selection(
    mut pop: Vec<Individual>,
    tournament_size: usize,
    direction: SearchDirection,
    rng: &mut impl Rng,
) -> Individual {
    pop.shuffle(rng);
    let mut tournament_set: Vec<Individual> = pop.into_iter().take(tournament_size).collect();
    tournament_set.sort_by(|ind0, ind1| ind0.total_fitness.cmp(&ind1.total_fitness));
    match direction {
        SearchDirection::Min => tournament_set[0].clone(),
        SearchDirection::Max => tournament_set[tournament_set.len() - 1].clone(),
    }
}

// Selects individuals based on individual cases rather than an aggregate value.
// Use Selection::EpsilonLexicase for regression problems.
// Tom Helmuth describing Lexicase: https://youtu.be/Th6Hx3SJOlk
fn lexicase_selection(
    mut pop: Vec<Individual>,
    direction: SearchDirection,
    rng: &mut impl Rng,
) -> Individual {
    let mut cases: Vec<usize> = (0..pop[0].fitness_cases.clone().unwrap().len()).collect();
    cases.shuffle(rng);

    while pop.len() > 1 && cases.len() > 0 {
        let t = cases[cases.len() - 1];
        pop.sort_by(|ind0, ind1| {
            ind0.fitness_cases.clone().unwrap()[t].cmp(&ind1.fitness_cases.clone().unwrap()[t])
        });
        let best: Decimal = match direction {
            SearchDirection::Min => pop[0].fitness_cases.clone().unwrap()[t],
            SearchDirection::Max => pop[pop.len() - 1].fitness_cases.clone().unwrap()[t],
        };
        pop = pop
            .into_iter()
            .filter(|ind| ind.fitness_cases.clone().unwrap()[t] == best)
            .collect();
        cases.pop();
    }
    if pop.len() == 1 {
        return pop[0].clone();
    } else {
        return pop.choose(rng).unwrap().clone();
    }
}

fn epsilon_lexicase_selection(
    mut pop: Vec<Individual>,
    direction: SearchDirection,
    rng: &mut impl Rng,
) -> Individual {
    let mut cases: Vec<usize> = (0..pop[0].fitness_cases.clone().unwrap().len()).collect();
    cases.shuffle(rng);

    while pop.len() > 1 && cases.len() > 0 {
        let t = cases[cases.len() - 1];
        pop.sort_by(|ind0, ind1| {
            ind0.fitness_cases.clone().unwrap()[t].cmp(&ind1.fitness_cases.clone().unwrap()[t])
        });
        let best: Decimal = match direction {
            SearchDirection::Min => pop[0].fitness_cases.clone().unwrap()[t],
            SearchDirection::Max => pop[pop.len() - 1].fitness_cases.clone().unwrap()[t],
        };
        // Contains all values of the specified case from each individual in the population.
        // Tracking which value belongs to who doesn't matter.
        let pop_cases: Vec<Decimal> = pop
            .iter()
            .map(|ind| ind.fitness_cases.clone().unwrap()[t])
            .collect();
        let epsilon = absolute_median_deviation(&pop_cases);
        pop = pop
            .into_iter()
            .filter(|ind| {
                ind.fitness_cases.clone().unwrap()[t] <= best
                    && ind.fitness_cases.clone().unwrap()[t] >= best - epsilon
            })
            .collect();
        cases.pop();
    }
    if pop.len() == 1 {
        return pop[0].clone();
    } else {
        return pop.choose(rng).unwrap().clone();
    }
}

#[derive(Clone)]
pub enum Selection {
    Lexicase,
    EpsilonLexicase,
    Tournament,
}

pub fn select_parent(pop: Vec<Individual>, push_args: &PushArgs, rng: &mut impl Rng) -> Individual {
    match push_args.parent_selection {
        Selection::Tournament => tournament_selection(
            pop,
            push_args.tournament_size,
            push_args.search_direction,
            rng,
        ),
        Selection::Lexicase => lexicase_selection(pop, push_args.search_direction, rng),
        Selection::EpsilonLexicase => {
            epsilon_lexicase_selection(pop, push_args.search_direction, rng)
        }
    }
}
