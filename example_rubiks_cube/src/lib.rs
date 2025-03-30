pub mod state;

use std::sync::OnceLock;

use meet_in_the_middle::{find_path, State};
pub use state::Cube as RubiksCube;

pub fn solve_cube(cube: &RubiksCube) -> impl IntoIterator<Item = RubiksCube> {
    find_path(cube, &get_solved_cube())
}

pub fn solve_cube_with_transitions(cube: &RubiksCube) -> Vec<Step> {
    let mut iterator = find_path(cube, &get_solved_cube()).into_iter();
    let mut result = vec![];

    let mut from = iterator.next().unwrap();

    while let Some(to) = iterator.next() {
        let transition = from.get_possible_transitions().find(|&t| from.apply(t) == to).unwrap().clone();
        result.push(Step { from_state: from, transition });
        from = to;
    }

    result
}

pub fn get_solved_cube() -> &'static RubiksCube {
    static SOLVED_CUBE_MEM: OnceLock<RubiksCube> = OnceLock::new();
    SOLVED_CUBE_MEM.get_or_init(|| {
        RubiksCube::solved()
    })
}

pub struct Step {
    pub from_state: RubiksCube,
    pub transition: state::transition::Rotation,
}