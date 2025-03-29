use std::fmt::Display;

use super::face::LineIndex as Index;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Rotation(u8);

pub static ALL_ROTATIONS: [Rotation; 18] = [
    Rotation::new(Axis::X, Index::First, Times::Once),
    Rotation::new(Axis::X, Index::First, Times::Twice),
    Rotation::new(Axis::X, Index::First, Times::Thrice),
    Rotation::new(Axis::X, Index::Last, Times::Once),
    Rotation::new(Axis::X, Index::Last, Times::Twice),
    Rotation::new(Axis::X, Index::Last, Times::Thrice),

    Rotation::new(Axis::Y, Index::First, Times::Once),
    Rotation::new(Axis::Y, Index::First, Times::Twice),
    Rotation::new(Axis::Y, Index::First, Times::Thrice),
    Rotation::new(Axis::Y, Index::Last, Times::Once),
    Rotation::new(Axis::Y, Index::Last, Times::Twice),
    Rotation::new(Axis::Y, Index::Last, Times::Thrice),

    Rotation::new(Axis::Z, Index::First, Times::Once),
    Rotation::new(Axis::Z, Index::First, Times::Twice),
    Rotation::new(Axis::Z, Index::First, Times::Thrice),
    Rotation::new(Axis::Z, Index::Last, Times::Once),
    Rotation::new(Axis::Z, Index::Last, Times::Twice),
    Rotation::new(Axis::Z, Index::Last, Times::Thrice),
];

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    /// Through B and D
    X = 0b00,  // 2 bits for 3 axes (X, Y, Z)

    /// Through A and F
    Y = 0b01,

    /// Through C and E
    Z = 0b10,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Times {
    Once   = 0b00,  // 2 bits for 3 rotation counts
    Twice  = 0b01,
    Thrice = 0b10,
}

impl Rotation {
    const AXIS_MASK: u8  = 0b1100_0000;
    const ROW_MASK: u8   = 0b0010_0000;
    const TIMES_MASK: u8 = 0b0001_1000;

    pub const fn new(axis: Axis, idx: Index, times: Times) -> Self {
        let value = (axis as u8) << 6 | (idx as u8) << 5 | (times as u8) << 3;
        Rotation(value)
    }

    pub fn axis(&self) -> Axis {
        match (self.0 & Self::AXIS_MASK) >> 6 {
            0b00 => Axis::X,
            0b01 => Axis::Y,
            0b10 => Axis::Z,
            _ => unreachable!(),
        }
    }

    pub fn line_index(&self) -> Index {
        if (self.0 & Self::ROW_MASK) != 0 {
            Index::Last
        } else {
            Index::First
        }
    }

    pub fn times(&self) -> Times {
        match (self.0 & Self::TIMES_MASK) >> 3 {
            0b00 => Times::Once,
            0b01 => Times::Twice,
            0b10 => Times::Thrice,
            _ => unreachable!(),
        }
    }
}

impl Display for Rotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rotation({:?}, {:?}, {:?})", self.axis(), self.line_index(), self.times())
    }
}