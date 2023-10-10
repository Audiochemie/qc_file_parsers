use std::fmt::Display;
use std::num::ParseIntError;

#[derive(Debug)]
pub struct ErrorMessage {
    pub msg: String,
}
#[derive(Debug)]
pub enum ParseFortranFormattedError {
    /// Raised when the format is unknown.
    UnknownFormat(ErrorMessage),
}

impl Display for ParseFortranFormattedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownFormat(fmtstg) => {
                write!(f, "Unidentifiable format string {}", fmtstg.msg)
            }
        }
    }
}

impl From<ParseIntError> for ParseFortranFormattedError {
    fn from(value: ParseIntError) -> Self {
        Self::UnknownFormat(ErrorMessage {
            msg: format!("Cannot parse {} as integer.", value),
        })
    }
}
