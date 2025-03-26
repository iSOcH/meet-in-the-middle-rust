use meet_in_the_middle::State;
use state::{transition::{Axis, Row, RubiksCubeRotation, Times}, RubiksCubeState};

mod state;

fn main() {
    let cube = RubiksCubeState::solved();
    println!("solved cube:\n{cube}");

    let only_implemented_transition = RubiksCubeRotation::new(Axis::X, Row::First, Times::Once);
    
    let mut rotated = cube.apply(&only_implemented_transition);
    println!("rotated:\n{rotated}");
    
    rotated = rotated.apply(&only_implemented_transition);
    println!("rotated:\n{rotated}");
    
    rotated = rotated.apply(&only_implemented_transition);
    println!("rotated:\n{rotated}");
    
    rotated = rotated.apply(&only_implemented_transition);
    println!("final:\n{rotated}");
    println!("after 4 identical rotations, same as initially? {}", cube == rotated);
}
