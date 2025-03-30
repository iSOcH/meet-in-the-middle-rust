use std::collections::{HashSet, VecDeque};

use crate::State;

pub fn find_path<TState, TTransition>(source: &TState, target: &TState) -> impl IntoIterator<Item = TState> where
    TState : State<Transition = TTransition> {
    let mut nodes_between = find_nodes_on_path(source, target);
    nodes_between.push_front(source.clone());
    nodes_between.push_back(target.clone());
    nodes_between
}

pub fn find_nodes_on_path<TState, TTransition>(source: &TState, target: &TState) -> VecDeque<TState> where
    TState : State<Transition = TTransition> {

    if source == target {
        return VecDeque::new();
    }

    let mut neighbors_of_source = source.get_neighbors();
    if neighbors_of_source.find(|n| n == target).is_some() {
        return VecDeque::new();
    }

    // this might feel weird, but without handling this case as well we can
    // endlessly recurse below. root cause / improvement might be to prevent
    // Solver::run from returning such a result if it was found
    let mut neighbors_of_target = target.get_neighbors();
    if neighbors_of_target.find(|n| n == source).is_some() {
        return VecDeque::new();
    }

    let mut solver = Solver::new(source.clone(), target.clone());
    
    if let Some(node_on_path) = solver.run() {
        let mut solution = VecDeque::new();
        solution.extend(find_nodes_on_path(source, &node_on_path));
        solution.push_back(node_on_path.clone());
        solution.extend(find_nodes_on_path(&node_on_path, target));
        return solution;
    }

    VecDeque::new()
}

pub struct Solver<TState, TTransition> where
    TState : State<Transition = TTransition>,
{
    source: TState,
    target: TState
}

impl<TState, TTransition> Solver<TState, TTransition>
    where TState : State<Transition = TTransition>,
{
    pub fn new(source: TState, target: TState) -> Solver<TState, TTransition> {
        Solver {
            source,
            target,
        }
    }

    pub fn run(&mut self) -> Option<TState> {
        let mut from_source = Discoverer::new(&self.source);
        let mut from_target = Discoverer::new(&self.target);

        loop {
            if let Some(node_found_from_source) = Self::explore(&mut from_source, &from_target, 1) {
                println!("We found a node on the way: {node_found_from_source:?}");
                return Some(node_found_from_source);
            }
            
            if let Some(node_found_from_target) = Self::explore(&mut from_target, &from_source, 1) {
                println!("We found a node on the way: {node_found_from_target:?}");
                return Some(node_found_from_target);
            }
        }
    }

    fn explore(discoverer: &mut Discoverer<TState, TTransition>, other_discoverer: &Discoverer<TState, TTransition>, num_nodes: usize) -> Option<TState> {
        for (new_state, _) in discoverer.take(num_nodes) {
            if other_discoverer.was_seen(&new_state) {
                return Some(new_state);
            }
        }

        None
    }
}

struct Discoverer<TState, TTransition> where
    TState : State<Transition = TTransition>,
{
    explored_states: HashSet<TState>,
    states_to_explore: VecDeque<TState>,
    states_to_explore_next: VecDeque<TState>,
    current_level: u8,
}

impl<TState, TTransition> Discoverer<TState, TTransition> where
    TState : State<Transition = TTransition>,
{
    fn new(source: &TState) -> Discoverer<TState, TTransition> {
        let mut states_to_explore_next = VecDeque::new();
        states_to_explore_next.push_back(source.clone());
        
        let mut explored_states = HashSet::new();
        explored_states.insert(source.clone());

        Discoverer {
            explored_states,
            states_to_explore: VecDeque::new(),
            states_to_explore_next,
            current_level: 0
        }
    }

    fn add_for_later(&mut self, state: TState) {
        self.states_to_explore_next.push_back(state);
    }

    fn was_seen(&self, state: &TState) -> bool {
        self.explored_states.contains(state)
    }
}

impl<TState, TTransition> Iterator for Discoverer<TState, TTransition> where
    TState : State<Transition = TTransition>,
{
    type Item = (TState, u8);

    fn next(&mut self) -> Option<Self::Item> {
        let to_explore = self.states_to_explore.pop_front().unwrap_or_else(|| {
            println!("Finished level {}, seen {} unique states", self.current_level, self.explored_states.len());

            self.current_level += 1;
            self.states_to_explore = std::mem::take(&mut self.states_to_explore_next);
            self.states_to_explore.pop_front().unwrap()
        });

        // breadth-first: remember to explore descendents of `to_explore` after this round
        for t in to_explore.get_possible_transitions() {
            let potential_future_state = to_explore.apply(&t);
            
            let not_yet_visited = self.explored_states.insert(potential_future_state.clone());

            if not_yet_visited {
                self.add_for_later(potential_future_state);
            }
        }

        Some((to_explore, self.current_level))
    }
}
