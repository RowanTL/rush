use super::args::PushArgs;
use crate::push::state::Gene;
use polars::prelude::*;
use rand::Rng;
use rand::prelude::SliceRandom;
use rand::rng;
use rust_decimal::Decimal;
use std::collections::HashSet;

/// Takes k random indices from the given range
fn choose_random_k(k: usize, indices_count: usize, rng: &mut impl Rng) -> Vec<usize> {
    let mut indices: Vec<usize> = (0..indices_count).collect();
    indices.shuffle(rng);
    indices.truncate(k);
    indices
}

/// Deletes the values at the given set of indices
fn delete_at_indices<T: Clone>(indices: &[usize], plushy: &[T]) -> Vec<T> {
    let indices_set: HashSet<usize> = indices.iter().cloned().collect();
    plushy
        .iter()
        .enumerate()
        .filter_map(|(i, item)| {
            if !indices_set.contains(&i) {
                Some(item.clone())
            } else {
                None
            }
        })
        .collect()
}

/// Deletes k random instructions from the plushy
fn delete_k_random<T: Clone>(k: usize, plushy: &[T], rng: &mut impl Rng) -> Vec<T> {
    let actual_k = std::cmp::min(k, plushy.len());
    if actual_k == 0 {
        return plushy.to_vec();
    }
    let indices = choose_random_k(actual_k, plushy.len(), rng);
    delete_at_indices(&indices, plushy)
}

pub fn auto_simplify_plushy<F>(plushy: Vec<Gene>, error_func: F, push_args: PushArgs) -> Vec<Gene>
where
    F: Fn(&PushArgs, &DataFrame, Vec<Gene>) -> Vec<Decimal>,
{
    if push_args.simplification_verbose {
        println!(
            "{{ start_plushy_length: {}, k: {} }}",
            plushy.len(),
            push_args.simplification_k
        );
    }

    let initial_errors = error_func(&push_args, &push_args.training_data, plushy.clone());
    let mut step = 0;
    let mut curr_plushy = plushy;

    while step < push_args.simplification_steps {
        let mut rng = rng();
        let random_k = rng.random_range(1..=push_args.simplification_k);

        let new_plushy = delete_k_random(random_k, &curr_plushy, &mut rng);
        let new_plushy_errors =
            error_func(&push_args, &push_args.training_data, new_plushy.clone());

        if new_plushy_errors.iter().sum::<Decimal>() <= initial_errors.iter().sum() {
            curr_plushy = new_plushy;
        }

        step += 1;
    }

    curr_plushy
}
