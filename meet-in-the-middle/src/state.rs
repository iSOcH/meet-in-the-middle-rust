use std::fmt::Debug;
use std::hash::Hash;

pub trait State : Hash + Eq + Debug + Clone {
    type Transition;

    fn apply(&self, change: &Self::Transition) -> Self;

    fn get_possible_transitions(&self) -> impl Iterator<Item = &Self::Transition>;

    fn get_neighbors(&self) -> impl Iterator<Item = Self> {
        self.get_possible_transitions().into_iter().map(|t| self.apply(&t))
    }
}
