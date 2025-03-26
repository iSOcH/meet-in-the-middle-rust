use std::{error::Error, fmt::{Debug, Display}};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color(u8);

impl TryFrom<u8> for Color {
    type Error = ColorFromU8Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value <= 5 {
            Ok(Color(value))
        } else {
            Err(ColorFromU8Error::ValueTooHigh)
        }
    }
}

impl From<Color> for u8 {
    fn from(value: Color) -> Self {
        value.0
    }
}

impl From<Color> for u32 {
    fn from(value: Color) -> Self {
        value.0 as u32
    }
}

#[derive(Debug)]
pub enum ColorFromU8Error {
    ValueTooHigh
}

impl Display for ColorFromU8Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self, f)
    }
}

impl Error for ColorFromU8Error {
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
            let color = Color::try_from(i).unwrap();
            assert_eq!(i, color.into());
        }
    }
}