use num_complex::Complex64;
use std::{collections::HashMap, fmt::Debug};

use crate::expr::{EvalError, EvalResult};
use crate::var::Var;

pub trait FuncDef: Debug {
    fn eval(&self, args: Vec<Complex64>, global_vars: &HashMap<&Var, Complex64>) -> EvalResult;
    fn is_variant_on_global(&self, global_vars: &Var) -> bool;
}

#[derive(Debug)]
pub enum Function {
    Abs,
    Sgn,
    Exp,
    Ln,
    Sin,
    Cos,
    Tan,
    Arcsin,
    Arccos,
    Arctan,
    Sinh,
    Cosh,
    Tanh,
    Arcsinh,
    Arccosh,
    Arctanh,
}

impl FuncDef for Function {
    fn eval(&self, args: Vec<Complex64>, _global_vars: &HashMap<&Var, Complex64>) -> EvalResult {
        if args.len()
            != match self {
                _ => 1,
            }
        {
            return Err(EvalError::FnArgCountMismatch {});
        }
        
        Ok(match self {
            Function::Abs => Complex64::from(args[0].norm()),
            Function::Sgn => args[0] / args[0].norm(),
            Function::Exp => args[0].exp(),
            Function::Ln => args[0].ln(),
            Function::Sin => args[0].sin(),
            Function::Cos => args[0].cos(),
            Function::Tan => args[0].tan(),
            Function::Arcsin => args[0].asin(),
            Function::Arccos => args[0].acos(),
            Function::Arctan => args[0].atan(),
            Function::Sinh => args[0].sinh(),
            Function::Cosh => args[0].cosh(),
            Function::Tanh => args[0].tanh(),
            Function::Arcsinh => args[0].asinh(),
            Function::Arccosh => args[0].acosh(),
            Function::Arctanh => args[0].atanh(),
        })
    }

    fn is_variant_on_global(&self, _global_vars: &Var) -> bool {
        false
    }
}
