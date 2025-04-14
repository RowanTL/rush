use crate::instructions::*;
use crate::push::interpreter::interpret_program;
use crate::push::state::EMPTY_STATE;

mod instructions;
mod push;

fn main() {
    // These need to stay so linter doesn't go crazy.
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
    exec_instructions();
    code_instructions();
    let mut empty_state = EMPTY_STATE;
    interpret_program(&mut empty_state, 1000, 1000);
}
