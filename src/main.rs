use crate::instructions::list::*;
use crate::push::interpreter::interpret_program;
use crate::push::state::EMPTY_STATE;
use push::utils::most_genes;

mod instructions;
mod push;

fn main() {
    let tvec = vec![1, 2, 3, 4, 5];
    println!("{:?}", tvec);

    // These need to stay so linter doesn't go crazy.
    let mut empty_state = EMPTY_STATE;
    empty_state.int = vec![1, 2, 3];
    interpret_program(&mut empty_state, 1000, 1000);

    int_instructions();
    float_instructions();
    string_instructions();
    boolean_instructions();
    char_instructions();
    vector_int_instructions();
    vector_float_instructions();
    vector_string_instructions();
    vector_boolean_instructions();
    vector_char_instructions();
    code_instructions();
    exec_instructions();
    all_instructions();
    most_genes();
}
