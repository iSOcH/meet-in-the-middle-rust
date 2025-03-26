use std::{array, fmt::Display};
use core::fmt::Write;

use indenter::{indented, Format};

use meet_in_the_middle::State;
use face::Face;

pub mod transition;
mod face;

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
pub struct Cube {
    sides: [Face; 6]
}

impl Cube {
    pub fn new(sides: [Face; 6]) -> Cube {
        Cube { sides }
    }

    pub fn solved() -> Cube {
        Cube { sides: array::from_fn(|i| Face::unicolor((i as u8).try_into().unwrap())) }
    }
}

impl State for Cube {
    type Transition = transition::Rotation;

    fn apply(&self, change: &Self::Transition) -> Self {
        let change_tuple = (change.axis(), change.row(), change.times());
        
        match change_tuple {
            (transition::Axis::X, transition::Row::First, transition::Times::Once) => {
                let new_b = self.sides[1].rotate_cw(transition::Times::Once);

                // left column moves like A(0,3,6) -> C(0,3,6) -> F(0,3,6) -> E(8,5,2) -> A(0,3,6)
                let mut new_c = self.sides[2].clone();
                new_c.set_from(&self.sides[0], 0.try_into().unwrap());
                new_c.set_from(&self.sides[0], 3.try_into().unwrap());
                new_c.set_from(&self.sides[0], 6.try_into().unwrap());

                let mut new_f = self.sides[5].clone();
                new_f.set_from(&self.sides[2], 0.try_into().unwrap());
                new_f.set_from(&self.sides[2], 3.try_into().unwrap());
                new_f.set_from(&self.sides[2], 6.try_into().unwrap());

                let mut new_e = self.sides[4].clone();
                new_e.set(8.try_into().unwrap(), self.sides[5].get(0.try_into().unwrap()));
                new_e.set(5.try_into().unwrap(), self.sides[5].get(3.try_into().unwrap()));
                new_e.set(2.try_into().unwrap(), self.sides[5].get(6.try_into().unwrap()));

                let mut new_a = self.sides[0].clone();
                new_a.set(0.try_into().unwrap(), self.sides[4].get(8.try_into().unwrap()));
                new_a.set(3.try_into().unwrap(), self.sides[4].get(5.try_into().unwrap()));
                new_a.set(6.try_into().unwrap(), self.sides[4].get(2.try_into().unwrap()));

                let new_d = self.sides[3].clone();

                Cube::new([new_a, new_b, new_c, new_d, new_e, new_f])
            },
            _ => todo!()
        }
    }

    fn get_possible_transitions(&self) -> impl Iterator<Item = &Self::Transition> {
        transition::ALL_ROTATIONS.iter()
    }
}

impl Display for Cube {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let indentation = "                 "; // 17 chars

        write!(indented(formatter).with_format(Format::Uniform { indentation }), "{}", self.sides[0])?;

        // without this, indenter somehow affects the next line, breaking the top borders of the second row of sides
        writeln!(formatter, "")?;

        let mut lfrb_strings: [_; 4] = array::from_fn(|_| String::new());

        for (idx, side_str) in lfrb_strings.iter_mut().enumerate() {
            let side = &self.sides[idx+1];
            write!(*side_str, "{}", side)?;
        }

        let lines_l = lfrb_strings[0].split('\n');
        let lines_f = lfrb_strings[1].split('\n');
        let lines_r = lfrb_strings[2].split('\n');
        let lines_b= lfrb_strings[3].split('\n');

        for (((l, f), r), b) in lines_l.zip(lines_f).zip(lines_r).zip(lines_b) {
            writeln!(formatter, "{l}    {f}    {r}    {b}")?;
        }

        writeln!(indented(formatter).with_format(Format::Uniform { indentation }), "{}", self.sides[5])
    }
}

#[cfg(test)]
mod tests {
    use crate::state::transition::{Axis, Row, Rotation, Times};

    use super::*;

    #[test]
    fn transition_4_times_should_be_noop() {
        let initial_cube = Cube::solved();
        let transition = Rotation::new(Axis::X, Row::First, Times::Once);
        
        let mut rotated = initial_cube.clone();
        for _ in 0..4 {
            rotated = rotated.apply(&transition);
        }

        assert_eq!(initial_cube, rotated);
    }
}