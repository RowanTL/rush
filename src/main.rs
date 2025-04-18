use crate::instructions::*;
use crate::push::interpreter::interpret_program;
use crate::push::state::EMPTY_STATE;

mod instructions;
mod push;

fn main() {
    // These need to stay so linter doesn't go crazy.
    let mut empty_state = EMPTY_STATE;
    interpret_program(&mut empty_state, 1000, 1000);

    let mut counts: Vec<(&str, usize)> = vec![];
    counts.push(("int", 2));
    counts.push(("float", 1));

    // counts.iter().map()
}
