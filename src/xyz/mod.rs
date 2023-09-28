pub mod xyzerrors;
pub mod xyzline;

use crate::xyz::xyzline::{numeric::XYZLineNumeric, symbol::XYZLineSymbol};
use std::io::BufRead;

/// Enum to wrap lines in a xyz file starting with a numeric or a symbolic line, i.e. either
/// element symbol or atomic number.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum XyzLine {
    Symbolic(xyzline::symbol::XYZLineSymbol),
    Numeric(xyzline::numeric::XYZLineNumeric),
}

///Represents an xyz file.
#[derive(Debug)]
pub struct Xyz {
    /// This file format needs to start with the number of atoms.
    pub number_of_atoms: usize,
    /// The vectors given as cartesian triples can either have length in bohr or in angstroem
    pub distances_in: String,
    /// It is possible that a, possibly empty, info line occurs after the number of atoms.
    pub info_line: String,
    /// The lines can either start/end with an element symbol or its Z-value, followed by
    /// coordinate triples.
    pub lines: Vec<XyzLine>,
}

impl Xyz {
    /// Constructor for an Xyz structu.
    /// # Arguments:
    /// * `file_handle` - A handle to a `BufReader`, i.e. the xyz file.
    ///  * `distances_in` - The unit in which the components of the xyz-vectors are given. 'bohr' ===
    ///  `bohr`or 'angstroem' === `ang`
    ///  
    ///  # Example:
    ///
    /// ```no_run
    ///  use std::fs::File;
    ///  use std::io::BufReader;
    ///  use qc_file_parsers::xyz::Xyz;
    ///  fn main() -> std::io::Result<()> {
    ///     let f = File::open("test_file.xyz")?;
    ///     let mut b = BufReader::new(f);
    ///     let x = Xyz::new(&mut b, "ang");
    ///     Ok(())
    ///  }
    ///  ```
    pub fn new<I: BufRead>(
        file_handle: &mut I,
        distances_in: &str,
    ) -> Result<Self, self::xyzerrors::ParseXYZError> {
        let mut line_iter = file_handle.lines();
        let number_of_atoms = line_iter
            .next()
            .ok_or(self::xyzerrors::ParseXYZError::EmptyLine)?
            .unwrap()
            .parse::<usize>()?;
        let info_line = line_iter.next().unwrap().unwrap();
        let which_format = line_iter.next().unwrap().unwrap();
        let mut lines: Vec<XyzLine> = Vec::new();
        match XYZLineNumeric::new(which_format.clone()) {
            Ok(o) => {
                lines.push(XyzLine::Numeric(o));
                line_iter
                    .for_each(|l| lines.push(XyzLine::Numeric(XYZLineNumeric::from(l.unwrap()))));
            }
            Err(_) => match XYZLineSymbol::new(which_format) {
                Ok(o) => {
                    lines.push(XyzLine::Symbolic(o));
                    line_iter.for_each(|l| {
                        lines.push(XyzLine::Symbolic(XYZLineSymbol::from(l.unwrap())))
                    });
                }
                Err(e) => {
                    println!("{}", e);
                    panic!(
                "Could not identify if the line starts with an element symbol or an atomic number!"
            )
                }
            },
        }
        Ok(Self {
            number_of_atoms,
            distances_in: String::from(distances_in),
            info_line,
            lines,
        })
    }
}
