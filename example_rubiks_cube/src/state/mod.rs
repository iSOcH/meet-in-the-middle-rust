use std::array;

use meet_in_the_middle::State;
use side::{RubiksSide, RubiksSideIndex};

pub mod transition;
mod side;

/// We model a rubicks cube like this
/// 
///                    Side A
///                 A0  A1  A2
///                 A3  A4  A5
///                 A6  A7  A8
/// 
///      Side B        Side C        Side D        Side E
///   B0  B1  B2    C0  C1  C2    D0  D1  D2    E0  E1  E2         X-Axis (through B and D)
///   B3  B4  B5    C3  C4  C5    D3  D4  D5    E3  E4  E5 ------- 
///   B6  B7  B8    C6  C7  C8    D6  D7  D8    E6  E7  E8         Z-Axis (through C and E)
///                                                 |
///                    Side F                       |
///                 F0  F1  F2                      |
///                 F3  F4  F5                      |
///                 F6  F7  F8                      |
///                      |                          |
///                      |                          |
///                    Y-Axis                     Y-Axis
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct RubiksCubeState {
    sides: [RubiksSide; 6]
}

impl RubiksCubeState {
    pub fn new(sides: [RubiksSide; 6]) -> RubiksCubeState {
        RubiksCubeState { sides }
    }

    pub fn solved() -> RubiksCubeState {
        RubiksCubeState { sides: array::from_fn(|i| RubiksSide::unicolor((i as u8).try_into().unwrap())) }
    }
}

impl State for RubiksCubeState {
    type Transition = transition::RubiksCubeRotation;

    fn apply(&self, change: &Self::Transition) -> Self {
        let change_tuple = (change.axis(), change.row(), change.times());
        
        match change_tuple {
            (transition::Axis::X, transition::Row::First, transition::Times::Once) => {
                let new_b = self.sides[1].rotate_cw(transition::Times::Once);

                // left column moves like A(0,3,6) -> C(0,3,6) -> F(0,3,6) -> E(8,5,2) -> A(0,3,6)
                let mut new_c = self.sides[2].clone();
                new_c.set_from(&self.sides[0], RubiksSideIndex(0));
                new_c.set_from(&self.sides[0], RubiksSideIndex(3));
                new_c.set_from(&self.sides[0], RubiksSideIndex(6));

                let mut new_f = self.sides[5].clone();
                new_f.set_from(&self.sides[2], RubiksSideIndex(0));
                new_f.set_from(&self.sides[2], RubiksSideIndex(3));
                new_f.set_from(&self.sides[2], RubiksSideIndex(6));

                let mut new_e = self.sides[4].clone();
                new_e.set(RubiksSideIndex(8), self.sides[5].get(RubiksSideIndex(0)));
                new_e.set(RubiksSideIndex(5), self.sides[5].get(RubiksSideIndex(3)));
                new_e.set(RubiksSideIndex(2), self.sides[5].get(RubiksSideIndex(6)));

                let mut new_a = self.sides[0].clone();
                new_a.set(RubiksSideIndex(0), self.sides[4].get(RubiksSideIndex(8)));
                new_a.set(RubiksSideIndex(3), self.sides[4].get(RubiksSideIndex(5)));
                new_a.set(RubiksSideIndex(6), self.sides[4].get(RubiksSideIndex(2)));

                let new_d = self.sides[3].clone();

                RubiksCubeState::new([new_a, new_b, new_c, new_d, new_e, new_f])
            },
            _ => todo!()
        }
    }

    fn get_possible_transitions(&self) -> impl Iterator<Item = &Self::Transition> {
        transition::ALL_ROTATIONS.iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::state::transition::{Axis, Row, RubiksCubeRotation, Times};

    use super::*;

    #[test]
    fn transition_4_times_should_be_noop() {
        let initial_cube = RubiksCubeState::solved();
        let transition = RubiksCubeRotation::new(Axis::X, Row::First, Times::Once);
        
        let mut rotated = initial_cube.clone();
        for _ in 0..4 {
            rotated = rotated.apply(&transition);
        }

        assert_eq!(initial_cube, rotated);
    }
}