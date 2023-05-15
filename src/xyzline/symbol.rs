use super::numeric::XYZLineNumeric;

pub const PSE_SYMBOLS: [&str; 11] = ["", "h", "he", "li", "be", "b", "c", "n", "o", "f", "ne"];

/// Represents a line in an xyz file containing an element symbol and a triple of cartesian
/// coordinates.
#[derive(Debug)]
pub struct XYZLineSymbol {
    /// The symbol
    symbol: String,
    /// The coordinate triple
    xyz: (f32, f32, f32),
}

impl PartialEq for XYZLineSymbol {
    fn eq(&self, other: &Self) -> bool {
        self.symbol.eq(&other.symbol) && self.xyz.eq(&other.xyz)
    }
}

impl XYZLineSymbol {
    pub fn new(symbol: &str, x: f32, y: f32, z: f32) -> Self {
        XYZLineSymbol {
            symbol: symbol.to_string(),
            xyz: (x, y, z),
        }
    }
}

trait ToNumeric {
    fn to_numeric(self) -> XYZLineNumeric;
}

impl ToNumeric for XYZLineSymbol {
    fn to_numeric(self) -> XYZLineNumeric {
        let z_value = PSE_SYMBOLS
            .iter()
            .position(|&symbol| symbol == self.symbol)
            .unwrap();
        let (x, y, z) = self.xyz;
        XYZLineNumeric::new(z_value, x, y, z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_to_numeric() {
        let test_line = XYZLineSymbol::new("he", 0.0_f32, 0.0_f32, 0.0_f32);
        let expected = XYZLineNumeric::new(2, 0.0_f32, 0.0_f32, 0.0_f32);
        assert_eq!(expected, test_line.to_numeric())
    }
}
