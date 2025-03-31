pub mod state;

use std::{collections::HashSet, sync::OnceLock};

use meet_in_the_middle::{find_path, State};
use rand::{seq::IteratorRandom, Rng};
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

pub fn cube_with_random_moves<TRng: Rng>(rng: &mut TRng, move_count: u8) -> RubiksCube {
    let mut cube = RubiksCube::solved();
    
    let mut seen_cubes = HashSet::new();
    seen_cubes.insert(cube.clone());

    let mut last_rotation: Option<state::transition::Rotation> = None;
    
    for _ in 0..move_count {
        loop {
            let transition = cube
                .get_possible_transitions()
                .filter(|&r| {
                    // picking the same face again would result in 2 subsequent moves which could have been done in a single step
                    last_rotation.map(|lr| r.axis() != lr.axis() && r.line_index() != lr.line_index()).unwrap_or(true)
                })
                .choose(rng)
                .unwrap()
                .clone();
            
            let modified_cube = cube.apply(&transition);

            if seen_cubes.insert(modified_cube.clone()) {
                cube = modified_cube;
                last_rotation = Some(transition);
                break;
            }
        }
    }

    cube
}