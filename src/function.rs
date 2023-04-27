use std::{collections::HashMap, fmt::Debug};
use num_complex::Complex64;

use crate::{expr::EvalResult, var::Var};

pub trait FuncDef : Debug {
    fn eval(&self, args: Vec<Complex64>, global_vars: &HashMap<&Var, Complex64>) -> EvalResult;
    fn is_variant_on_global(&self, global_vars: &Var) -> bool;
}