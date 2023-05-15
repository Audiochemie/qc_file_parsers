use std::error;
use std::{fmt::Display, num::ParseIntError};

pub enum ParseXYZError {
    WrongHeaderError(ParseIntError),
}

impl Display for ParseXYZError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ParseXYZError::WrongHeaderError(_) => {
                write!(
                    f,
                    "Could not parse the first line as integer. Does it start with the number of atoms?"
                )
            }
        }
    }
}

impl error::Error for ParseXYZError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            ParseXYZError::WrongHeaderError(ref e) => Some(e),
        }
    }
}


impl From<ParseIntError> for ParseXYZError {
    fn from(value: ParseIntError) -> Self {
        Self::WrongHeaderError(value)
    }
}
