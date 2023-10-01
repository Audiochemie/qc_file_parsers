use crate::xyz::xyzerrors::ParseXYZError;
use nalgebra::Point3;
use num::Float;

use super::symbol::XYZLineSymbol;
use super::symbol::PSE_SYMBOLS;

#[derive(Debug, Clone)]
pub struct XYZLineNumeric<T>
where
    T: Float + std::fmt::Debug + std::str::FromStr + 'static,
    <T as std::str::FromStr>::Err: std::fmt::Debug
{
    pub z_value: usize,
    pub xyz: Point3<T>,
}

impl<T> XYZLineNumeric<T>
where
    T: Float + std::fmt::Debug + std::str::FromStr + 'static,
    <T as std::str::FromStr>::Err: std::fmt::Debug
{
    pub fn new(line: String) -> Result<Self, ParseXYZError> {
        // First we split the string by white spaces.
        let mut split_line = line.split_whitespace();
        let z_value = split_line.next().unwrap().parse::<usize>()?;
        let x = split_line.next().unwrap().parse::<T>().unwrap();
        let y = split_line.next().unwrap().parse::<T>().unwrap();
        let z = split_line.next().unwrap().parse::<T>().unwrap();
        Ok(Self {
            z_value,
            xyz: Point3::new(x, y, z),
        })
    }
}

impl<T> PartialEq for XYZLineNumeric<T>
where
    T: Float + std::fmt::Debug + std::str::FromStr + 'static,
    <T as std::str::FromStr>::Err: std::fmt::Debug
{
    fn eq(&self, other: &Self) -> bool {
        (self.z_value == other.z_value) && (self.xyz == other.xyz)
    }
}

impl<T> Eq for XYZLineNumeric<T>
where
    T: Float + std::fmt::Debug + std::str::FromStr + 'static,
    <T as std::str::FromStr>::Err: std::fmt::Debug
{
}

impl<T> From<String> for XYZLineNumeric<T>
where
    T: Float + std::fmt::Debug + std::str::FromStr + 'static,
    <T as std::str::FromStr>::Err: std::fmt::Debug
{
    fn from(line: String) -> Self {
        // First we split the string by white spaces.
        let mut split_line = line.split_whitespace();
        let z_value = split_line.next().unwrap().parse::<usize>().unwrap();
        let x = split_line.next().unwrap().parse::<T>().unwrap();
        let y = split_line.next().unwrap().parse::<T>().unwrap();
        let z = split_line.next().unwrap().parse::<T>().unwrap();
        Self {
            z_value,
            xyz: Point3::new(x, y, z),
        }
    }
}

impl<T> From<XYZLineSymbol<T>> for XYZLineNumeric<T>
where
    T: Float + std::fmt::Debug + std::str::FromStr + 'static,
    <T as std::str::FromStr>::Err: std::fmt::Debug
{
    fn from(value: XYZLineSymbol<T>) -> Self {
        Self {
            z_value: PSE_SYMBOLS
                .iter()
                .position(|&sym| sym == value.symbol)
                .unwrap(),
            xyz: value.xyz,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_from_symbol() {
        let expected: XYZLineNumeric<f32> = XYZLineNumeric::from("2 0.23 0.14 0.23".to_string());
        let test: XYZLineSymbol<f32> = XYZLineSymbol::from("he 0.23 0.14 0.23".to_string());
        assert_eq!(expected, XYZLineNumeric::from(test));
    }
}
