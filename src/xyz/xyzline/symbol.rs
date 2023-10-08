use crate::xyz::xyzerrors::ParseXYZError;

use nalgebra::Point3;
use num::Float;

use super::numeric::XYZLineNumeric;

use std::fmt::Debug;
use std::cmp::PartialEq;


pub const PSE_SYMBOLS: [&str; 11] = ["", "h", "he", "li", "be", "b", "c", "n", "o", "f", "ne"];


/// Represents a line in an xyz file containing an element symbol and a triple of cartesian
/// coordinates.
#[derive(Debug, Clone)]
pub struct XYZLineSymbol<T>
where
    T: Float + std::fmt::Debug + std::str::FromStr + 'static,
    <T as std::str::FromStr>::Err: std::fmt::Debug
{
    /// The symbol
    pub symbol: String,
    /// The coordinate triple
    pub xyz: Point3<T>,
}

impl<T> PartialEq for XYZLineSymbol<T>
where
    T: Float + std::fmt::Debug + std::str::FromStr + 'static,
    <T as std::str::FromStr>::Err: std::fmt::Debug
{
    fn eq(&self, other: &Self) -> bool {
        self.symbol.eq(&other.symbol) && self.xyz.eq(&other.xyz)
    }
}

impl<T> Eq for XYZLineSymbol<T>
where
    T: Float + std::fmt::Debug + std::str::FromStr + 'static,
    <T as std::str::FromStr>::Err: std::fmt::Debug
{}

impl<T> XYZLineSymbol<T>
where
    T: Float + std::fmt::Debug + std::str::FromStr + 'static,
    <T as std::str::FromStr>::Err: std::fmt::Debug
{
    pub fn new(line: String) -> Result<Self, ParseXYZError> {
        let mut split_line = line.split_whitespace();
        let symbol = split_line.next().unwrap().to_lowercase();
        let x = split_line.next().unwrap().parse::<T>().unwrap();
        let y = split_line.next().unwrap().parse::<T>().unwrap();
        let z = split_line.next().unwrap().parse::<T>().unwrap();
        Ok(Self {
            symbol,
            xyz: Point3::new(x, y, z),
        })
    }
}

impl<T> From<String> for XYZLineSymbol<T>
where
    T: Float + std::fmt::Debug + std::str::FromStr + 'static,
    <T as std::str::FromStr>::Err: std::fmt::Debug
{
    fn from(value: String) -> Self {
        let mut split_line = value.split_whitespace();
        let symbol = split_line.next().unwrap().to_string().to_lowercase();
        let x = split_line.next().unwrap().parse::<T>().unwrap();
        let y = split_line.next().unwrap().parse::<T>().unwrap();
        let z = split_line.next().unwrap().parse::<T>().unwrap();
        Self {
            symbol,
            xyz: Point3::new(x, y, z),
        }
    }
}

impl<T> From<XYZLineNumeric<T>> for XYZLineSymbol<T>
where
    T: Float + std::fmt::Debug + std::str::FromStr + 'static,
    <T as std::str::FromStr>::Err: std::fmt::Debug
{
    fn from(value: XYZLineNumeric<T>) -> Self {
        Self {
            symbol: PSE_SYMBOLS[value.z_value].to_string(),
            xyz: value.xyz,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_from_numeric() {
        let test: XYZLineNumeric<f32> = XYZLineNumeric::from("2 0.0 0.0 0.0".to_string());
        let expected: XYZLineSymbol<f32> = XYZLineSymbol::from("he 0.0 0.0 0.0".to_string());
        assert_eq!(expected.xyz, XYZLineSymbol::from(test).xyz);
    }
}
