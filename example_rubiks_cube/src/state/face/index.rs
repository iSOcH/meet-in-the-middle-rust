use std::{error::Error, fmt::Display};

#[derive(Debug, Clone, Copy)]
pub struct Index(pub u8);

impl Index {
    pub fn to_shift(&self) -> u8 {
        self.0 * 3
    }
}

impl TryFrom<u8> for Index {
    type Error = IndexFromU8Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value < 8 {
            Ok(Index(value))
        } else {
            Err(IndexFromU8Error::ValueTooHigh)
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum IndexFromU8Error {
    ValueTooHigh
}

impl Error for IndexFromU8Error {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

impl Display for IndexFromU8Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
