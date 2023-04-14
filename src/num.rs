#[derive(Debug)]
pub enum Num {
    Rational(bool, u32, u32), // ( neg?, num, denom )
    Radical(u32, u32), // ( radicand, index )
    I,
    Pi,
    E,
    Sqrt2,
    Ln2
}