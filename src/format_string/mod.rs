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
use core::panic;
use num::Num;
use std::{io::BufRead, str::FromStr};

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

/// Function returning the correct parse function for type T
fn _parse_numeric_slice<T>(// Explanation: for<'a> specifies a higher-ranked trait bound, i.e. higher ranked than the
    // lifttime. for every possible lifetime 'a a type T can have we can return a function which
    // takes a string slice and returns a Result.
) -> for<'a> fn(&'a str) -> std::result::Result<T, <T as std::str::FromStr>::Err>
where
    T: FromStr + Num + std::fmt::Debug,
    <T as FromStr>::Err: std::fmt::Debug,
{
    T::from_str
}

/// Enum for the possible outcomes of parsing.
#[derive(Debug, PartialEq)]
pub enum ParsedValue {
    Fl(f64),
    In(i32),
    Lo(bool),
    St(String),
}
/// Function to parse a file with data formatted according to a given Fortran format string.
/// # Arguments
///  * `f_ff` - File to process.
pub fn parse_fortran_formatted_buf<I: BufRead>(
    f_ff: &mut I,
) -> Result<Vec<Vec<ParsedValue>>, ParseFortranFormattedError> {
    let mut line_buffer = f_ff.lines().map(|l| l.unwrap());
    // MUST be the Fortran Format string.
    let ff: Vec<FortranFormat> = get_formats(line_buffer.next().unwrap())?;
    let tuple_len: usize = ff.iter().fold(0, |acc, f| acc + f.rep);
    let mut result_data: Vec<Vec<ParsedValue>> = Vec::new();
    for l in line_buffer {
        let mut start: usize = 0;
        for f in ff.iter() {
            let slice_len = f.fw;
            let mut to_push: Vec<ParsedValue> = Vec::with_capacity(tuple_len);
            // TODO Refactor!!!
            for _r in 0..f.rep {
                let parsed: ParsedValue;
                let slice: String = l[start..slice_len].trim().to_string();
                if f.kind == *"f" {
                    if slice.is_empty() {
                        panic!("{} is empty", slice);
                    }
                    parsed = ParsedValue::Fl(slice.parse::<f64>().unwrap());
                } else if f.kind == *"i" {
                    parsed = ParsedValue::In(slice.parse::<i32>().unwrap());
                } else if f.kind == *"a" {
                    parsed = ParsedValue::St(slice.parse::<String>().unwrap());
                } else {
                    panic!("AAAHHHH!!");
                }

                to_push.push(parsed);
                start += slice_len;
            }
            result_data.push(to_push);
        }
    }
    Ok(result_data)
}

#[cfg(test)]
mod unit_test {
    use std::str::FromStr;
    use std::{fs::File, io::BufReader};

    use crate::format_string::{FortranFormat, _parse_numeric_slice};

    use super::{get_formats, parse_fortran_formatted_buf, ParsedValue};

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
    fn test_get_formats() {
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

    #[test]
    fn test_parse_slice() {
        let test_slice = "3";
        let parser = _parse_numeric_slice::<usize>();
        assert_eq!(
            test_slice.parse::<usize>().unwrap(),
            parser(test_slice).unwrap()
        );
        let test_slice = "12.32100";
        let parser = _parse_numeric_slice::<f32>();
        assert_eq!(
            test_slice.parse::<f32>().unwrap(),
            parser(test_slice).unwrap()
        );
    }

    #[test]
    fn test_parse_fortran() {
        let test_file = File::open("tests/test_file.dat").unwrap();
        let mut test_buffer = BufReader::new(test_file);
        let parsed = parse_fortran_formatted_buf(&mut test_buffer).unwrap();
        let inner: Vec<ParsedValue> = vec![
            ParsedValue::Fl(112.0_f64),
            ParsedValue::Fl(113.0_f64),
            ParsedValue::Fl(14.0_f64),
            ParsedValue::Fl(15.0_f64),
            ParsedValue::In(1234),
            ParsedValue::In(567),
            ParsedValue::In(9),
        ];
        let expected: Vec<Vec<ParsedValue>> = vec![inner];
        for (p, e) in parsed.iter().zip(expected.iter()) {
            assert_eq!(p, e);
        }
    }
}
