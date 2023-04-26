use std::{sync::Arc, collections::HashMap, ops::{Add, Mul}};

use num_complex::Complex64;

use crate::{var::Var, num::Num, expr::{EvalResult, EvalError}};

pub enum EExpr {
    Sum(Vec<EExpr>),
    Product(Vec<EExpr>),
    Var(Arc<Var>),
    Const(Num),
}

impl EExpr {
    fn eval(&self, var_values: &HashMap<&Var, Complex64>) -> EvalResult {
        match self {
            Self::Sum(terms) => {
                let mut sum = Complex64::new(0.0, 0.0);
                for term in terms {
                    sum += term.eval(var_values)?
                }
                Ok(sum)
            }
            Self::Product(terms) => {
                let mut product = Complex64::new(1.0, 0.0);
                for term in terms {
                    product *= term.eval(var_values)?
                }
                    
                Ok(product)
            }
            Self::Var(var) => {
                var_values.get(var.as_ref())
                    .map(|v| Ok(v.clone()))
                    .unwrap_or(Err(EvalError::VarMissing { name: var.get_name() }))
            }
            Self::Const(num) => {
                Ok(num.eval_float())
            }
        }
    }
}

impl Add for EExpr {
    type Output = EExpr;
    fn add(self, rhs: Self) -> Self::Output {
        Self::Sum(vec![self, rhs])
    }
}

impl Mul for EExpr {
    type Output = EExpr;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::Product(vec![self, rhs])
    }
}