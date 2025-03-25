/// Inspired by https://www.youtube.com/watch?v=wL3uWO-KLUE

mod path;
mod state;
mod solver;

pub use state::State;
pub use solver::{find_path, find_nodes_on_path};