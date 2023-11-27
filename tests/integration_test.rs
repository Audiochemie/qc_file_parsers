mod file_setup;
#[cfg(test)]
mod tests {
    use crate::file_setup;
    use nalgebra::Point3;
    use qc_file_parsers::format_string::{parse_fortran_formatted_buf, ParsedValue};
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
    fn test_fortran_format_string() {
        let mut test_file = file_setup::setup_allene_fortran_format_string().unwrap();
        let test_parsed = parse_fortran_formatted_buf(&mut test_file).unwrap();
        let expected = vec![
            vec![ParsedValue::St("C".to_string())],
            vec![
                ParsedValue::Fl(0.0),
                ParsedValue::Fl(0.0),
                ParsedValue::Fl(1.889725988579),
            ],
            vec![ParsedValue::St("C".to_string())],
            vec![
                ParsedValue::Fl(2.551130084582),
                ParsedValue::Fl(0.000000000000),
                ParsedValue::Fl(1.889725988579),
            ],
            vec![ParsedValue::St("C".to_string())],
            vec![
                ParsedValue::Fl(-2.551130084582),
                ParsedValue::Fl(0.000000000000),
                ParsedValue::Fl(1.889725988579),
            ],
            vec![ParsedValue::St("H".to_string())],
            vec![
                ParsedValue::Fl(3.495993078871),
                ParsedValue::Fl(1.157216106424),
                ParsedValue::Fl(0.732509882155),
            ],
            vec![ParsedValue::St("H".to_string())],
            vec![
                ParsedValue::Fl(3.495993078871),
                ParsedValue::Fl(-1.157216106424),
                ParsedValue::Fl(3.046942095003),
            ],
            vec![ParsedValue::St("H".to_string())],
            vec![
                ParsedValue::Fl(-3.495993078871),
                ParsedValue::Fl(-1.157216106424),
                ParsedValue::Fl(0.732509882155),
            ],
            vec![ParsedValue::St("H".to_string())],
            vec![
                ParsedValue::Fl(-3.495993078871),
                ParsedValue::Fl(1.157216106424),
                ParsedValue::Fl(3.046942095003),
            ],
        ];
        println!("{:?}", test_parsed);
        assert_eq!(test_parsed.len(), expected.len());
        for (p, e) in test_parsed.iter().zip(expected.iter()) {
            assert_eq!(p, e)
        }
    }

}
