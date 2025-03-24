use std::fmt::Debug;
use std::hash::Hash;

pub trait State : Hash + Eq + Debug {
    type Transition;

    fn apply(&self, change: &Self::Transition) -> Self;

    fn get_possible_transitions(&self) -> Vec<Self::Transition>;
}
