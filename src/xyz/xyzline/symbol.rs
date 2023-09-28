use nalgebra::Point3;
use crate::xyz::xyzerrors::ParseXYZError;

use super::numeric::XYZLineNumeric;

pub const PSE_SYMBOLS: [&str; 11] = ["", "h", "he", "li", "be", "b", "c", "n", "o", "f", "ne"];

/// Represents a line in an xyz file containing an element symbol and a triple of cartesian
/// coordinates.
#[derive(Debug, Clone)]
pub struct XYZLineSymbol {
    /// The symbol
    pub symbol: String,
    /// The coordinate triple
    pub xyz: Point3<f32>,
}


impl PartialEq for XYZLineSymbol {
    fn eq(&self, other: &Self) -> bool {
        self.symbol.eq(&other.symbol) && self.xyz.eq(&other.xyz)
    }
}

impl Eq for XYZLineSymbol {}

impl XYZLineSymbol {
    pub fn new(line: String) -> Result<Self, ParseXYZError> {
        let mut split_line = line.split_whitespace();
        let symbol = split_line.next().unwrap().to_lowercase();
        let x = split_line.next().unwrap().parse::<f32>()?;
        let y = split_line.next().unwrap().parse::<f32>()?;
        let z = split_line.next().unwrap().parse::<f32>()?;
        Ok(Self{
            symbol,
            xyz: Point3::new(x, y, z),
        })
    }
}

impl From<String> for XYZLineSymbol {
    fn from(value: String) -> Self {
        let mut split_line = value.split_whitespace();
        let symbol = split_line.next().unwrap().to_string();
        let x = split_line.next().unwrap().parse::<f32>().unwrap();
        let y = split_line.next().unwrap().parse::<f32>().unwrap();
        let z = split_line.next().unwrap().parse::<f32>().unwrap();
        Self{
            symbol,
            xyz: Point3::new(x, y, z),
        }
    }

}

impl From<XYZLineNumeric> for XYZLineSymbol {
    fn from(value: XYZLineNumeric) -> Self {
        Self {symbol: PSE_SYMBOLS[value.z_value].to_string(), xyz: value.xyz }
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_from_numeric() {
        let test = XYZLineNumeric::from("2 0.0 0.0 0.0".to_string());
        let expected = XYZLineSymbol::from("he 0.0 0.0 0.0".to_string());
        assert_eq!(expected.xyz, XYZLineSymbol::from(test).xyz);
    }
}
