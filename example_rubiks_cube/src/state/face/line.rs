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
}

#[derive(Debug, Clone, PartialEq)]
pub enum Orientation {
    Row,
    Column
}

#[repr(u8)]
#[derive(Debug, PartialEq, Clone)]
pub enum Index {
    /// move which always affects C0
    First = 0b0,  // 1 bit for 2 rows

    /// move which always affects C8
    Last  = 0b1,
}

type Indices = [FaceIndex; 3];

impl Id {
    pub fn indices(&self) -> Indices {
        let index_offset: u8 = match self.index {
            Index::First => 0,
            Index::Last => 2
        };

        let step_size: u8 = match self.orientation {
            Orientation::Column => 3,
            Orientation::Row => 1
        };

        let mut arr = array::from_fn(|i| FaceIndex::try_from(index_offset + step_size * i as u8).unwrap());

        for face_index in arr.iter_mut() {
            let index_raw = 8 - u8::from(*face_index);
            *face_index = index_raw.try_into().unwrap();
        }

        arr
    }
}
