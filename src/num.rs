use std::fmt;
use num_complex::Complex64;

use crate::algo;

#[derive(Debug, Clone, Copy)]
pub enum Num {
    Rational { num: i32, den: u32 },
    Radical { radicand: u32, index: u32 },
    Pi,
    E,
    I,
    Infinity,
    Undefined
}

impl Num {
    pub fn eval_float(&self) -> Complex64 {
        match self {
            Num::Rational { num, den } => Complex64::new(
                (*num as f64) / (*den as f64),
                0.0,
            ),
            Num::Radical { radicand, index } => {
                Complex64::new((*radicand as f64).powf(1.0 / (*index as f64)), 0.0)
            }
            Num::Pi => Complex64::new(std::f64::consts::PI, 0.0),
            Num::E => Complex64::new(std::f64::consts::E, 0.0),
            Num::I => Complex64::new(0.0, 1.0),
            Num::Infinity => Complex64::new(f64::INFINITY, 0.0),
            Num::Undefined => todo!("Num::Undefined is, well, undefined (unimplemented)")
        }
    }
    pub fn reduce(&self) -> Num {
        match self {
            Num::Rational { num, den } => {
                let gcd = algo::euclid_gcd(num.abs() as u32, *den);
                Num::Rational { num: num / gcd as i32, den: den / gcd }
            }
            _ => *self
        }
    }
    pub fn zero() -> Num { Num::Rational { num: 0, den: 1 } }
    pub fn is_zero(&self) -> bool {
        match self {
            Num::Rational { num, den } => *num == 0 && *den != 0,
            Num::Radical { radicand, index } => *radicand == 0 && *index != 0,
            _ => false,
        }
    }
    pub fn one() -> Num { Num::Rational { num: 1, den: 1 } }
    pub fn is_one(&self) -> bool {
        match self {
            Num::Rational { num, den } => *num == 1 && *den != 0,
            Num::Radical { radicand, index } => *radicand == 1 && *index != 0,
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
                write!(f, "{}/{}", num, den)
            }
            Num::Radical { radicand, index } => write!(f, "\\sqrt[{index}]{{{radicand}}}"),
            Num::Pi => write!(f, "\\pi"),
            Num::E => write!(f, "e"),
            Num::I => write!(f, "i"),
            Num::Infinity => write!(f, "∞"),
            Num::Undefined => write!(f, "UNDEF")
        }
    }
}
