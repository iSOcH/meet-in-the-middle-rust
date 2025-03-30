use std::collections::HashSet;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use rand::seq::IteratorRandom;

use example_rubiks_cube::{solve_cube, RubiksCube};
use meet_in_the_middle::State;

#[test]
fn can_solve_2_move() {
    let mut rng = StdRng::from_seed([0; 32]);

    for _ in 0..10 {
        can_solve_n_moves(&mut rng, 2);
    }
}

#[test]
fn can_solve_4_moves() {
    let mut rng = StdRng::from_seed([0; 32]);

    for _ in 0..10 {
        can_solve_n_moves(&mut rng, 4);
    }
}

#[test]
fn can_solve_7_moves() {
    let mut rng = StdRng::from_seed([0; 32]);

    for _ in 0..10 {
        can_solve_n_moves(&mut rng, 7);
    }
}

fn can_solve_n_moves(rng: &mut StdRng, move_count: u8) {
    let unsolved_cube = cube_with_random_moves(rng, move_count);
    let solution: Vec<_> = solve_cube(&unsolved_cube).into_iter().collect();

    let max_solution_nodes = move_count as usize + 1;
    assert!(solution.len() <= max_solution_nodes, "There should have been a solution with at most {max_solution_nodes} nodes for state\n{unsolved_cube}");

    assert_eq!(solution[0], unsolved_cube);
    assert_eq!(*solution.last().unwrap(), RubiksCube::solved());

    let mut steps = solution.windows(2);
    while let Some([from, to]) = steps.next() {
        assert!(from.get_neighbors().any(|n| n == *to), "{from}\n{to}");
    }
}

fn cube_with_random_moves<TRng: Rng>(rng: &mut TRng, move_count: u8) -> RubiksCube {
    let mut cube = RubiksCube::solved();
    
    let mut seen_cubes = HashSet::new();
    seen_cubes.insert(cube.clone());
    
    for _ in 0..move_count {
        loop {
            let transition = cube.get_possible_transitions().choose(rng).unwrap();
            
            let modified_cube = cube.apply(transition);
            if seen_cubes.insert(modified_cube.clone()) {
                cube = modified_cube;
                break;
            }
        }
    }

    cube
}