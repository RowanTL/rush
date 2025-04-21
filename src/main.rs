use crate::push::interpreter::interpret_program;
use crate::push::state::EMPTY_STATE;

mod instructions;
mod push;

fn main() {
    // These need to stay so linter doesn't go crazy.
    let mut empty_state = EMPTY_STATE;
    interpret_program(&mut empty_state, 1000, 1000);
}
