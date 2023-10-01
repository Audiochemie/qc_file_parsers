pub mod xyz;

/// Trait defining if something is f32 or f64.
pub trait IsFloat {}
impl IsFloat for f32 {} 
impl IsFloat for f64 {}
