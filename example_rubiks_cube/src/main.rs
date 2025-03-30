use example_rubiks_cube::{solve_cube, solve_cube_with_transitions, state::{transition::{Axis::*, Rotation, Times::*}, LineIndex::*}, RubiksCube};
use meet_in_the_middle::State;

fn main() {
    // 0: white,
    // 1: orange,
    // 2: green,
    // 3: red,
    // 4: blue,
    // 5: yellow

    let cube = RubiksCube::from_unvalidated_raw_colors(&[
        [4, 0, 1, 1, 0, 5, 2, 4, 2],
        [1, 2, 3, 1, 1, 5, 4, 3, 3],
        [0, 3, 0, 1, 2, 0, 5, 4, 4],
        [1, 3, 0, 1, 3, 0, 5, 0, 1],
        [4, 2, 5, 4, 4, 4, 2, 2, 0],
        [2, 5, 3, 2, 5, 3, 3, 5, 5]
    ]).unwrap();

    // let cube_copied = RubiksCube::from_unvalidated_raw_colors(&[
    //    [0, 0, 0, 0, 0, 0, 1, 1, 5],
    //    [1, 1, 5, 1, 1, 5, 1, 1, 4],
    //    [2, 2, 3, 2, 2, 3, 5, 5, 2],
    //    [2, 3, 3, 2, 3, 2, 1, 3, 2],
    //    [4, 4, 4, 0, 4, 4, 0, 4, 4],
    //    [3, 3, 0, 5, 5, 4, 5, 5, 3]
    // ]).unwrap();

    // let cube = RubiksCube::solved()
    //     .apply(&Rotation::new(Y, First, Thrice))
    //     .apply(&Rotation::new(X, Last, Thrice))
    //     .apply(&Rotation::new(Y, First, Once))
    //     .apply(&Rotation::new(Z, First, Once));

    // assert_eq!(cube, cube_copied);

    // let cube = RubiksCube::from_unvalidated_raw_colors(&[
    //     [0, 0, 3, 0, 0, 3, 3, 0, 3],
    //     [1, 1, 2, 2, 1, 1, 2, 1, 1],
    //     [5, 3, 5, 2, 2, 0, 2, 2, 0],
    //     [4, 4, 4, 2, 3, 5, 2, 3, 5],
    //     [0, 4, 4, 4, 4, 3, 4, 4, 3],
    //     [5, 5, 1, 5, 5, 5, 0, 1, 1]
    // ]).unwrap();

    // // needs 10 moves
    // let cube = RubiksCube::from_unvalidated_raw_colors(&[
    //     [2, 2, 2, 0, 0, 3, 0, 0, 4],
    //     [5, 3, 1, 1, 1, 1, 1, 2, 2],
    //     [2, 2, 1, 5, 2, 1, 0, 2, 5],
    //     [0, 4, 5, 0, 3, 0, 3, 3, 3],
    //     [3, 1, 1, 4, 4, 4, 0, 4, 4],
    //     [3, 5, 4, 3, 5, 5, 5, 5, 4]
    // ]).unwrap();

    let solution: Vec<_> = solve_cube_with_transitions(&cube);
    for step in solution {
        println!("{}\n----------------------------------------------------------------", step.from_state);
        println!("Apply {}", step.transition);
    }
    println!("{}", RubiksCube::solved());
}
