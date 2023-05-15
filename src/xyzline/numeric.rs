use super::symbol::PSE_SYMBOLS;
use super::symbol::XYZLineSymbol;

#[derive(Debug)]
pub struct XYZLineNumeric {
    z_value: usize,
    xyz: (f32, f32, f32),
}

impl XYZLineNumeric {
   pub fn new(z_value: usize, x: f32, y: f32, z: f32) -> Self {
        XYZLineNumeric {
            z_value,
            xyz: (x, y, z),
        }
    }
}

impl PartialEq for XYZLineNumeric {
    fn eq(&self, other: &Self) -> bool {
        ( self.z_value == other.z_value ) && ( self.xyz == other.xyz )
    }
    
}

pub trait ToSymbol {
    fn to_symbol(self) -> XYZLineSymbol;
}

impl ToSymbol for XYZLineNumeric {
    fn to_symbol(self) -> XYZLineSymbol{
        let symbol = PSE_SYMBOLS[self.z_value];
        let (x,y,z) = self.xyz;
        XYZLineSymbol::new(symbol, x, y, z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_to_symbol() {
        let test_line = XYZLineNumeric::new(2, 0.0_f32, 0.0_f32, 0.0_f32);
        let expected = XYZLineSymbol::new("he", 0.0_f32, 0.0_f32, 0.0_f32);
        assert_eq!(expected, test_line.to_symbol());
        
    }
}
