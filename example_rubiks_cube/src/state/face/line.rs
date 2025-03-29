use std::array;

use super::FaceIndex;

#[derive(Debug, Clone, PartialEq)]
pub struct Id {
    orientation: Orientation,
    index: Index,
    mirrored: bool
}

impl Id {
    pub fn new(orientation: Orientation, index_first: bool) -> Id {
        Id {
            orientation,
            index: if index_first { Index::First } else { Index::Last },
            mirrored: false,
        }
    }

    pub fn mirrored(&self) -> Id {
        Id {
            mirrored: !self.mirrored,
            ..self.clone()
        }
    }

    pub fn rotate_cw(&self) -> Id {
        let next_index = match (&self.orientation, &self.index) {
            (Orientation::Row, Index::First) => Index::Last,
            (Orientation::Column, Index::Last) => Index::Last,
            (Orientation::Row, Index::Last) => Index::First,
            (Orientation::Column, Index::First) => Index::First,
        };

        Id {
            orientation: self.orientation.other(),
            index: next_index,
            mirrored: self.mirrored,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    Row,
    Column
}

impl Orientation {
    fn other(&self) -> Orientation {
        match self {
            Orientation::Row => Orientation::Column,
            Orientation::Column => Orientation::Row,
        }
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Index {
    /// move which always affects C0
    First = 0b0,  // 1 bit for 2 rows

    /// move which always affects C8
    Last  = 0b1,
}

type Indices = [FaceIndex; 3];

impl Id {
    pub fn indices(&self) -> Indices {
        let index_offset: u8 = match (self.index, self.orientation) {
            (Index::First, _) => 0,
            (Index::Last, Orientation::Column) => 2,
            (Index::Last, Orientation::Row) => 6
        };

        let step_size: u8 = match self.orientation {
            Orientation::Column => 3,
            Orientation::Row => 1
        };

        let mut arr = array::from_fn(|i| FaceIndex::try_from(index_offset + step_size * i as u8).unwrap());

        if self.mirrored {
            for face_index in arr.iter_mut() {
                let index_raw = 8 - u8::from(*face_index);
                *face_index = index_raw.try_into().unwrap();
            }
        }

        arr
    }
}
