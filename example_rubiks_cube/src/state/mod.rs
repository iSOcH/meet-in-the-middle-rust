use std::{array, fmt::Display};
use core::fmt::Write;

use indenter::{indented, Format};

use meet_in_the_middle::State;
pub use face::{Face, LineId, LineIndex};
use transition::Axis;

pub mod transition;
mod face;

/// We model a rubicks cube like this
/// ```text
///                   Side A
///                A0  A1  A2
///                A3  A4  A5
///                A6  A7  A8
/// 
///      Side B        Side C        Side D        Side E
///  B0  B1  B2    C0  C1  C2    D0  D1  D2    E0  E1  E2         X-Axis (through B and D)
///  B3  B4  B5    C3  C4  C5    D3  D4  D5    E3  E4  E5 ------- 
///  B6  B7  B8    C6  C7  C8    D6  D7  D8    E6  E7  E8         Z-Axis (through C and E)
///                                                
///                   Side F                       
///                F0  F1  F2                      
///                F3  F4  F5                      
///                F6  F7  F8                      
///                     |                          
///                     |                          
///                   Y-Axis (through A and F)
/// ``````
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Cube {
    sides: [Face; 6]
}

impl Cube {
    pub fn new(sides: [Face; 6]) -> Self {
        Cube { sides }
    }

    pub fn solved() -> Self {
        Cube::new(array::from_fn(|i| Face::unicolor((i as u8).try_into().unwrap())))
    }
}

impl State for Cube {
    type Transition = transition::Rotation;

    fn apply(&self, change: &Self::Transition) -> Self {

        // face_move_rotations[0] contains the number of rotations to perform during move from face_move_indices[0] to face_move_indices[1]
        let (mut rotated_face_idx, untouched_face_idx, face_move_indices, face_move_rotations, mut line_source) = match change.axis() {
            Axis::X => (1, 3, [0, 2, 5, 4], [0, 0, 2, 2], LineId::new(face::LineOrientation::Column, false)),
            Axis::Y => (0, 5, [1, 2, 3, 4], [0; 4], LineId::new(face::LineOrientation::Row, true)),
            Axis::Z => (2, 4, [0, 3, 5, 1], [1; 4], LineId::new(face::LineOrientation::Column, false)),
        };

        if change.line_index() == LineIndex::Last {
            rotated_face_idx = untouched_face_idx;
            line_source = line_source.rotate_cw().rotate_cw();
        }

        let rotation_count = change.times() as u8;
        let mut new_cube = self.clone();

        for _ in 0..=rotation_count {
            new_cube.sides[rotated_face_idx] = new_cube.sides[rotated_face_idx].rotate_cw(transition::Times::Once);

            let mut face_src = new_cube.sides[face_move_indices[3]].clone();

            for (side_move_target_idx, &side_target_idx) in face_move_indices.iter().enumerate() {
                let side_tmp = new_cube.sides[side_target_idx].clone();
                let rotations_index = (side_move_target_idx + 4 - 1) % 4; // +4: ensure positive result
                let rotations = face_move_rotations[rotations_index];
                let mut line = line_source.clone();

                for _ in 0..rotations {
                    face_src = face_src.rotate_cw(transition::Times::Once);
                    line = line.rotate_cw();
                }

                // since we rotated face_src already we can use the same line as on the target for copying
                // println!("Copying {line_source:?} to {line:?} on face {side_target_idx}, before:\n{new_cube}");
                new_cube.sides[side_target_idx].set_from_line(&face_src, &line, false);
                // println!("{new_cube}");

                face_src = side_tmp;
                line_source = line;
            }
        }

        new_cube
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

        write!(indented(formatter).with_format(Format::Uniform { indentation }), "{}", self.sides[5])
    }
}

#[cfg(test)]
mod tests {
    use rand::{rngs, Rng, SeedableRng};

    use super::{face::test::random_face, *};

    #[test]
    fn transition_4_times_should_be_noop() {
        let mut rnd = rngs::StdRng::from_seed([0; 32]);

        for _ in 0..100 {
            let initial_cube = random_cube(&mut rnd);

            for transition in &transition::ALL_ROTATIONS {
                let mut rotated = initial_cube.clone();
                for _ in 0..4 {
                    rotated = rotated.apply(transition);
                }
        
                assert_eq!(initial_cube, rotated, "transition: {}", transition);
            } 
        }        
    }

    fn random_cube<TRng: Rng>(rng: &mut TRng) -> Cube {
        let faces = array::from_fn(|_| random_face(rng));
        Cube::new(faces)
    }
}