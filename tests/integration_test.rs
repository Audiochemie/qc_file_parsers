mod file_setup;
#[cfg(test)]
mod tests {
    use crate::file_setup;
    use nalgebra::{DMatrix, Point3};
    use qc_file_parsers::array_text::parse_text_into_matrix;
    use qc_file_parsers::xyz::xyzline::symbol::XYZLineSymbol;
    use qc_file_parsers::xyz::{Xyz, XyzLine};

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

    #[test]
    fn test_array_from_text() {
        let mut test_file = file_setup::setup_array_text().unwrap();
        let expected = DMatrix::<f64>::from_vec(
            27,
            3,
            vec![
                0.0927643390,
                0.0000000000,
                0.0000000000,
                -0.0463821695,
                0.0000000000,
                0.0000000000,
                -0.0463821695,
                0.0000000000,
                0.0000000000,
                0.0000000000,
                0.3171327134,
                0.0000000000,
                0.0000000000,
                -0.1585663567,
                0.0800202030,
                0.0000000000,
                -0.1585663567,
                -0.0800202030,
                0.0000000000,
                0.0000000000,
                0.2800907293,
                0.0000000000,
                0.0347765865,
                -0.1400453646,
                0.0000000000,
                -0.0347765865,
                -0.1400453646,
                -0.0463821695,
                0.0000000000,
                0.0000000000,
                0.0514668232,
                0.0000000000,
                0.0000000000,
                -0.0050846537,
                0.0000000000,
                0.0000000000,
                0.0000000000,
                -0.1585663567,
                0.0347765865,
                0.0000000000,
                0.1730075524,
                -0.0573983947,
                0.0000000000,
                -0.0144411957,
                0.0226218083,
                0.0000000000,
                0.0800202030,
                -0.1400453646,
                0.0000000000,
                -0.0573983947,
                0.1268373488,
                0.0000000000,
                -0.0226218083,
                0.0132080159,
                -0.0463821695,
                0.0000000000,
                0.0000000000,
                -0.0050846537,
                0.0000000000,
                0.0000000000,
                0.0514668232,
                0.0000000000,
                0.0000000000,
                0.0000000000,
                -0.1585663567,
                -0.0347765865,
                0.0000000000,
                -0.0144411957,
                -0.0226218083,
                0.0000000000,
                0.1730075524,
                0.0573983947,
                0.0000000000,
                -0.0800202030,
                -0.1400453646,
                0.0000000000,
                0.0226218083,
                0.0132080159,
                0.0000000000,
                0.0573983947,
                0.1268373488,
            ],
        );
        let parsed: DMatrix<f64> = parse_text_into_matrix(&mut test_file, " ", 3, 27);
        assert_eq!(parsed, expected)

    }
}
