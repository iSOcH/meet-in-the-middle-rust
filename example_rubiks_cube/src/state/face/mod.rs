use std::fmt::{Debug, Display};

use super::transition;
pub use color::Color;
pub use index::Index as FaceIndex;

mod index;
mod color;

#[derive(Clone, PartialEq, Eq, Hash)]
// 9 * 6 colors = 9 * 3 bits = 27 bits
pub struct Face(u32);

impl Face {
    pub fn new(colors: [Color; 9]) -> Face {
        let mut value = 0u32;
        
        for logical_index in 0..9usize {
            let face_index = FaceIndex(logical_index as u8);
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

    pub fn set_from(&mut self, source: &Face, index: FaceIndex) {
        self.set(index, source.get(index));
    }

    pub fn rotate_cw(&self, times: transition::Times) -> Face {
        let mut new_side = self.clone();
        
        match times {
            transition::Times::Once => {
                new_side.set(FaceIndex(0), self.get(FaceIndex(6)));
                new_side.set(FaceIndex(1), self.get(FaceIndex(3)));
                new_side.set(FaceIndex(2), self.get(FaceIndex(0)));

                new_side.set(FaceIndex(3), self.get(FaceIndex(7)));
                new_side.set(FaceIndex(5), self.get(FaceIndex(1)));

                new_side.set(FaceIndex(6), self.get(FaceIndex(8)));
                new_side.set(FaceIndex(7), self.get(FaceIndex(5)));
                new_side.set(FaceIndex(8), self.get(FaceIndex(2)));
            },
            _ => todo!()
        }

        new_side
    }
}

impl Debug for Face {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut builder = f.debug_list();

        for i in 0..9 {
            let entry: u8 = self.get(FaceIndex(i)).into();
            builder.entry(&entry);
        }

        builder.finish()
    }
}

impl Display for Face {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "+-----------+")?;
        writeln!(f, "| {} | {} | {} |", u8::from(self.get(FaceIndex(0))), u8::from(self.get(FaceIndex(1))), u8::from(self.get(FaceIndex(2))))?;
        writeln!(f, "|---+---+---|")?;
        writeln!(f, "| {} | {} | {} |", u8::from(self.get(FaceIndex(3))), u8::from(self.get(FaceIndex(4))), u8::from(self.get(FaceIndex(5))))?;
        writeln!(f, "|---+---+---|")?;
        writeln!(f, "| {} | {} | {} |", u8::from(self.get(FaceIndex(6))), u8::from(self.get(FaceIndex(7))), u8::from(self.get(FaceIndex(8))))?;
        writeln!(f, "+-----------+")
    }
}

#[cfg(test)]
mod test {
    use std::array;
    use super::*;

    #[test]
    fn set_cell() {
        for idx in 0..9 {
            let mut mutated = indexed_side();
            let rubiks_index = FaceIndex(idx);
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
        let initial_side = indexed_side();

        let mut rotated = initial_side.clone();

        for _ in 0..4 {
            rotated = rotated.rotate_cw(transition::Times::Once);
        }

        assert_eq!(initial_side, rotated);
    }

    fn indexed_side() -> Face {
        let colors = array::from_fn(|i| ((i % 6) as u8).try_into().unwrap());
        let side = Face::new(colors);
        side
    }
}
