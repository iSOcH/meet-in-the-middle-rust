use std::fmt::{Debug, Display};

use super::transition;
pub use color::Color;
pub use index::Index as FaceIndex;

pub use line::{Id as LineId, Index as LineIndex, Orientation as LineOrientation};

mod index;
pub mod color;
mod line;

#[derive(Clone, PartialEq, Eq, Hash)]
// 9 * 6 colors = 9 * 3 bits = 27 bits
pub struct Face(u32);

impl Face {
    pub fn new(colors: [Color; 9]) -> Face {
        let mut value = 0u32;
        
        for logical_index in 0..9usize {
            let face_index = FaceIndex::try_from(logical_index as u8).unwrap();
            let shift = face_index.to_shift();
            value |= (u32::from(colors[logical_index])) << shift;
        }

        Face(value)
    }

    pub fn unicolor(color: Color) -> Face {
        Self::new([color; 9])
    }

    pub fn get(&self, index: FaceIndex) -> Color {
        let mask: u8 = 0b111;
        let shift = index.to_shift();
        let bits = (self.0 >> shift) as u8 & mask;
        bits.try_into().unwrap()
    }

    pub fn set(&mut self, index: FaceIndex, value: Color) {
        let shift = index.to_shift();

        let clear_stamp: u32 = !(0b111 << shift);
        let value_stamp: u32 = (u32::from(value)) << shift;

        self.0 = (self.0 & clear_stamp) | value_stamp;
    }

    pub fn set_from_line(&mut self, source: &Face, line: &line::Id, mirror: bool) {
        let source_indices = line.indices();
        let target_indices = if mirror { line.mirrored().indices() } else { source_indices };

        for (idx_s, idx_d) in source_indices.into_iter().zip(target_indices) {
            let color = source.get(idx_s);
            self.set(idx_d, color);
        }
    }

    pub fn rotate_cw(&self, times: transition::Times) -> Face {
        let mut new_side = self.clone();
        
        match times {
            transition::Times::Once => {
                new_side.set(0.try_into().unwrap(), self.get(6.try_into().unwrap()));
                new_side.set(1.try_into().unwrap(), self.get(3.try_into().unwrap()));
                new_side.set(2.try_into().unwrap(), self.get(0.try_into().unwrap()));

                new_side.set(3.try_into().unwrap(), self.get(7.try_into().unwrap()));
                new_side.set(5.try_into().unwrap(), self.get(1.try_into().unwrap()));

                new_side.set(6.try_into().unwrap(), self.get(8.try_into().unwrap()));
                new_side.set(7.try_into().unwrap(), self.get(5.try_into().unwrap()));
                new_side.set(8.try_into().unwrap(), self.get(2.try_into().unwrap()));
            },
            transition::Times::Twice => {
                for target_idx in 0..9 {
                    let source_idx = 8 - target_idx;
                    new_side.set(target_idx.try_into().unwrap(), self.get(source_idx.try_into().unwrap()));
                }
            },
            transition::Times::Thrice => {
                new_side.set(0.try_into().unwrap(), self.get(2.try_into().unwrap()));
                new_side.set(1.try_into().unwrap(), self.get(5.try_into().unwrap()));
                new_side.set(2.try_into().unwrap(), self.get(8.try_into().unwrap()));

                new_side.set(3.try_into().unwrap(), self.get(1.try_into().unwrap()));
                new_side.set(5.try_into().unwrap(), self.get(7.try_into().unwrap()));

                new_side.set(6.try_into().unwrap(), self.get(0.try_into().unwrap()));
                new_side.set(7.try_into().unwrap(), self.get(3.try_into().unwrap()));
                new_side.set(8.try_into().unwrap(), self.get(6.try_into().unwrap()));
            },
        }

        new_side
    }
}

impl Debug for Face {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut builder = f.debug_list();

        for i in 0..9 {
            let entry: u8 = self.get(i.try_into().unwrap()).into();
            builder.entry(&entry);
        }

        builder.finish()
    }
}

impl Display for Face {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "+-----------+")?;
        writeln!(f, "| {} | {} | {} |", u8::from(self.get(0.try_into().unwrap())), u8::from(self.get(1.try_into().unwrap())), u8::from(self.get(2.try_into().unwrap())))?;
        writeln!(f, "|---+---+---|")?;
        writeln!(f, "| {} | {} | {} |", u8::from(self.get(3.try_into().unwrap())), u8::from(self.get(4.try_into().unwrap())), u8::from(self.get(5.try_into().unwrap())))?;
        writeln!(f, "|---+---+---|")?;
        writeln!(f, "| {} | {} | {} |", u8::from(self.get(6.try_into().unwrap())), u8::from(self.get(7.try_into().unwrap())), u8::from(self.get(8.try_into().unwrap())))?;
        write!(f, "+-----------+")
    }
}

#[cfg(test)]
pub(crate) mod test {
    use std::array;
    use rand::{rngs, SeedableRng};

    use crate::state::transition::Times;

    use super::*;

    #[test]
    fn set_cell() {
        for idx in 0..9 {
            let mut mutated = indexed_side();
            let rubiks_index = idx.try_into().unwrap();
            let written = ((idx + 1) % 6).try_into().unwrap();
            
            mutated.set(rubiks_index, written);

            let read = mutated.get(rubiks_index);
            assert_eq!(read, written);
        }
        
    }

    #[test]
    fn rotate_cw_once() {
        let initial_side = indexed_side();
        let rotated = initial_side.rotate_cw(transition::Times::Once);

        let expected = Face::new([
            0.try_into().unwrap(),
            3.try_into().unwrap(),
            0.try_into().unwrap(),

            1.try_into().unwrap(),
            4.try_into().unwrap(),
            1.try_into().unwrap(),

            2.try_into().unwrap(),
            5.try_into().unwrap(),
            2.try_into().unwrap(),
        ]);

        assert_eq!(expected, rotated);
    }

    #[test]
    fn rotate_cw_4_times_should_be_noop() {
        for times in [Times::Once, Times::Twice, Times::Thrice] {
            let initial_side = indexed_side();

            let mut rotated = initial_side.clone();
    
            for _ in 0..4 {
                rotated = rotated.rotate_cw(times);
            }
    
            assert_eq!(initial_side, rotated);
        }
    }

    #[test]
    fn rotate_cw_twice_2_times_should_be_noop() {
        let initial_side = indexed_side();

        let mut rotated = initial_side.clone();

        for _ in 0..2 {
            rotated = rotated.rotate_cw(Times::Twice);
        }

        assert_eq!(initial_side, rotated);
    }

    #[test]
    fn rotate_cw_multiply() {
        let mut rng = rngs::StdRng::seed_from_u64(42);

        for _ in 0..10 {
            let face = random_face(&mut rng);
    
            let once = face.rotate_cw(Times::Once);

            let twice = once.rotate_cw(Times::Once);
            assert_eq!(twice, face.rotate_cw(Times::Twice));

            let thrice = twice.rotate_cw(Times::Once);
            assert_eq!(thrice, face.rotate_cw(Times::Thrice));
        }
    }

    fn indexed_side() -> Face {
        let colors = array::from_fn(|i| ((i % 6) as u8).try_into().unwrap());
        let side = Face::new(colors);
        side
    }

    pub(crate) fn random_face<TRand: rand::Rng>(rng: &mut TRand) -> Face {
        let colors = array::from_fn(|_| rng.random_range(0..=5)).map(|r| Color::try_from(r).unwrap());
        Face::new(colors)
    }
}
