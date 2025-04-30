use crate::gp::args::{PushArgs, SearchDirection};
use crate::gp::individual::Individual;
use rand::Rng;
use rand::seq::SliceRandom;

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
        _ => todo!(),
    }
}
