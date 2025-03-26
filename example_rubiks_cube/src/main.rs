use meet_in_the_middle::State;
use state::{transition::{Axis, Rotation, Times}, Cube};

mod state;

fn main() {
    let cube = Cube::solved();
    // println!("solved cube:\n{cube}");

    let only_implemented_transition = Rotation::new(Axis::X, state::LineIndex::First, Times::Once);
    
    let rotated = cube.apply(&only_implemented_transition);

    let solution = meet_in_the_middle::find_path(&rotated, &cube).into_iter();
    for step in solution {
        println!("{step}\n----------------------------------------------------------------");
    }
}
