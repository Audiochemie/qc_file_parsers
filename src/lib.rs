use std::io::BufRead;

use xyzline::{numeric::XYZLineNumeric, symbol::XYZLineSymbol};

pub mod xyzerrors;
pub mod xyzline;

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
