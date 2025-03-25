use meet_in_the_middle::State;
use transition::RubiksCubeRotation;

/// We model a rubicks cube like this
/// 
///                    Side A
///                 A0  A1  A2
///                 A3  A4  A5
///                 A6  A7  A8
/// 
///      Side B        Side C        Side D        Side E
///   B0  B1  B2    C0  C1  C2    D0  D1  D2    E0  E1  E2
///   B3  B4  B5    C3  C4  C5    D3  D4  D5    E3  E4  E5
///   B6  B7  B8    C6  C7  C8    D6  D7  D8    E6  E7  E8
/// 
///                    Side F
///                 F0  F1  F2
///                 F3  F4  F5
///                 F6  F7  F8
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct RubiksCubeState {
    
}

impl State for RubiksCubeState {
    type Transition = transition::RubiksCubeRotation;

    fn apply(&self, change: &Self::Transition) -> Self {
        todo!()
    }

    fn get_possible_transitions(&self) -> impl Iterator<Item = &Self::Transition> {
        transition::ALL_ROTATIONS.iter()
    }
}

mod transition {
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
}
