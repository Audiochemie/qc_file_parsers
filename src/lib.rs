use std::io::BufRead;

use xyzline::{numeric::XYZLineNumeric, symbol::XYZLineSymbol};

mod xyzerrors;
mod xyzline;

///Collects different geometry file formats used in quantum chemistry
pub enum GeometryFormat {
    /// The _classic_ cartesian xyz file.
    Xyz,
}

pub enum XyzLine {
    Symbolic(xyzline::symbol::XYZLineSymbol),
    Numeric(xyzline::numeric::XYZLineNumeric),
}

///Represents an xyz file.
struct Xyz {
    /// This file format needs to start with the number of atoms.
    number_of_atoms: usize,
    /// The vectors given as cartesian triples can either have length in bohr or in angstroem
    distances_in: String,
    /// It is possible that a, possibly empty, info line occurs after the number of atoms.
    info_line: String,
    /// The lines can either start/end with an element symbol or its Z-value, followed by
    /// coordinate triples.
    lines: Vec<XyzLine>,
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
        if let Ok(o) = XYZLineNumeric::new(which_format.clone()) {
            lines.push(XyzLine::Numeric(o));
            line_iter.for_each(|l| lines.push(XyzLine::Numeric(XYZLineNumeric::from(l.unwrap()))));
        } else if let Ok(o) = XYZLineSymbol::new(which_format) {
            lines.push(XyzLine::Symbolic(o));
            line_iter.for_each(|l| lines.push(XyzLine::Symbolic(XYZLineSymbol::from(l.unwrap()))));
        } else {
            panic!(
                "Could not identify if the line starts with an element symbol or an atomic number!"
            )
        }

        Ok(Self {
            number_of_atoms,
            distances_in: String::from(distances_in),
            info_line,
            lines
        })
    }
}
