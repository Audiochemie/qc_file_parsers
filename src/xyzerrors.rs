use std::error;
use std::{fmt::Display, num::ParseFloatError, num::ParseIntError};

#[derive(Debug)]
pub enum ParseXYZError {
    EmptyLine,
    WrongHeaderError(ParseIntError),
    CoordinateError(ParseFloatError),
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
            ParseXYZError::CoordinateError(_) => {
                write!(f, "Could not parse the coordinate. Is it a correct float? Is the seperator a whitespace?")
            }
            ParseXYZError::EmptyLine => {
                write!(f, "Found an empty line where there shouldn't be one.")
            }
        }
    }
}

impl error::Error for ParseXYZError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            ParseXYZError::WrongHeaderError(ref e) => Some(e),
            ParseXYZError::CoordinateError(ref e) => Some(e),
            ParseXYZError::EmptyLine => None,
        }
    }
}

impl From<ParseIntError> for ParseXYZError {
    fn from(value: ParseIntError) -> Self {
        Self::WrongHeaderError(value)
    }
}
