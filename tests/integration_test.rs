mod file_setup;
#[cfg(test)]
mod tests {
    use crate::file_setup;
    use qc_file_parsers::xyz::xyzline::symbol::XYZLineSymbol;
    use qc_file_parsers::xyz::{Xyz, XyzLine};
    use nalgebra::Point3;
    #[test]
    fn test_symbolic_constructor() {
        let mut test_file = file_setup::setup_allene_symbolic().unwrap();
        let test_parsed = Xyz::new(&mut test_file, "Ang").unwrap();
        assert_eq!(test_parsed.number_of_atoms, 7);
        assert!(test_parsed.info_line.is_empty());
        assert_eq!(
            XyzLine::Symbolic(XYZLineSymbol {
                symbol: "h".to_string(),
                xyz: Point3::new(3.495_993_1_f32, 1.157_216_1_f32, 0.732_509_9_f32)
            }),
            test_parsed.lines[3]
        );
    }
    #[test]
    fn test_numeric_constructor() {
        let mut test_file = file_setup::setup_acetaldehyde_numeric().unwrap();
        let test_parsed = Xyz::new(&mut test_file, "Ang").unwrap();
        assert_eq!(test_parsed.number_of_atoms, 7);
        assert!(test_parsed.info_line.is_empty());
        assert_eq!(
            XyzLine::Numeric(qc_file_parsers::xyz::xyzline::numeric::XYZLineNumeric {
                z_value: 8,
                xyz: Point3::new(1.899_115_9_f32, 0.0_f32, 4.139_062_4_f32)
            }),
            test_parsed.lines[2]
        );
    }
}
