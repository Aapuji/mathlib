use std::{sync::Arc, collections::HashMap, fmt};
use num_complex::Complex64;

use crate::{num::Num, var::Var};
use super::{Expr, EvalResult};

#[derive(Debug)]
pub struct ConstExpr {
    num: Num
}

impl ConstExpr {
    pub fn new(num: Num) -> Self {
        Self { num }
    }
}

impl Expr for ConstExpr {
    fn is_variant_on(&self, _var: Arc<Var>) -> bool {
        false
    }
    fn eval(&self, _var_values: &HashMap<&Var, Complex64>) -> EvalResult {
        Ok(self.num.eval_float())
    }
}

impl fmt::Display for ConstExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CONST[{}]", self.num)
    }
}
