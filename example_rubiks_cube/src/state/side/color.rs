use std::{error::Error, fmt::{Debug, Display}};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RubiksSideColor(u8);

impl TryFrom<u8> for RubiksSideColor {
    type Error = RubiksSideFromU8Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value <= 5 {
            Ok(RubiksSideColor(value))
        } else {
            Err(RubiksSideFromU8Error::ValueTooHigh)
        }
    }
}

impl From<RubiksSideColor> for u8 {
    fn from(value: RubiksSideColor) -> Self {
        value.0
    }
}

impl From<RubiksSideColor> for u32 {
    fn from(value: RubiksSideColor) -> Self {
        value.0 as u32
    }
}

#[derive(Debug)]
pub enum RubiksSideFromU8Error {
    ValueTooHigh
}

impl Display for RubiksSideFromU8Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self, f)
    }
}

impl Error for RubiksSideFromU8Error {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_and_back_0_to_6() {
        for i in 0..6 {
            let color = RubiksSideColor::try_from(i).unwrap();
            assert_eq!(i, color.into());
        }
    }
}