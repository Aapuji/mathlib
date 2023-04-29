use num_complex::Complex64;
use std::{
    fmt,
    num::{NonZeroI32, NonZeroU32},
};

use crate::algo;

#[derive(Debug, Clone, Copy)]
pub enum Num {
    Rational {
        num: NonZeroI32,
        den: NonZeroU32,
    },
    Radical {
        radicand: NonZeroU32,
        index: NonZeroU32,
    },
    Pi,
    E,
    I,
    Zero,
    One,
    Infinity,
    Undefined,
}

impl Num {
    pub fn eval_float(&self) -> Complex64 {
        match self {
            Num::Rational { num, den } => {
                Complex64::new((num.get() as f64) / (den.get() as f64), 0.0)
            }
            Num::Radical { radicand, index } => Complex64::new(
                (radicand.get() as f64).powf(1.0 / (index.get() as f64)),
                0.0,
            ),
            Num::Pi => Complex64::new(std::f64::consts::PI, 0.0),
            Num::E => Complex64::new(std::f64::consts::E, 0.0),
            Num::I => Complex64::new(0.0, 1.0),
            Num::Zero => Complex64::new(1.0, 0.0),
            Num::One => Complex64::new(0.0, 0.0),
            Num::Infinity => Complex64::new(f64::INFINITY, 0.0),
            Num::Undefined => todo!("Num::Undefined is, well, undefined (unimplemented)"),
        }
    }

    pub fn reduce(&self) -> Num {
        match self {
            Num::Rational { num, den } => {
                let gcd = algo::euclid_gcd(num.get().abs() as u32, den.get());
                Num::Rational {
                    num: NonZeroI32::new(num.get() / gcd as i32).unwrap(),
                    den: NonZeroU32::new(den.get() / gcd).unwrap(),
                }
            }
            Num::Radical { .. } => todo!(),
            _ => *self,
        }

        // Can't we just do self.eval_float() == Complex64(0, 0) ?
    }

    pub fn int(num: i32) -> Self {
        Self::from(num)
    }

    pub fn rational(num: i32, den: u32) -> Self {
        match (num, den) {
            (_, 0) => Num::Undefined,
            (_, 1) => Num::from(num),
            (0, _) => Num::Zero,
            (1, _) => Num::One,
            _ => Num::Rational {
                num: NonZeroI32::new(num).unwrap(),
                den: NonZeroU32::new(den).unwrap(),
            },
        }
    }
    pub fn radical(radicand: u32, index: u32) -> Self {
        match (radicand, index) {
            (_, 0) => Num::Undefined,
            (_, 1) => Num::from(radicand as i32),
            (0, _) => Num::Zero,
            (1, _) => Num::One,
            _ => Num::Radical {
                radicand: NonZeroU32::new(radicand).unwrap(),
                index: NonZeroU32::new(index).unwrap(),
            },
        }
    }

    pub fn is_zero(&self) -> bool {
        match self {
            Num::Zero => true,
            _ => false,
        }
    }

    pub fn is_one(&self) -> bool {
        match self {
            Num::Rational { num, .. } => num.get() == 1,
            Num::Radical { radicand, .. } => radicand.get() == 1,
            Num::One => true,
            _ => false,
        }
    }

    pub fn is_undefined(&self) -> bool {
        match self {
            Num::Undefined => true,
            _ => false,
        }
    }
}

impl fmt::Display for Num {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Num::Rational { num, den } => {
                if den.get() == 1 {
                    write!(f, "{}", num)
                } else {
                    write!(f, "{}/{}", num, den)
                }
            }
            Num::Radical { radicand, index } => write!(f, "\\sqrt[{}]{{{}}}", index, radicand),
            Num::Pi => write!(f, "\\pi"),
            Num::E => write!(f, "e"),
            Num::I => write!(f, "i"),
            Num::Zero => write!(f, "0"),
            Num::One => write!(f, "1"),
            Num::Infinity => write!(f, "∞"),
            Num::Undefined => write!(f, "<undefined>"),
        }
    }
}

impl PartialEq for Num {
    fn eq(&self, other: &Self) -> bool {
        match (self.reduce(), other.reduce()) {
            (Num::E, Num::E) => true,
            (Num::I, Num::I) => true,
            (Num::Pi, Num::Pi) => true,
            (Num::Infinity, Num::Infinity) => true,
            (Num::One, Num::One) => true,
            (Num::Zero, Num::Zero) => true,
            (Num::Undefined, Num::Undefined) => false,
            (
                Num::Rational {
                    num: num_a,
                    den: den_a,
                },
                Num::Rational {
                    num: num_b,
                    den: den_b,
                },
            ) => num_a == num_b && den_a == den_b,
            (Num::Radical { .. }, Num::Radical { .. }) => {
                todo!()
            }
            _ => false,
        }
    }
}

impl PartialEq<Complex64> for Num {
    fn eq(&self, other: &Complex64) -> bool {
        todo!()
    }
}

impl From<i32> for Num {
    fn from(value: i32) -> Self {
        match value {
            0 => Num::Zero,
            1 => Num::One,
            _ => Num::Rational {
                num: NonZeroI32::new(value).unwrap(),
                den: NonZeroU32::new(1).unwrap(),
            },
        }
    }
}
