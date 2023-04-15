use std::fmt;
use num_complex::Complex64;

#[derive(Debug, Clone, Copy)]
pub enum Num {
    Rational { num: i32, den: u32 },
    Radical { radicand: u32, index: u32 },
    Pi,
    E,
    I,
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
        }
    }
}
