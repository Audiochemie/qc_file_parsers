use crate::xyzerrors::ParseXYZError;

use super::symbol::XYZLineSymbol;
use super::symbol::PSE_SYMBOLS;

#[derive(Debug)]
pub struct XYZLineNumeric {
    pub z_value: usize,
    pub xyz: (f32, f32, f32),
}

impl XYZLineNumeric {
    pub fn new(line: String) -> Result<Self, ParseXYZError> {
        // First we split the string by white spaces.
        let mut split_line = line.split_whitespace();
        let z_value = split_line.next().unwrap().parse::<usize>()?;
        let x = split_line.next().unwrap().parse::<f32>()?;
        let y = split_line.next().unwrap().parse::<f32>()?;
        let z = split_line.next().unwrap().parse::<f32>()?;
        Ok(Self {
            z_value,
            xyz: (x, y, z),
        })
    }
}

impl PartialEq for XYZLineNumeric {
    fn eq(&self, other: &Self) -> bool {
        (self.z_value == other.z_value) && (self.xyz == other.xyz)
    }
}

impl Eq for XYZLineNumeric {}

impl From<String> for XYZLineNumeric {
    fn from(line: String) -> Self {
        // First we split the string by white spaces.
        let mut split_line = line.split_whitespace();
        let z_value = split_line.next().unwrap().parse::<usize>().unwrap();
        let x = split_line.next().unwrap().parse::<f32>().unwrap();
        let y = split_line.next().unwrap().parse::<f32>().unwrap();
        let z = split_line.next().unwrap().parse::<f32>().unwrap();
        Self {
            z_value,
            xyz: (x, y, z),
        }
    }
}

impl From<XYZLineSymbol> for XYZLineNumeric {
    fn from(value: XYZLineSymbol) -> Self {
        Self { z_value: PSE_SYMBOLS.iter().position(|&sym| sym == value.symbol).unwrap(), xyz: value.xyz}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_from_symbol() {
        let expected = XYZLineNumeric::from("2 0.23 0.14 0.23".to_string());
        let test = XYZLineSymbol::from("he 0.23 0.14 0.23".to_string());
        assert_eq!(expected, XYZLineNumeric::from(test));
    }
}
