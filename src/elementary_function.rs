use std::collections::HashMap;

use num_complex::Complex64;

use crate::{expr::{EvalResult, EvalError}, function::FuncDef, var::Var};

#[derive(Debug)]
pub enum ElementaryFn {
    Sin,
    Tan,
    Exp,
    Ln,
}

impl FuncDef for ElementaryFn {
    fn eval(&self, args: Vec<Complex64>, _global_vars: &HashMap<&Var, Complex64>) -> EvalResult {
        if args.len() != match self {
            _ => 1
        } {
            return Err(EvalError::FnArgCountMismatch { });
        }
        Ok(match self {
            ElementaryFn::Sin => args[0].sin(),
            ElementaryFn::Tan => args[0].tan(),
            ElementaryFn::Exp => args[0].exp(),
            ElementaryFn::Ln => args[0].ln(),
        })
    }
    fn is_variant_on_global(&self, _global_vars: &Var) -> bool {
        false
    }
}