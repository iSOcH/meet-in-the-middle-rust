pub mod state;

use std::sync::OnceLock;

use meet_in_the_middle::find_path;
pub use state::Cube as RubiksCube;

pub fn solve_cube(cube: &RubiksCube) -> impl IntoIterator<Item = RubiksCube> {
    find_path(cube, &get_solved_cube())
}

pub fn get_solved_cube() -> &'static RubiksCube {
    static SOLVED_CUBE_MEM: OnceLock<RubiksCube> = OnceLock::new();
    SOLVED_CUBE_MEM.get_or_init(|| {
        RubiksCube::solved()
    })
}