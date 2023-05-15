mod xyzline;
/// Collects different informations contained on the lines in different formats.
enum Line {
    /// Line in an xyz style file with z-values to classify atoms.
    XYZLineNumeric,
    /// Line in an xyz style file with element symbols to classify atoms.
    XYZLineSymbol,
}



///Collects different geometry file formats used in quantum chemistry
pub enum GeometryFormat {
    /// The _classic_ cartesian xyz file .
   Xyz,
}

///Represents an xyz file.
struct Xyz<L> {
    number_of_atoms: usize,
    distances_in: String,
    info_line: String,
    lines: Vec<L>
}

impl<L> Xyz<L> {
    fn new() {}
}
