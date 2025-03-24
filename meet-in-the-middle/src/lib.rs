/// Inspired by https://www.youtube.com/watch?v=wL3uWO-KLUE

mod path;
mod state;

pub use state::State;

use std::{cell::RefCell, collections::{HashSet, VecDeque}};

pub struct Solver<TState, TTransition> where
    TState : State<Transition = TTransition>,
{
    source: TState,
    target: TState,

    explored_states: RefCell<HashSet<TState>>
}

impl<TState, TTransition> Solver<TState, TTransition>
    where TState : State<Transition = TTransition>,
{
    pub fn new(source: TState, target: TState) -> Solver<TState, TTransition> {
        Solver {
            source,
            target,
            explored_states: RefCell::new(HashSet::new()),
        }
    }

    pub fn run(&mut self) {
        // interestingly we need to annotate the argument type of the closure here although we do not (and possibly cannot?) annotate the lifetime of &TState
        // without annotating this, we get
        //
        // Implementation of `Fn` is not general enough
        // closure with signature `fn(&'2 TState) -> bool` must implement `Fn<(&'1 TState,)>`, for any lifetime `'1`...
        // ...but it actually implements `Fn<(&'2 TState,)>`, for some specific lifetime `'2`
        // 
        // when we want to pass it to Discoverer::new. this is really quite weird since vscode even says the inferred type
        // is exacty what we then manually specify.
        let already_explored_binding = |n: &TState| self.explored_states.borrow().contains(n);

        let from_source = Discoverer::new(&self.source, &already_explored_binding);
        // let from_target = Discoverer::new(&self.target);

        for n in from_source {
            if n == self.target {
                println!("reached target");
                break;
            } else {
                // println!("checked {n:?}... ");
                self.explored_states.borrow_mut().insert(n);
            }
        }

        println!("Finished");
    }
}

struct Discoverer<'a, TState, TTransition> where
    TState : State<Transition = TTransition>,
{
    source: &'a TState,
    states_to_explore: VecDeque<TState>,

    // a reference to something implementing Fn(&TState) -> bool
    // the outer reference shares lifetime with this struct
    // while TState passed to the Fn must accept an independent (much shorter) lifetime
    // "for<'b>" is called a "Higher-Ranked Trait Bound", here it basically says "for any lifetime 'b ..."
    // the parentheses would not be needed
    already_seen: &'a dyn (for<'b> Fn(&'b TState) -> bool)
}

impl<'a, TState, TTransition> Discoverer<'a, TState, TTransition> where
    TState : State<Transition = TTransition>,
{
    fn new(source: &'a TState, already_seen: &'a dyn for<'b> Fn(&'b TState) -> bool) -> Discoverer<'a, TState, TTransition> {
        let initial_transitions = source.get_possible_transitions();
        Discoverer {
            source: source,
            states_to_explore: initial_transitions.iter().map(|t| source.apply(t)).collect(),
            already_seen
        }
    }

    fn add_for_later(&mut self, state: TState) {
        self.states_to_explore.push_back(state);
    }
}

impl<'a, TState, TTransition> Iterator for Discoverer<'a, TState, TTransition> where
    TState : State<Transition = TTransition>,
{
    type Item = TState;

    fn next(&mut self) -> Option<Self::Item> {
        let to_explore = loop {
            let potential_next = self.states_to_explore.pop_front()?;
            if !(self.already_seen)(&potential_next) {
                break Some(potential_next);
            }
        }?;

        if (self.already_seen)(&to_explore) {
            return None;
        }

        // breadth-first: remember to explore descendents of `to_explore` after this round
        for t in to_explore.get_possible_transitions() {
            let potential_future_state = to_explore.apply(&t);
            
            let was_already_seen = (self.already_seen)(&potential_future_state);

            if !was_already_seen {
                self.add_for_later(potential_future_state);
            }
        }

        Some(to_explore)
    }
}