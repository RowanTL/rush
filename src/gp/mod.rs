use crate::gp::genome::make_random_plushy;
use args::{PushArgs, SearchDirection};
use individual::Individual;
use rust_decimal::dec;
use simplification::auto_simplify_plushy;
use std::iter::repeat;
use variation::new_individual;

pub mod args;
pub mod genome;
pub mod individual;
pub mod selection;
pub mod simplification;
pub mod utils;
pub mod variation;

pub fn gp_loop(push_args: PushArgs) {
    let mut rng = rand::rng();

    // Population is evaluated in the Individual creation.
    let mut pop: Vec<Individual> = repeat(Individual::with_error(
        make_random_plushy(
            push_args.instructions.clone().unwrap(),
            push_args.max_init_plushy_size,
            &mut rng,
        ),
        push_args.error_function.unwrap(),
        &push_args,
        &push_args.training_data.clone().unwrap(),
    ))
    .take(push_args.pop_size)
    .collect::<Vec<_>>();
    // Need to sort it here tho
    match push_args.search_direction {
        SearchDirection::Min => pop.sort_by(|ind0, ind1| {
            ind0.total_fitness
                .unwrap()
                .cmp(&ind1.total_fitness.unwrap())
        }),
        SearchDirection::Max => pop.sort_by(|ind0, ind1| {
            ind0.total_fitness
                .unwrap()
                .cmp(&ind1.total_fitness.unwrap())
                .reverse()
        }),
    }

    let mut generation: usize = 0;
    let mut best_ind = pop[0].clone();

    while generation < push_args.max_generations
        && (&pop[0].total_fitness.unwrap() != &dec!(0.0) || push_args.dont_end)
    {
        // Create new children and evaluate them too
        // May have to remove these repeats later for multithreading.
        // Could probably make own repeat function that does this but
        // multi threaded.
        pop = if push_args.elitism {
            let mut ret_vec = repeat(new_individual(pop, &push_args, &mut rng))
                .take(push_args.pop_size - 1)
                .collect::<Vec<_>>();
            ret_vec.insert(0, best_ind.clone());
            ret_vec
        } else {
            repeat(new_individual(pop, &push_args, &mut rng))
                .take(push_args.pop_size)
                .collect()
        };
        match push_args.search_direction {
            SearchDirection::Min => pop.sort_by(|ind0, ind1| {
                ind0.total_fitness
                    .unwrap()
                    .cmp(&ind1.total_fitness.unwrap())
            }),
            SearchDirection::Max => pop.sort_by(|ind0, ind1| {
                ind0.total_fitness
                    .unwrap()
                    .cmp(&ind1.total_fitness.unwrap())
                    .reverse()
            }),
        }
        best_ind = pop[0].clone();
        generation += 1;

        println!("Generation: {}", generation);
        println!("Best Individual: {:?}", best_ind);
    }

    let simplified_plushy = auto_simplify_plushy(
        best_ind.plushy,
        push_args.error_function.unwrap(),
        &push_args,
    );
    let simplified_ind = Individual::with_error(
        simplified_plushy,
        push_args.error_function.unwrap(),
        &push_args.clone(),
        &push_args.training_data.unwrap(),
    );

    println!("Simplified Best Individual: {:?}", simplified_ind);
}
