use crate::instructions::list::all_instructions;
use crate::push::state::Gene;

pub fn most_genes() -> Vec<Gene> {
    let mut instructions: Vec<Gene> = all_instructions()
        .into_iter()
        .map(|x| Gene::StateFunc(x))
        .collect();
    instructions.push(Gene::Close);
    instructions.push(Gene::Skip);
    instructions
}

/// Finds and replaces all occurences of a value in list
/// with another value. Returns a new vector. Takes ownership
/// of passed in vector.
pub fn find_replace<T: PartialEq + Clone>(mut vec: Vec<T>, old_val: T, new_val: T) -> Vec<T> {
    for val in vec.iter_mut() {
        if *val == old_val {
            *val = new_val.clone()
        }
    }
    vec
}
