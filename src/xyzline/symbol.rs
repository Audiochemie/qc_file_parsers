use std::fmt::Display;

use super::numeric::XYZLineNumeric;

pub const PSE_SYMBOLS: [&str; 11] = ["", "h", "he", "li", "be", "b", "c", "n", "o", "f", "ne"];

pub struct XYZLineSymbol {
    symbol: String,
    xyz: (f32, f32, f32),
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

impl  ToNumeric for XYZLineSymbol {
    fn to_numeric(self) -> XYZLineNumeric
    {
        let z_value = PSE_SYMBOLS.iter().position(|&symbol| symbol == self.symbol).unwrap();
        let (x,y,z) = self.xyz;
        XYZLineNumeric::new(z_value, x, y, z)
    }
    
}
