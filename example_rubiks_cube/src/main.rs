use example_rubiks_cube::{solve_cube, state::{transition::{Axis::*, Rotation, Times::*}, LineIndex::*}, RubiksCube};
use meet_in_the_middle::State;

fn main() {
    // 0: white,
    // 1: yellow,
    // 2: red,
    // 3: green,
    // 4: orange,
    // 5: blue

    // let cube = RubiksCube::from_unvalidated_raw_colors(&[
    //    [0, 1, 0, 2, 0, 3, 0, 4, 4],
    //    [3, 5, 3, 1, 4, 4, 2, 5, 3],
    //    [2, 0, 5, 3, 3, 1, 2, 0, 0],
    //    [1, 0, 5, 2, 2, 3, 5, 4, 4],
    //    [4, 4, 4, 2, 5, 5, 3, 3, 1],
    //    [1, 2, 2, 0, 1, 5, 5, 1, 1],
    // ]).unwrap();

    // should only take 3 moves
    // let cube = RubiksCube::from_unvalidated_raw_colors(&[
    //    [0, 0, 4, 0, 0, 0, 0, 0, 0],
    //    [4, 4, 4, 4, 4, 4, 3, 3, 1],
    //    [3, 3, 3, 3, 3, 3, 2, 2, 3],
    //    [2, 2, 5, 2, 2, 5, 2, 2, 0],
    //    [1, 5, 5, 1, 5, 5, 5, 4, 4],
    //    [5, 5, 1, 1, 1, 1, 1, 1, 2],
    // ]).unwrap();

    let mut cube = RubiksCube::solved();
    cube = cube.apply(&Rotation::new(X, First, Thrice));
    println!("{cube}");
    cube = cube.apply(&Rotation::new(X, First, Thrice));
    println!("{cube}");
    cube = cube.apply(&Rotation::new(X, First, Thrice));
    println!("{cube}");
    cube = cube.apply(&Rotation::new(X, First, Thrice));
    println!("{cube}");

    // println!("unsolved:\n{cube}");

    // let solution = solve_cube(&cube);
    // for step in solution {
    //     println!("{step}\n----------------------------------------------------------------");
    // }
}
