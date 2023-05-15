use super::symbol::PSE_SYMBOLS;
use super::symbol::XYZLineSymbol;

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
