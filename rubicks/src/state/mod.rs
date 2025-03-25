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
/// 
/// For the transitions side 0 is top and side 2 faces the user,
/// x axis goes through 1 and 3, y through 0 and 5 and z through 2 and 4
struct RubiksCubeState {
    
}

impl State for RubiksCubeState {
    type Transition = RubiksCubeRotation;

    fn apply(&self, change: &Self::Transition) -> Self {
        todo!()
    }

    fn get_possible_transitions(&self) -> impl Iterator<Item = &Self::Transition> {
        todo!()
    }
}

mod transition {
    // there are "only" 18 possibilities which should fit in 5 bits, but this uses 3 bytes. quite wasteful :/
    pub struct RubiksCubeRotation {
        axis: Axis,
        row: Row,
        times: Times
    }

    enum Axis {
        X,
        Y,
        Z
    }

    enum Row {
        /// move which always affects C0
        First,
        
        /// move which always affects C8
        Last
    }

    enum Times {
        Once,
        Twice,
        Thrice
    }
}