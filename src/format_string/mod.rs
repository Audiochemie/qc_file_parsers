//! This module implements functions to read files, which begin with a
//! Fortran formatting string.
//! A detailed explanation can be found in Chapter 11
//! [here](https://doi.org/10.1093/oso/9780198811893.001.0001)
//!
//! ```Fortran
//! !c three integers with field length 5, one white space,
//! !c 10 floats with field length 8 three of them decimal places.
//!     (3I5,1x,10F8.3)
//! ```
use std::{io::BufRead, num::ParseFloatError, str::FromStr};

use self::frmtstngerror::ParseFortranFormattedError;
pub mod frmtstngerror;

/// Maps a Fortran format string, which encodes the formatting of a single line in the file.
#[derive(Debug, PartialEq, Eq)]
pub struct FortranFormat {
    /// Number of repeat counts of the formatee, e.g. 5x <=> 5 white spaces
    pub rep: usize,
    /// The format descriptor, i.e. one of [`i`, `f`, `l`, `a`]
    /// TODO implememnt scientific notation
    pub kind: String,
    /// Defines the width of the field.
    pub fw: usize,
    /// Fortran defines decimal places to be printed for reals, e.g. field-with four with four
    /// decimal places -> f4.3
    /// A similar syntax is used if a minimum of m places (e.g. with leading zeros) shall be printed ouputting integers.
    pub suffix: usize,
}

/// Implemements the Constructor for the FortranFormat struct.
impl FortranFormat {
    /// Constructor function for FortranFormat struct
    /// # Arguments
    ///  * `rep` - number of repititions.
    ///  * `kind` - specifies the kind of the formatted data.
    ///  * `fw` - field with
    ///  * `suffix` - a possible integer given after the fw
    pub fn new(rep: usize, kind: String, fw: usize, suffix: usize) -> Self {
        Self {
            rep,
            kind,
            fw,
            suffix,
        }
    }
}

/// Implements the default for FortranFormat.
impl Default for FortranFormat {
    // Default is one repitition of an integer.
    fn default() -> Self {
        Self::new(1, "i".to_string(), 1, 1)
    }
}

impl FromStr for FortranFormat {
    /// Yields a FortranFormat struct from an input string slice.
    /// # Arguemnts
    ///  * `s` - Input string to convert from
    type Err = ParseFortranFormattedError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chiterator = s.chars();
        let i_kind: usize = chiterator.position(|c| c.is_alphabetic()).unwrap();
        let rep = s.get(..i_kind).unwrap().parse::<usize>().unwrap_or(0);
        let kind = s.get(i_kind..=i_kind).unwrap().to_string();
        let (fw, suffix): (usize, usize);
        let left = s.get(i_kind + 1..).unwrap().to_string();
        if left.is_empty() {
            (fw, suffix) = (1, 0);
        } else {
            let find_dot = left.find('.').unwrap_or(left.len());
            fw = left.get(..find_dot).unwrap().parse::<usize>()?;
            suffix = left.get(find_dot + 1..).unwrap_or("0").parse::<usize>()?;
        };
        Ok(Self {
            rep,
            kind,
            fw,
            suffix,
        })
    }
}

/// Function to read the file header, which must consist of a Fortran
/// format string.
///
/// # Arguments
///
///  * `sb` - String buffer.
///
pub fn get_formats(sb: String) -> Result<Vec<FortranFormat>, ParseFortranFormattedError> {
    let trimmed = sb.trim();
    if trimmed.starts_with('(') && trimmed.ends_with(')') {
        let drop_and_split = trimmed[1..trimmed.len() - 1].split(',');
        let formats: Vec<FortranFormat> = drop_and_split
            .map(|s| FortranFormat::from_str(s).unwrap())
            .collect();
        Ok(formats)
    } else {
        Err(ParseFortranFormattedError::UnknownFormat(
            frmtstngerror::ErrorMessage {
                msg: "Fortran format string must start with '(' and end with ')'!".to_string(),
            },
        ))
    }
}

/// Function to parse a file, which has only uniform data, i.e. only one Fortran format string for
/// the whole file.
pub fn parse_file_uniform<I: BufRead>(fb: &mut I) -> Result<(), ParseFortranFormattedError> {
    let mut sb = String::new();
    fb.read_line(&mut sb).unwrap();
    let format_string = get_formats(sb)?;
    let mut l = String::new();
    while let Ok(v) = fb.read_line(&mut l) {
        // TODO Refactor. Ideally I want to have the possibility to
        // preprocess the format string into a sequence of parse closures.
        // Example:
        // "(i3,3f12.8)" -> parse::<i32>(), parse::<f64> three times
        for fmt in format_string {
            if fmt.kind == *"f" {
            } else if fmt.kind == *"i" {
            } else if fmt.kind == *"a" {
            } else if fmt.kind == *"a" {
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod unit_test {
    use std::str::FromStr;

    use crate::format_string::FortranFormat;

    use super::get_formats;

    #[test]
    fn test_from_str() {
        let test_string = "23f4.2";
        let expected = FortranFormat {
            rep: 23,
            kind: "f".to_string(),
            fw: 4,
            suffix: 2,
        };
        let parsed = FortranFormat::from_str(test_string).unwrap();
        assert_eq!(expected, parsed);
        let test_string = "6f12.8";
        let expected = FortranFormat {
            rep: 6,
            kind: "f".to_string(),
            fw: 12,
            suffix: 8,
        };
        let parsed = FortranFormat::from_str(test_string).unwrap();
        assert_eq!(expected, parsed);
        let test_string = "i4";
        let expected = FortranFormat {
            rep: 0,
            kind: "i".to_string(),
            fw: 4,
            suffix: 0,
        };
        let parsed = FortranFormat::from_str(test_string).unwrap();
        assert_eq!(expected, parsed);
        let test_string = "2x";
        let expected = FortranFormat {
            rep: 2,
            kind: "x".to_string(),
            fw: 1,
            suffix: 0,
        };
        let parsed = FortranFormat::from_str(test_string).unwrap();
        assert_eq!(expected, parsed);
    }

    #[test]
    fn fn_get_formats() {
        let test_string = "(i4,1x,3a,5f12.8)";
        let parsed = get_formats(test_string.to_string()).unwrap();
        let expected = vec![
            FortranFormat::new(0, "i".to_string(), 4, 0),
            FortranFormat::new(1, "x".to_string(), 1, 0),
            FortranFormat::new(3, "a".to_string(), 1, 0),
            FortranFormat::new(5, "f".to_string(), 12, 8),
        ];
        assert_eq!(parsed, expected);
    }
}
