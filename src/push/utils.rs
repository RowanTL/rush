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
