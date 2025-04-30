use crate::gp::args::PushArgs;
use crate::gp::individual::Individual;
use rand::Rng;

pub enum Selection {
    Lexicase,
    Tournament,
}

pub fn select_parent(
    _pop: Vec<Individual>,
    _push_args: &PushArgs,
    _rng: &mut impl Rng,
) -> Individual {
    todo!()
}
