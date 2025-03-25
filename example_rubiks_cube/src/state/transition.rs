#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct RubiksCubeRotation(u8);

pub static ALL_ROTATIONS: [RubiksCubeRotation; 18] = [
    RubiksCubeRotation::new(Axis::X, Row::First, Times::Once),
    RubiksCubeRotation::new(Axis::X, Row::First, Times::Twice),
    RubiksCubeRotation::new(Axis::X, Row::First, Times::Thrice),
    RubiksCubeRotation::new(Axis::X, Row::Last, Times::Once),
    RubiksCubeRotation::new(Axis::X, Row::Last, Times::Twice),
    RubiksCubeRotation::new(Axis::X, Row::Last, Times::Thrice),

    RubiksCubeRotation::new(Axis::Y, Row::First, Times::Once),
    RubiksCubeRotation::new(Axis::Y, Row::First, Times::Twice),
    RubiksCubeRotation::new(Axis::Y, Row::First, Times::Thrice),
    RubiksCubeRotation::new(Axis::Y, Row::Last, Times::Once),
    RubiksCubeRotation::new(Axis::Y, Row::Last, Times::Twice),
    RubiksCubeRotation::new(Axis::Y, Row::Last, Times::Thrice),

    RubiksCubeRotation::new(Axis::Z, Row::First, Times::Once),
    RubiksCubeRotation::new(Axis::Z, Row::First, Times::Twice),
    RubiksCubeRotation::new(Axis::Z, Row::First, Times::Thrice),
    RubiksCubeRotation::new(Axis::Z, Row::Last, Times::Once),
    RubiksCubeRotation::new(Axis::Z, Row::Last, Times::Twice),
    RubiksCubeRotation::new(Axis::Z, Row::Last, Times::Thrice),
];

#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum Axis {
    /// Through B and D
    X = 0b00,  // 2 bits for 3 axes (X, Y, Z)

    /// Through A and F
    Y = 0b01,

    /// Through C and E
    Z = 0b10,
}

#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum Row {
    /// move which always affects C0
    First = 0b0,  // 1 bit for 2 rows

    /// move which always affects C8
    Last  = 0b1,
}

#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum Times {
    Once   = 0b00,  // 2 bits for 3 rotation counts
    Twice  = 0b01,
    Thrice = 0b10,
}

impl RubiksCubeRotation {
    const AXIS_MASK: u8  = 0b1100_0000;
    const ROW_MASK: u8   = 0b0010_0000;
    const TIMES_MASK: u8 = 0b0001_1000;

    pub const fn new(axis: Axis, row: Row, times: Times) -> Self {
        let value = (axis as u8) << 6 | (row as u8) << 5 | (times as u8) << 3;
        RubiksCubeRotation(value)
    }

    pub fn axis(&self) -> Axis {
        match (self.0 & Self::AXIS_MASK) >> 6 {
            0b00 => Axis::X,
            0b01 => Axis::Y,
            0b10 => Axis::Z,
            _ => unreachable!(),
        }
    }

    pub fn row(&self) -> Row {
        if (self.0 & Self::ROW_MASK) != 0 {
            Row::Last
        } else {
            Row::First
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
