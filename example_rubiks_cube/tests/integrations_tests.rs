use rand::rngs::StdRng;
use rand::SeedableRng;

use example_rubiks_cube::{cube_with_random_moves, solve_cube, RubiksCube};
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